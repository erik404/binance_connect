use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use async_std::task;
use async_std::task::sleep;
use log::{error, info};
use url::Url;

use crate::constants;
use crate::error::BinanceConnectError;
use crate::futures_usd::client::client;
use crate::futures_usd::enums::binance::{
    BookDepthUpdateSpeed, KlineContractType, KlineInterval, MarkPriceUpdateSpeed,
    PartialBookDepthLevel,
};
use crate::futures_usd::enums::events::Event;
use crate::futures_usd::enums::streams::*;
use crate::futures_usd::listen_key::*;

/// Represents a configuration struct for handling "would block" situations in the WebSocket.
#[derive(Debug, Clone)]
pub struct WouldBlockConfig {
    /// The duration to wait for non-blocking WebSocket operations to complete before timing out.
    pub time_out: Duration,

    /// A flag indicating whether an error should be raised when a blocking operation occurs.
    pub error_on_block: bool,
}

impl Default for WouldBlockConfig {
    /// Creates a new `WouldBlockConfig` instance with default values.
    ///
    /// The default configuration sets a short timeout of 10 milliseconds and does not raise errors on block.
    fn default() -> Self {
        Self {
            // Sets the timeout to 10 milliseconds by default.
            time_out: Duration::from_millis(10),

            // Does not raise errors on block by default.
            error_on_block: false,
        }
    }
}

/// Represents a configuration struct for the Binance Futures WebSocket client.
#[derive(Debug, Clone)]
pub struct FuturesWebSocketConfig {
    /// Optional API authentication credentials.
    api_auth: Option<ApiAuth>,
    /// The main WebSocket URL for the Binance Futures market.
    url: Url,
    /// The WebSocket URL for the Binance Futures testnet.
    url_testnet: Url,
    /// A flag indicating whether to use the Binance Futures testnet.
    testnet: bool,
    /// Configuration for handling "would block" situations in the WebSocket.
    would_block_config: WouldBlockConfig,
    /// A flag indicating whether the WebSocket client should attempt to reconnect on errors.
    reconnect: bool,
}

impl Default for FuturesWebSocketConfig {
    /// Creates a new `FuturesWebSocketConfig` instance with default values.
    ///
    /// The default configuration sets the following:
    /// - No API authentication (`api_auth` is `None`).
    /// - Main WebSocket URL to the Binance Futures market.
    /// - Testnet WebSocket URL to the Binance Futures testnet.
    /// - Testnet flag is set to `false`.
    /// - Default `WouldBlockConfig`.
    /// - Reconnect flag is set to `true`.
    fn default() -> Self {
        Self {
            api_auth: None,
            url: Url::parse(constants::WS_URL_FUTURES).unwrap(),
            url_testnet: Url::parse(constants::WS_URL_FUTURES_TESTNET).unwrap(),
            testnet: false,
            would_block_config: WouldBlockConfig::default(),
            reconnect: true,
        }
    }
}

impl FuturesWebSocketConfig {
    /// Sets the `would_block_config` for the WebSocket configuration.
    pub fn with_would_block_config(mut self, would_block_config: WouldBlockConfig) -> Self {
        self.would_block_config = would_block_config;
        self
    }

    /// Sets the main WebSocket URL for the WebSocket configuration.
    pub fn with_url(mut self, url: &str) -> Result<Self, url::ParseError> {
        self.url = Url::parse(url)?;
        Ok(self)
    }

    /// Sets the testnet WebSocket URL for the WebSocket configuration.
    pub fn with_url_testnet(mut self, url: &str) -> Result<Self, url::ParseError> {
        self.url_testnet = Url::parse(url)?;
        Ok(self)
    }

    /// Sets the API authentication credentials for the WebSocket configuration.
    pub fn with_api_auth(mut self, api_auth: ApiAuth) -> Self {
        self.api_auth = Some(api_auth);
        self
    }

    /// Configures the WebSocket client to use the Binance Futures testnet.
    pub fn use_testnet(mut self) -> Self {
        self.testnet = true;
        self
    }

    /// Disables automatic reconnection on WebSocket errors.
    pub fn do_not_reconnect(mut self) -> Self {
        self.reconnect = false;
        self
    }

    /// Retrieves the appropriate WebSocket URL based on the testnet flag.
    fn get_url(&self) -> Url {
        if self.testnet {
            self.url_testnet.clone()
        } else {
            self.url.clone()
        }
    }
}

#[derive(Debug)]
pub struct FuturesUsdStream {
    config: FuturesWebSocketConfig,
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    listen_key: ListenKey,
    streams_public: Vec<Streams>,
    authenticated: bool,
}

