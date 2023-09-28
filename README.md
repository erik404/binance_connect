# binance_connect::futures_usd

## Features

- Connects to the Binance USD-M Futures WebSocket to receive real-time market and account updates via an event consumer.
- Events that are consumed contain a (sanitized) struct representation of the returned Binance data.
- ListenKey creation and keep-alive is managed by the library.
- WebSocket connection drops are caught and managed by the library. This because Binance forcefully drops connections after the 24h mark. This can be configured in the `FuturesWebSocketConfig` using the `reconnect(bool)` setter (default setting is true).


## Getting Started
###
#### Without config

Create an instance of the `FuturesUsdStream` and add the desired streams for real-time updates. In this example, we're subscribing to the book depth for the BTC/USDT trading pair and all book tickers.

 ```rust
 let fus: FuturesUsdStream = FuturesUsdStream::default()
     .with_book_depth("btcusdt", BookDepthUpdateSpeed::Millis500)
     .with_book_tickers()
     .start();
 ```

##
#### With config

Set up your Binance API authentication using the ApiAuth struct. This is only required when requesting [User Data Streams](https://binance-docs.github.io/apidocs/futures/en/#user-data-streams).

 ```rust
 let api_auth: ApiAuth = ApiAuth::new(
     "YOUR_API_KEY".to_string(),
     "YOUR_API_SECRET".to_string(),
 );
 ```

A `FuturesWebsocketConfig` can be created to pass configuration options to the `FuturesUsdStream`. For example; You can specify whether to use the Binance testnet or the live environment. When an authenticated connection is required the `FuturesWebsocketConfig` is mandatory.

 ```rust
 let config: FuturesWebSocketConfig = FuturesWebSocketConfig::with_config(config)
     .with_api_auth(api_auth)
     .use_testnet(); // Use the testnet environment (remove for live trading)
 ```


Create an instance of the `FuturesUsdStream` using the `with_config()` builder and add the desired streams for real-time updates. In this example, we're subscribing to the book depth for the BTC/USDT trading pair, all book tickers and because the `with_api_auth` method is called on the `FuturesWebsocketConfig` all [User Data Streams](https://binance-docs.github.io/apidocs/futures/en/#user-data-streams).

 ```rust
 let fus: FuturesUsdStream = FuturesUsdStream::with_config(config)
     .with_book_depth("btcusdt", BookDepthUpdateSpeed::Millis500)
     .with_book_tickers()
     .start();
 ```

## Consuming Events

Start consuming events from the `FuturesUsdStream` using the `consume()` method and handle them as needed.

 ```rust
 for event in fus.consume() {
     match event {
         BookDepthEvent(book_depth) => {
             println!("{:?}", book_depth)
         }
         BookTickersEvent(book_tickers) => {
             for book_ticker in book_tickers.data {
                 println!("{:?}", book_ticker)
             }
         }
        // Only available with `with_api_auth(api_auth)` called on `FuturesWebsocketConfig` 
         AccountUpdateEvent(account_update) => {
             println!("{:?}", account_update)
         }
         _ => {}
     }
 }
 ```
The structs that are returned when consuming Events follow a predictable property name convention as opposed to the single letter convention used by Binance. Property values can be an Enum. _See [ src/futures_usd/enums/binance.rs](src/futures_usd/enums/binance.rs)_

_BookTicker response struct as example reference. See [src/futures_usd/response.rs](src/futures_usd/response.rs)_

```rust
pub struct BookTicker {
    pub event_type: EventType,
    pub event_time: u64,
    pub symbol: String,
    pub update_id: u64,
    pub bid_price: f64,
    pub bid_quantity: f64,
    pub ask_price: f64,
    pub ask_quantity: f64,
    pub transaction_time: u64,
}
```


#### Events 

```rust
 /* MARKET_DATA */
 BookTickerEvent(BookTicker),
 BookTickersEvent(BookTickers),
 AggTradeEvent(AggTrade),
 MarkPriceUpdateEvent(MarkPriceUpdate),
 MarkPriceUpdatesEvent(MarkPriceUpdates),
 KlineEvent(Kline),
 ContinuousKlineEvent(ContinuousKline),
 MiniTickerEvent(MiniTicker),
 MiniTickersEvent(MiniTickers),
 TickerEvent(Ticker),
 TickersEvent(Tickers),
 ForceOrderEvent(ForceOrder),
 BookDepthEvent(BookDepth),
 CompositeIndexEvent(CompositeIndex),
 ContractInfoEvent(ContractInfo),
 AssetIndexUpdateEvent(AssetIndexUpdate),
 AssetIndexUpdatesEvent(AssetIndexUpdates),
 /* USER_DATA */
 OrderTradeUpdateEvent(OrderTradeUpdate),
 AccountUpdateEvent(AccountUpdate),
 MarginCallEvent(MarginCall),
 AccountConfigUpdateEvent(AccountConfigUpdate),
 StrategyUpdateEvent(StrategyUpdate),
 GridUpdateEvent(GridUpdate),
 ConditionalOrderTriggerRejectEvent(ConditionalOrderTriggerReject),
 /* SYSTEM */
 SubscribeResponseEvent,
```

## Errors

All errors are propagated to a BinanceConnectError. _See [src/error.rs](/src/error.rs)_
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.txt) file for details.
