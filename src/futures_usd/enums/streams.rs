use crate::futures_usd::enums::binance::{
    BookDepthUpdateSpeed, KlineContractType, KlineInterval, MarkPriceUpdateSpeed,
    PartialBookDepthLevel,
};
use crate::futures_usd::enums::streams::Streams::*;

/// Holds all the streams used in the Futures USD environment of Binance

const STREAM_BOOK_TICKER: &str = "@bookTicker";
const STREAM_BOOK_TICKERS: &str = "!bookTicker";
const STREAM_AGG_TRADE: &str = "@aggTrade";
const STREAM_MARK_PRICE: &str = "@markPrice";
const STREAM_MARK_PRICE_ARR: &str = "!markPrice@arr";
const STREAM_KLINE: &str = "@kline_";
const STREAM_CONTINUOUS_KLINE: &str = "@continuousKline_";
const STREAM_MINI_TICKER: &str = "@miniTicker";
const STREAM_MINI_TICKERS: &str = "!miniTicker@arr";
const STREAM_TICKER: &str = "@ticker";
const STREAM_TICKERS: &str = "!ticker@arr";
const STREAM_FORCE_ORDER: &str = "@forceOrder";
const STREAM_FORCE_ORDERS: &str = "!forceOrder@arr";
const STREAM_PARTIAL_BOOK_DEPTH: &str = "@depth";
const STREAM_BOOK_DEPTH: &str = "@depth";
const STREAM_COMPOSITE_INDEX: &str = "@compositeIndex";
const STREAM_CONTRACT_INFO: &str = "!contractInfo";
const STREAM_ASSET_INDEX_UPDATE: &str = "@assetIndex";
const STREAM_ASSET_INDEX_UPDATES: &str = "!assetIndex@arr";

#[derive(Debug)]
pub enum Streams {
    BookTicker(String),
    BookTickers(String),
    AggTrade(String),
    MarkPriceUpdate(String),
    MarkPriceUpdates(String),
    Kline(String),
    ContinuousKline(String),
    MiniTicker(String),
    MiniTickers(String),
    Ticker(String),
    Tickers(String),
    ForceOrder(String),
    ForceOrders(String),
    PartialBookDepth(String),
    BookDepth(String),
    CompositeIndex(String),
    ContractInfo(String),
    AssetIndexUpdate(String),
    AssetIndexUpdates(String),
}

impl Streams {
    pub fn book_ticker(symbol: &str) -> Self {
        BookTicker(symbol.to_lowercase() + STREAM_BOOK_TICKER)
    }

    pub fn book_tickers() -> Self {
        BookTickers(STREAM_BOOK_TICKERS.to_string())
    }

    pub fn agg_trade(symbol: &str) -> Self {
        AggTrade(symbol.to_lowercase() + STREAM_AGG_TRADE)
    }

    pub fn mark_price_update(symbol: &str, update_speed: MarkPriceUpdateSpeed) -> Self {
        if update_speed == MarkPriceUpdateSpeed::Seconds3 {
            MarkPriceUpdate(format!("{}{}", symbol.to_lowercase(), STREAM_MARK_PRICE))
        } else {
            MarkPriceUpdate(format!(
                "{}{}@{}",
                symbol.to_lowercase(),
                STREAM_MARK_PRICE,
                update_speed.to_str()
            ))
        }
    }

    pub fn mark_price_updates(update_speed: MarkPriceUpdateSpeed) -> Self {
        if update_speed == MarkPriceUpdateSpeed::Seconds3 {
            MarkPriceUpdates(STREAM_MARK_PRICE_ARR.to_string())
        } else {
            MarkPriceUpdates(format!(
                "{}@{}",
                STREAM_MARK_PRICE_ARR,
                update_speed.to_str()
            ))
        }
    }

    pub fn kline(symbol: &str, kline_interval: KlineInterval) -> Self {
        Kline(format!(
            "{}{}{}",
            symbol.to_lowercase(),
            STREAM_KLINE,
            kline_interval.to_str()
        ))
    }

