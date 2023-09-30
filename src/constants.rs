/* --- FUTURES --- */
/** BASE_URI **/
pub const BASE_URL_FUTURES: &str = "https://fapi.binance.com";
pub const WS_URL_FUTURES: &str = "wss://fstream.binance.com";
pub const BASE_URL_FUTURES_TESTNET: &str = "https://testnet.binancefuture.com";
pub const WS_URL_FUTURES_TESTNET: &str = "wss://stream.binancefuture.com";
/** ENDPOINTS **/
pub const FUTURES_LISTEN_KEY: &str = "/fapi/v1/listenKey";

/* --- ERROR MESSAGES --- */
pub const ERR_ON_NO_ENABLED_STREAM_UNAUTHENTICATED_REQ: &str =
    "Can't start unauthenticated WS connection without at least 1 enabled stream";