impl Default for FuturesUsdStream {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver,
            config: FuturesWebSocketConfig::default(),
            listen_key: ListenKey { key: String::new() },
            streams_public: Vec::new(),
            authenticated: false,
        }
    }
}

impl FuturesUsdStream {
    /// Creates a new instance of the WebSocket client with the specified configuration.
    ///
    /// # Arguments
    ///
    /// - `config`: The WebSocket configuration to use for the client.
    ///
    /// # Returns
    ///
    /// A new instance of the WebSocket client configured with the given settings.
    ///
    pub fn with_config(config: FuturesWebSocketConfig) -> Self {
        let (sender, receiver) = channel();
        Self {
            config,
            sender,
            receiver,
            listen_key: ListenKey { key: String::new() },
            streams_public: Vec::new(),
            authenticated: false,
        }
    }

    /// Starts the WebSocket connection and background thread for event handling.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the WebSocket connection started.
    ///
    pub fn start(mut self) -> Self {
        self.listen_key();
        let url: Url = self.url();
        Self::ws_conn_thread(
            url,
            self.sender.clone(),
            self.config.clone(),
            self.subscribe_payload(),
        );
        self
    }

    /// Spawns a new thread for establishing a WebSocket connection.
    ///
    /// This function spawns a new thread to handle the WebSocket connection using the provided URL,
    /// WebSocket configuration, and optional subscription payload.
    ///
    /// # Arguments
    ///
    /// - `url`: The WebSocket URL to connect to.
    /// - `sender`: A `Sender<Event>` for sending WebSocket events to the calling code.
    /// - `config`: The WebSocket configuration.
    /// - `subscribe_payload`: An optional subscription payload to send upon connection.
    ///
    fn ws_conn_thread(
        url: Url,
        sender: Sender<Event>,
        config: FuturesWebSocketConfig,
        subscribe_payload: Option<String>,
    ) {
        thread::spawn(move || {
            Self::open_ws_con(url, sender, config, subscribe_payload);
        });
    }

    /// Opens a WebSocket connection and handles reconnection in case of errors.
    ///
    /// This function establishes a WebSocket connection using the provided URL and WebSocket configuration.
    /// It also handles automatic reconnection in the case of connection errors, if the `reconnect` option
    /// is enabled in the configuration.
    ///
    /// # Arguments
    ///
    /// - `url`: The WebSocket URL to connect to.
    /// - `sender`: A `Sender<Event>` for sending WebSocket events to the calling code.
    /// - `config`: The WebSocket configuration, including options for reconnecting.
    /// - `subscribe_payload`: An optional subscription payload to send upon connection.
    ///
    fn open_ws_con(
        url: Url,
        sender: Sender<Event>,
        config: FuturesWebSocketConfig,
        subscribe_payload: Option<String>,
    ) {
        let result: Result<(), BinanceConnectError> = client(
            sender.clone(),
            url.clone(),
            config.would_block_config.clone(),
            subscribe_payload.clone(),
        );
        if let Err(err) = result {
            if config.reconnect && matches!(err, BinanceConnectError::SocketError(_)) {
                info!("Reconnecting on SocketError: {:?}", err.to_string());
                thread::sleep(Duration::from_millis(100));
                Self::open_ws_con(url, sender, config, subscribe_payload);
            } else {
                panic!("futures_usd thread panicked {:?}", err.to_string());
            }
        }
    }

    /// Consumes the current instance and returns the event receiver.
    ///
    /// This function transfers ownership of the current instance to the caller and provides
    /// access to the event receiver, allowing the caller to receive WebSocket events.
    ///
    /// # Returns
    ///
    /// A `Receiver<Event>` that can be used to receive WebSocket events.
    ///
    pub fn consume(self) -> Receiver<Event> {
        self.receiver
    }

    /// Retrieves and manages the listen key used for WebSocket authentication.
    ///
    /// This function is responsible for obtaining the listen key and setting up automatic
    /// refreshes at a fixed interval to maintain WebSocket authentication.
    ///
    fn listen_key(&mut self) {
        let api_auth: &Option<ApiAuth> = &self.config.api_auth;
        if let Some(api_auth) = api_auth {
            self.authenticated = true;
            let listen_key = get_listen_key(api_auth, self.config.testnet)
                .unwrap_or_else(|err| panic!("{:?}", err));
            self.listen_key = listen_key;
            info!("{:?}", self.listen_key);
            task::spawn(Self::refresh_listen_key(
                api_auth.clone(),
                self.config.testnet,
            ));
        }
    }

