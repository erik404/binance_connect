use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::futures_usd::response::*;

/// Holds all the Events send within the library

#[derive(Debug)]
pub enum Event {
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
    SubscribeResponseEvent,
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum EventType {
    /* MARKET_DATA */
    #[serde(rename = "bookTicker")]
    BookTickerEventType,
    #[serde(rename = "aggTrade")]
    AggTradeEventType,
    #[serde(rename = "markPriceUpdate")]
    MarkPriceUpdateEventType,
    #[serde(rename = "kline")]
    KlineEventType,
    #[serde(rename = "continuous_kline")]
    ContinuousKlineEventType,
    #[serde(rename = "24hrMiniTicker")]
    MiniTickerEventType,
    #[serde(rename = "24hrTicker")]
    TickerEventType,
    #[serde(rename = "forceOrder")]
    ForceOrderEventType,
    #[serde(rename = "depthUpdate")]
    BookDepthEventType,
    #[serde(rename = "compositeIndex")]
    CompositeIndexEventType,
    #[serde(rename = "contractInfo")]
    ContractInfoEventType,
    #[serde(rename = "assetIndexUpdate")]
    AssetIndexUpdateEventType,
    /* USER_DATA */
    #[serde(rename = "ORDER_TRADE_UPDATE")]
    OrderTradeUpdateEventType,
    #[serde(rename = "ACCOUNT_UPDATE")]
    AccountUpdateEventType,
    #[serde(rename = "MARGIN_CALL")]
    MarginCallEventType,
    #[serde(rename = "ACCOUNT_CONFIG_UPDATE")]
    AccountConfigUpdateEventType,
    #[serde(rename = "STRATEGY_UPDATE")]
    StrategyUpdateEventType,
    #[serde(rename = "GRID_UPDATE")]
    GridUpdateEventType,
    #[serde(rename = "CONDITIONAL_ORDER_TRIGGER_REJECT")]
    ConditionalOrderTriggerRejectEventType,
}
