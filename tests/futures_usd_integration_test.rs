// Event::BookTickerEvent(_) => {}
// Event::BookTickersEvent(_) => {}
// Event::AggTradeEvent(_) => {}
// Event::MarkPriceUpdateEvent(_) => {}
// Event::MarkPriceUpdatesEvent(_) => {}
// Event::KlineEvent(_) => {}
// Event::ContinuousKlineEvent(_) => {}
// Event::MiniTickerEvent(_) => {}
// Event::MiniTickersEvent(_) => {}
// Event::TickerEvent(_) => {}
// Event::TickersEvent(_) => {}
// Event::ForceOrderEvent(_) => {}
// Event::BookDepthEvent(_) => {}
// Event::CompositeIndexEvent(_) => {}
// Event::ContractInfoEvent(_) => {}
// Event::AssetIndexUpdateEvent(_) => {}
// Event::AssetIndexUpdatesEvent(_) => {}
// Event::OrderTradeUpdateEvent(_) => {}
// Event::AccountUpdateEvent(_) => {}
// Event::MarginCallEvent(_) => {}
// Event::AccountConfigUpdateEvent(_) => {}
// Event::StrategyUpdateEvent(_) => {}
// Event::GridUpdateEvent(_) => {}
// Event::ConditionalOrderTriggerRejectEvent(_) => {}
// Event::SubscribeResponseEvent => {}

#[cfg(test)]
mod tests {
    use binance_connect::futures_usd::enums::events::Event::{BookTickerEvent, BookTickersEvent};
    use binance_connect::futures_usd::enums::events::{Event, EventType};
    use binance_connect::futures_usd::response::{BookTicker, BookTickers};
    use binance_connect::futures_usd::stream::{FuturesUsdStream, FuturesWebSocketConfig};
    use log::info;
    use std::env;

    #[test]
    #[should_panic(
        expected = "Can't start unauthenticated WS connection without at least 1 enabled stream"
    )]
    fn test_panic_on_non_authenticated_connection_without_at_least_1_enabled_stream() {
        FuturesUsdStream::default().start();
    }

    #[test]
    fn test_stream_book_ticker() {
        set_log_level();
        let symbol: &str = "btcusdt";

        let config: FuturesWebSocketConfig = FuturesWebSocketConfig::default().use_testnet();
        let fus: FuturesUsdStream = FuturesUsdStream::with_config(config)
            .with_book_ticker(symbol)
            .start();

        let mut result: Option<BookTicker> = None;
        for event in fus.consume() {
            if let BookTickerEvent(book_ticker) = event {
                result = Some(book_ticker);
                break;
            }
        }

        match result {
            None => {
                panic!("No BookTicker received")
            }
            Some(book_ticker) => {
                assert_eq!(
                    book_ticker.event_type,
                    EventType::BookTickerEventType,
                    "book_ticker.event_type does not match EventType::BookTickerEventType"
                );
                assert_eq!(
                    book_ticker.symbol.to_lowercase(),
                    symbol,
                    "Symbol does not match the symbol the stream started with"
                );
            }
        }
    }

    fn set_log_level() {
        env::set_var("RUST_LOG", "info");
        tracing_subscriber::fmt::init();
    }
}