    /// Asynchronously refreshes the listen key used for WebSocket authentication.
    ///
    /// This function continually refreshes the listen key at a fixed interval to ensure the WebSocket
    /// connection remains authenticated.
    ///
    /// # Arguments
    ///
    /// - `api_auth`: An `ApiAuth` struct containing API authentication information.
    /// - `test_net`: A boolean indicating whether the testnet environment should be used.
    ///
    async fn refresh_listen_key(api_auth: ApiAuth, test_net: bool) {
        loop {
            sleep(Duration::from_secs(3000)).await;
            get_listen_key(&api_auth, test_net)
                .map(|listen_key| {
                    info!("listen_key refreshed {}", listen_key.key);
                })
                .unwrap_or_else(|err| {
                    error!("could not refresh listen_key {:?}", err);
                });
        }
    }

    /// Generates the WebSocket URL for establishing a connection to the Binance WebSocket API.
    ///
    /// This function constructs the WebSocket URL based on the current configuration and the selected streams.
    /// If the connection is authenticated, it uses the listen key as part of the URL. If not authenticated,
    /// it requires at least one public stream to be selected.
    ///
    /// # Returns
    ///
    /// A `Url` instance representing the WebSocket URL.
    ///
    fn url(&mut self) -> Url {
        match self.authenticated {
            true => Url::parse(&format!(
                "{}ws/{}",
                self.config.get_url(),
                self.listen_key.key,
            ))
            .unwrap(),
            false => {
                let stream: Option<Streams> = self.streams_public.pop();
                match stream {
                    Some(stream) => Url::parse(
                        format!("{}ws/{}", self.config.get_url(), stream.to_str()).as_str(),
                    )
                    .unwrap(),
                    None => panic!(
                        "Can't start unauthenticated ws connection without at least 1 futures_usd"
                    ),
                }
            }
        }
    }

    /// Generates a subscription payload for the current instance.
    ///
    /// # Returns
    ///
    /// - `Some(String)`: A JSON payload for subscribing to the streams.
    /// - `None`: If there are no streams to subscribe to.
    fn subscribe_payload(&self) -> Option<String> {
        if self.streams_public.is_empty() {
            return None;
        }
        Some(format!(
            "{{\"method\": \"SUBSCRIBE\",\"params\":[{}],\"id\": 1}}",
            self.streams_public
                .iter()
                .map(|item| format!("\"{}\"", item.to_str()))
                .collect::<Vec<String>>()
                .join(",")
        ))
    }

    /// Adds a book ticker stream to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the book ticker stream should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the book ticker stream added.
    ///
    pub fn with_book_ticker(mut self, symbol: &str) -> Self {
        self.streams_public.push(Streams::book_ticker(symbol));
        self
    }

    /// Adds a book tickers stream to the current instance.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the book tickers stream added.
    ///
    pub fn with_book_tickers(mut self) -> Self {
        self.streams_public.push(Streams::book_tickers());
        self
    }

    /// Adds an aggregated trade stream for a specific symbol to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the aggregated trade stream
    ///   should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the aggregated trade stream added.
    ///
    pub fn with_agg_trade(mut self, symbol: &str) -> Self {
        self.streams_public.push(Streams::agg_trade(symbol));
        self
    }

    /// Adds a mark price update stream for a specific symbol with a specified update speed to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the mark price update stream should be added.
    /// - `update_speed`: A `MarkPriceUpdateSpeed` enum value specifying the update speed for the mark price data.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the mark price update stream added.
    ///
    pub fn with_mark_price_update(
        mut self,
        symbol: &str,
        update_speed: MarkPriceUpdateSpeed,
    ) -> Self {
        self.streams_public
            .push(Streams::mark_price_update(symbol, update_speed));
        self
    }

    /// Adds mark price update streams with a specified update speed to the current instance.
    ///
    /// # Arguments
    ///
    /// - `update_speed`: A `MarkPriceUpdateSpeed` enum value specifying the update speed for the mark price data.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the mark price update streams added.
    ///
    pub fn with_mark_price_updates(mut self, update_speed: MarkPriceUpdateSpeed) -> Self {
        self.streams_public
            .push(Streams::mark_price_updates(update_speed));
        self
    }

    /// Adds a Kline/candlestick chart stream for a specific symbol with a specified interval to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the Kline stream should be added.
    /// - `kline_interval`: A `KlineInterval` enum value specifying the interval for Kline data.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the Kline stream added.
    ///
    pub fn with_kline(mut self, symbol: &str, kline_interval: KlineInterval) -> Self {
        self.streams_public
            .push(Streams::kline(symbol, kline_interval));
        self
    }