    pub fn continuous_kline(
        symbol: &str,
        kline_contract_type: KlineContractType,
        kline_interval: KlineInterval,
    ) -> Self {
        ContinuousKline(format!(
            "{}_{}{}{}",
            symbol.to_lowercase(),
            kline_contract_type.to_str(),
            STREAM_CONTINUOUS_KLINE,
            kline_interval.to_str()
        ))
    }

    pub fn mini_ticker(symbol: &str) -> Self {
        MiniTicker(format!("{}{}", symbol.to_lowercase(), STREAM_MINI_TICKER))
    }

    pub fn mini_tickers() -> Self {
        MiniTickers(STREAM_MINI_TICKERS.to_string())
    }

    pub fn ticker(symbol: &str) -> Self {
        Ticker(format!("{}{}", symbol.to_lowercase(), STREAM_TICKER))
    }

    pub fn tickers() -> Self {
        Tickers(STREAM_TICKERS.to_string())
    }

    pub fn force_order(symbol: &str) -> Self {
        ForceOrder(format!("{}{}", symbol.to_lowercase(), STREAM_FORCE_ORDER))
    }

    pub fn force_orders() -> Self {
        ForceOrders(STREAM_FORCE_ORDERS.to_string())
    }

    pub fn partial_book_depth(
        symbol: &str,
        book_depth_level: PartialBookDepthLevel,
        book_depth_update_speed: BookDepthUpdateSpeed,
    ) -> Self {
        let mut update_speed: String = String::new();
        if book_depth_update_speed != BookDepthUpdateSpeed::Millis250 {
            update_speed = format!("@{}", book_depth_update_speed.to_str());
        }
        PartialBookDepth(format!(
            "{}{}{}{}",
            symbol.to_lowercase(),
            STREAM_PARTIAL_BOOK_DEPTH,
            book_depth_level.to_str(),
            update_speed
        ))
    }

    pub fn book_depth(symbol: &str, book_depth_update_speed: BookDepthUpdateSpeed) -> Self {
        let mut update_speed: String = String::new();
        if book_depth_update_speed != BookDepthUpdateSpeed::Millis250 {
            update_speed = format!("@{}", book_depth_update_speed.to_str());
        }
        BookDepth(format!(
            "{}{}{}",
            symbol.to_lowercase(),
            STREAM_BOOK_DEPTH,
            update_speed
        ))
    }

    pub fn composite_index(symbol: &str) -> Self {
        CompositeIndex(format!(
            "{}{}",
            symbol.to_lowercase(),
            STREAM_COMPOSITE_INDEX
        ))
    }

    pub fn contract_info() -> Self {
        ContractInfo(STREAM_CONTRACT_INFO.to_string())
    }

    pub fn asset_index_update(symbol: &str) -> Self {
        AssetIndexUpdate(format!(
            "{}{}",
            symbol.to_lowercase(),
            STREAM_ASSET_INDEX_UPDATE
        ))
    }

    pub fn asset_index_updates() -> Self {
        AssetIndexUpdates(STREAM_ASSET_INDEX_UPDATES.to_string())
    }

    pub fn to_str(&self) -> &str {
        match self {
            BookTicker(stream) => stream.as_str(),
            BookTickers(stream) => stream.as_str(),
            AggTrade(stream) => stream.as_str(),
            MarkPriceUpdate(stream) => stream.as_str(),
            MarkPriceUpdates(stream) => stream.as_str(),
            Kline(stream) => stream.as_str(),
            ContinuousKline(stream) => stream.as_str(),
            MiniTicker(stream) => stream.as_str(),
            MiniTickers(stream) => stream.as_str(),
            Ticker(stream) => stream.as_str(),
            Tickers(stream) => stream.as_str(),
            ForceOrder(stream) => stream.as_str(),
            ForceOrders(stream) => stream.as_str(),
            PartialBookDepth(stream) => stream.as_str(),
            BookDepth(stream) => stream.as_str(),
            CompositeIndex(stream) => stream.as_str(),
            ContractInfo(stream) => stream.as_str(),
            AssetIndexUpdate(stream) => stream.as_str(),
            AssetIndexUpdates(stream) => stream.as_str(),
        }
    }
}
