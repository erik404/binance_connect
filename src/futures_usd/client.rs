use std::io::ErrorKind;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;

use log::{debug, info};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

use crate::error::BinanceConnectError;
use crate::futures_usd::deserializer::deserialize;
use crate::futures_usd::enums::events::Event;
use crate::futures_usd::stream::WouldBlockConfig;

/// Establishes a WebSocket connection to the provided URL, reads and processes messages,
/// and sends events to the specified sender.
///
/// # Arguments
///
/// * `sender` - A `Sender<Event>` to send events to.
/// * `url` - The URL to connect to.
/// * `would_block_config` - Configuration for handling WouldBlock errors.
/// * `subscribe_payload` - An optional JSON payload to subscribe to specific streams.
///
/// # Returns
///
/// This function returns `Ok(())` if the connection and processing were successful, or
/// a `BinanceConnectError` if an error occurred.
pub fn client(
    sender: Sender<Event>,
    url: Url,
    stop_signal: Arc<AtomicBool>,
    would_block_config: WouldBlockConfig,
    subscribe_payload: Option<String>,
) -> Result<(), BinanceConnectError> {
    // Establish a WebSocket connection.
    let mut socket: WebSocket<MaybeTlsStream<TcpStream>> = socket(url)?;

    // If a subscribe payload is provided, send the subscription request.
    if let Some(subscribe_payload) = subscribe_payload {
        debug!("{:?}", subscribe_payload);
        socket.send(Message::Text(subscribe_payload))?;
    }

    // Continuously read and process WebSocket messages.
    while !stop_signal.load(Ordering::Relaxed) {
        match socket.read() {
            Ok(message) => match message {
                // Handle incoming JSON messages.
                Message::Text(json_response) => {
                    // Stop signal might have been called
                    if stop_signal.load(Ordering::Relaxed) {
                        return Ok(());
                    };

                    // Deserialize the JSON into an `Event` and send it to the sender.
                    let event: Event = deserialize(json_response)?;
                    sender.send(event)?;
                }
                // Handle incoming Ping messages.
                Message::Ping(ping) => {
                    // Stop signal might have been called
                    if stop_signal.load(Ordering::Relaxed) {
                        return Ok(());
                    };
                    // Respond to Ping with Pong to keep the connection alive.
                    socket.send(Message::Pong(ping))?;
                    debug!("Pong");
                }
                _ => {}
            },
            Err(err) => match err {
                tungstenite::Error::Io(ref io_err) if io_err.kind() == ErrorKind::WouldBlock => {
                    // Stop signal might have been called
                    if stop_signal.load(Ordering::Relaxed) {
                        return Ok(());
                    };

                    if would_block_config.error_on_block {
                        // Return a SocketError if configured to do so.
                        Err(BinanceConnectError::SocketError(err))?;
                    }
                    // Sleep for the specified time if a WouldBlock error occurs.
                    info!(
                        "futures_usd client thread slept {:?} because of WouldBlock error",
                        would_block_config.time_out
                    );
                    std::thread::sleep(would_block_config.time_out);
                }
                _ => {
                    // Stop signal might have been called
                    if stop_signal.load(Ordering::Relaxed) {
                        return Ok(());
                    };
                    // Return a SocketError for other types of errors.
                    Err(BinanceConnectError::SocketError(err))?;
                }
            },
        }
    }

    Ok(())
}

/// Establishes a WebSocket connection to the provided URL.
fn socket(url: Url) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, BinanceConnectError> {
    let (socket, _) = connect(url)?;
    Ok(socket)
}