    /// Adds a continuous Kline/candlestick chart stream for a specific symbol, contract type, and interval to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the continuous Kline stream should be added.
    /// - `kline_contract_type`: A `KlineContractType` enum value specifying the contract type for Kline data.
    /// - `kline_interval`: A `KlineInterval` enum value specifying the interval for Kline data.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the continuous Kline stream added.
    ///
    pub fn with_continuous_kline(
        mut self,
        symbol: &str,
        kline_contract_type: KlineContractType,
        kline_interval: KlineInterval,
    ) -> Self {
        self.streams_public.push(Streams::continuous_kline(
            symbol,
            kline_contract_type,
            kline_interval,
        ));
        self
    }

    /// Adds a mini-ticker stream for a specific trading symbol to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the mini-ticker stream should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the mini-ticker stream added.
    ///
    pub fn with_mini_ticker(mut self, symbol: &str) -> Self {
        self.streams_public.push(Streams::mini_ticker(symbol));
        self
    }

    /// Adds mini-ticker streams for all trading symbols to the current instance.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the mini-ticker streams added.
    ///
    pub fn with_mini_tickers(mut self) -> Self {
        self.streams_public.push(Streams::mini_tickers());
        self
    }

    /// Adds a ticker stream for a specific trading symbol to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the ticker stream should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the ticker stream added.
    ///
    pub fn with_ticker(mut self, symbol: &str) -> Self {
        self.streams_public.push(Streams::ticker(symbol));
        self
    }

    /// Adds ticker streams for all trading symbols to the current instance.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the ticker streams added.
    ///
    pub fn with_tickers(mut self) -> Self {
        self.streams_public.push(Streams::tickers());
        self
    }

    /// Adds a force order stream for a specific trading symbol to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the force order stream should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the force order stream added.
    ///
    pub fn with_force_order(mut self, symbol: &str) -> Self {
        self.streams_public.push(Streams::force_order(symbol));
        self
    }

    /// Adds force order streams for all trading symbols to the current instance.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the force order streams added.
    ///
    pub fn with_force_orders(mut self) -> Self {
        self.streams_public.push(Streams::force_orders());
        self
    }

    /// Adds a partial book depth stream for a specific trading symbol with specified depth level and update speed to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the partial book depth stream should be added.
    /// - `book_depth_level`: A `PartialBookDepthLevel` enum value specifying the depth level for the partial book depth data.
    /// - `book_depth_update_speed`: A `BookDepthUpdateSpeed` enum value specifying the update speed for the book depth data.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the partial book depth stream added.
    ///
    pub fn with_partial_book_depth(
        mut self,
        symbol: &str,
        book_depth_level: PartialBookDepthLevel,
        book_depth_update_speed: BookDepthUpdateSpeed,
    ) -> Self {
        self.streams_public.push(Streams::partial_book_depth(
            symbol,
            book_depth_level,
            book_depth_update_speed,
        ));
        self
    }

    /// Adds a full book depth stream for a specific trading symbol with a specified update speed to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the full book depth stream should be added.
    /// - `book_depth_update_speed`: A `BookDepthUpdateSpeed` enum value specifying the update speed for the book depth data.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the full book depth stream added.
    ///
    pub fn with_book_depth(
        mut self,
        symbol: &str,
        book_depth_update_speed: BookDepthUpdateSpeed,
    ) -> Self {
        self.streams_public
            .push(Streams::book_depth(symbol, book_depth_update_speed));
        self
    }

    /// Adds a composite index stream for a specific trading symbol to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the composite index stream should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the composite index stream added.
    ///
    pub fn with_composite_index(mut self, symbol: &str) -> Self {
        self.streams_public.push(Streams::composite_index(symbol));
        self
    }

    /// Adds a contract info stream to the current instance.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the contract info stream added.
    ///
    pub fn with_contract_info(mut self) -> Self {
        self.streams_public.push(Streams::contract_info());
        self
    }

    /// Adds an asset index update stream for a specific trading symbol to the current instance.
    ///
    /// # Arguments
    ///
    /// - `symbol`: A string representing the trading symbol for which the asset index update stream should be added.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the asset index update stream added.
    ///
    pub fn with_asset_index_update(mut self, symbol: &str) -> Self {
        self.streams_public
            .push(Streams::asset_index_update(symbol));
        self
    }

    /// Adds asset index update streams for all trading symbols to the current instance.
    ///
    /// # Returns
    ///
    /// A modified instance of the struct with the asset index update streams added.
    ///
    pub fn with_asset_index_updates(mut self) -> Self {
        self.streams_public.push(Streams::asset_index_updates());
        self
    }
}
