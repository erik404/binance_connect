use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::constants;
use crate::error::BinanceConnectError;

/// Represents API authentication credentials.
#[derive(Debug, Clone)]
pub struct ApiAuth {
    pub api_key: String,
    pub api_secret: String,
}

impl ApiAuth {
    /// Creates a new instance of ApiAuth.
    pub fn new(api_key: String, api_secret: String) -> ApiAuth {
        Self {
            api_key,
            api_secret,
        }
    }
}

/// Represents the response for creating a listen key.
#[derive(Deserialize, Debug, Clone)]
pub struct ListenKey {
    #[serde(rename = "listenKey")]
    pub key: String,
}

/// Retrieves a new listen key from Binance.
pub fn get_listen_key(
    api_auth: &ApiAuth,
    test_net: bool,
) -> Result<ListenKey, BinanceConnectError> {
    // Create a new HTTP client.
    let client: Client = Client::new();
    // Determine the appropriate Binance base URL based on the test_net flag.
    let endpoint: String = format!("{}{}", base_url(test_net), constants::FUTURES_LISTEN_KEY);
    // Send a POST request to obtain a listen key.
    let response: Response = client
        .post(endpoint)
        .header("X-MBX-APIKEY", &api_auth.api_key)
        .send()?;

    // Check if the response status is OK (200).
    if response.status() == StatusCode::OK {
        // Deserialize the response JSON into a ListenKey struct.
        serde_json::from_str(&response.text().unwrap()).map_err(BinanceConnectError::JsonError)
    } else {
        // Handle non-OK HTTP status codes by returning an error.
        Err(BinanceConnectError::HttpResponseError(format!(
            "Not-OK status code received {:?}",
            response.status()
        )))
    }
}

/// Returns the appropriate Binance base URL based on the test_net flag.
fn base_url(test_net: bool) -> &'static str {
    if test_net {
        constants::BASE_URL_FUTURES_TESTNET
    } else {
        constants::BASE_URL_FUTURES
    }
}
