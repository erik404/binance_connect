use serde::{Deserialize, Deserializer, Serialize};

use crate::futures_usd::enums::binance::{
    AccountUpdateReason, ContractStatus, ContractType, ExecutionType, KlineContractType,
    KlineInterval, MarginType, OrderStatus, OrderType, PositionSide, PriceMatch, Side, StpMode,
    StrategyStatus, TimeInForce, WorkingType,
};
use crate::futures_usd::enums::events::EventType;

/// Holds all the possible responses from Binance

/* FUNCTIONALITY */

/// Deserialize a floating-point number represented as a string.
///
/// This function is used as a custom deserializer for parsing a floating-point number from a string
/// when deserializing JSON data. It takes a deserializer input and attempts to parse the input string
/// as an `f64`. If successful, it returns the parsed `f64`. If parsing fails, it returns a custom
/// deserialization error indicating that the parsing of the `f64` failed.
///
/// # Arguments
///
/// - `deserializer`: A deserializer implementing the `Deserializer` trait for deserializing JSON data.
///
/// # Returns
///
/// - `Result<f64, D::Error>`: A result containing the parsed `f64` or a deserialization error.
///
fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.parse::<f64>() {
        Ok(f) => Ok(f),
        Err(_) => Err(serde::de::Error::custom("Failed to parse f64")),
    }
}

/* GENERIC */

#[derive(Debug, Deserialize, Serialize)]
pub struct EventTypeWrapper {
    #[serde(rename = "e")]
    pub event_type: EventType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeResponse {
    pub result: Option<serde_json::Value>,
    pub id: Option<u64>,
}

/* MARKET */

#[derive(Debug)]
pub struct BookTickers {
    pub data: Vec<BookTicker>,
}

impl BookTickers {
    pub fn new(book_tickers: Vec<BookTicker>) -> BookTickers {
        Self { data: book_tickers }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookTicker {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(rename = "b", deserialize_with = "deserialize_f64")]
    pub bid_price: f64,
    #[serde(rename = "B", deserialize_with = "deserialize_f64")]
    pub bid_quantity: f64,
    #[serde(rename = "a", deserialize_with = "deserialize_f64")]
    pub ask_price: f64,
    #[serde(rename = "A", deserialize_with = "deserialize_f64")]
    pub ask_quantity: f64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AggTrade {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    pub price: f64,
    #[serde(rename = "q", deserialize_with = "deserialize_f64")]
    pub quantity: f64,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub buyer_is_market_maker: bool,
}

#[derive(Debug)]
pub struct MarkPriceUpdates {
    pub data: Vec<MarkPriceUpdate>,
}

impl MarkPriceUpdates {
    pub fn new(mark_price_updates: Vec<MarkPriceUpdate>) -> MarkPriceUpdates {
        Self {
            data: mark_price_updates,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MarkPriceUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    pub mark_price: f64,
    #[serde(rename = "i", deserialize_with = "deserialize_f64")]
    pub index_price: f64,
    #[serde(rename = "P", deserialize_with = "deserialize_f64")]
    pub estimated_settle_price: f64,
    #[serde(rename = "r", deserialize_with = "deserialize_f64")]
    pub funding_rate: f64,
    #[serde(rename = "T")]
    pub next_funding_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct Kline {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline_data: KlineData,
}

#[derive(Debug, Deserialize)]
pub struct ContinuousKline {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "ps")]
    pub pair: String,
    #[serde(rename = "ct")]
    pub contract_type: KlineContractType,
    #[serde(rename = "k")]
    pub kline_data: KlineData,
}

#[derive(Debug, Deserialize)]
pub struct KlineData {
    #[serde(rename = "t")]
    pub kline_start_time: i64,
    #[serde(rename = "T")]
    pub kline_close_time: i64,
    #[serde(rename = "s", default)]
    pub kline_symbol: String,
    #[serde(rename = "i")]
    pub interval: KlineInterval,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "o", deserialize_with = "deserialize_f64")]
    pub open_price: f64,
    #[serde(rename = "c", deserialize_with = "deserialize_f64")]
    pub close_price: f64,
    #[serde(rename = "h", deserialize_with = "deserialize_f64")]
    pub high_price: f64,
    #[serde(rename = "l", deserialize_with = "deserialize_f64")]
    pub low_price: f64,
    #[serde(rename = "v", deserialize_with = "deserialize_f64")]
    pub base_asset_volume: f64,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
    #[serde(rename = "x")]
    pub is_kline_closed: bool,
    #[serde(rename = "q", deserialize_with = "deserialize_f64")]
    pub quote_asset_volume: f64,
    #[serde(rename = "V", deserialize_with = "deserialize_f64")]
    pub taker_buy_base_asset_volume: f64,
    #[serde(rename = "Q", deserialize_with = "deserialize_f64")]
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug)]
pub struct MiniTickers {
    pub data: Vec<MiniTicker>,
}

impl MiniTickers {
    pub fn new(mini_tickers: Vec<MiniTicker>) -> MiniTickers {
        Self { data: mini_tickers }
    }
}

#[derive(Deserialize, Debug)]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", deserialize_with = "deserialize_f64")]
    pub close_price: f64,
    #[serde(rename = "o", deserialize_with = "deserialize_f64")]
    pub open_price: f64,
    #[serde(rename = "h", deserialize_with = "deserialize_f64")]
    pub high_price: f64,
    #[serde(rename = "l", deserialize_with = "deserialize_f64")]
    pub low_price: f64,
    #[serde(rename = "v", deserialize_with = "deserialize_f64")]
    pub total_traded_base_asset_volume: f64,
    #[serde(rename = "q", deserialize_with = "deserialize_f64")]
    pub total_traded_quote_asset_volume: f64,
}

#[derive(Debug)]
pub struct Tickers {
    pub data: Vec<Ticker>,
}

impl Tickers {
    pub fn new(tickers: Vec<Ticker>) -> Tickers {
        Self { data: tickers }
    }
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    pub price_change: f64,
    #[serde(rename = "P", deserialize_with = "deserialize_f64")]
    pub price_change_percent: f64,
    #[serde(rename = "w", deserialize_with = "deserialize_f64")]
    pub weighted_avg_price: f64,
    #[serde(rename = "c", deserialize_with = "deserialize_f64")]
    pub last_price: f64,
    #[serde(rename = "Q", deserialize_with = "deserialize_f64")]
    pub last_quantity: f64,
    #[serde(rename = "o", deserialize_with = "deserialize_f64")]
    pub open_price: f64,
    #[serde(rename = "h", deserialize_with = "deserialize_f64")]
    pub high_price: f64,
    #[serde(rename = "l", deserialize_with = "deserialize_f64")]
    pub low_price: f64,
    #[serde(rename = "v", deserialize_with = "deserialize_f64")]
    pub total_traded_base_asset_volume: f64,
    #[serde(rename = "q", deserialize_with = "deserialize_f64")]
    pub total_traded_quote_asset_volume: f64,
    #[serde(rename = "O")]
    pub statistics_open_time: u64,
    #[serde(rename = "C")]
    pub statistics_close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub total_number_of_trades: u64,
}

#[derive(Debug, Deserialize)]
pub struct ForceOrder {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "o")]
    pub order: ForceOrderData,
}

#[derive(Debug, Deserialize)]
pub struct ForceOrderData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    pub original_quantity: String,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    pub price: f64,
    #[serde(rename = "ap", deserialize_with = "deserialize_f64")]
    pub average_price: f64,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "l", deserialize_with = "deserialize_f64")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "z", deserialize_with = "deserialize_f64")]
    pub order_filled_accumulated_quantity: f64,
    #[serde(rename = "T")]
    pub order_trade_time: i64,
}

#[derive(Debug, Deserialize)]
pub struct BookDepth {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: i64,
    #[serde(rename = "u")]
    pub final_update_id: i64,
    #[serde(rename = "pu")]
    pub previous_final_update_id: i64,
    #[serde(rename = "b")]
    pub bids: Vec<BidUpdate>,
    #[serde(rename = "a")]
    pub asks: Vec<AskUpdate>,
}

#[derive(Debug, Deserialize)]
pub struct BidUpdate {
    #[serde(rename = "0", deserialize_with = "deserialize_f64")]
    pub price_level: f64,
    #[serde(rename = "1", deserialize_with = "deserialize_f64")]
    pub quantity: f64,
}

#[derive(Debug, Deserialize)]
pub struct AskUpdate {
    #[serde(rename = "0", deserialize_with = "deserialize_f64")]
    pub price_level: f64,
    #[serde(rename = "1", deserialize_with = "deserialize_f64")]
    pub quantity: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompositeIndex {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    pub price: f64,
    #[serde(rename = "C")]
    pub composition_type: String,
    #[serde(rename = "c")]
    pub composition: Vec<Composition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Composition {
    #[serde(rename = "b")]
    pub base_asset: String,
    #[serde(rename = "q")]
    pub quote_asset: String,
    #[serde(rename = "w", deserialize_with = "deserialize_f64")]
    pub weight_quantity: f64,
    #[serde(rename = "W", deserialize_with = "deserialize_f64")]
    pub weight_percentage: f64,
    #[serde(rename = "i", deserialize_with = "deserialize_f64")]
    pub index_price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractInfo {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub pair: String,
    #[serde(rename = "ct")]
    pub contract_type: ContractType,
    #[serde(rename = "dt")]
    pub delivery_date_time: i64,
    #[serde(rename = "ot")]
    pub onboard_date_time: i64,
    #[serde(rename = "cs")]
    pub contract_status: ContractStatus,
    #[serde(rename = "bks")]
    pub brackets: Vec<ContractInfoBracket>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractInfoBracket {
    #[serde(rename = "bs")]
    pub notional_bracket: i32,
    #[serde(rename = "bnf")]
    pub floor_notional: i64,
    #[serde(rename = "bnc")]
    pub cap_notional: i64,
    #[serde(rename = "mmr", deserialize_with = "deserialize_f64")]
    pub maintenance_ratio: f64,
    #[serde(rename = "cf")]
    pub auxiliary_number: i64,
    #[serde(rename = "mi")]
    pub min_leverage: i32,
    #[serde(rename = "ma")]
    pub max_leverage: i32,
}

#[derive(Debug)]
pub struct AssetIndexUpdates {
    pub data: Vec<AssetIndexUpdate>,
}

impl AssetIndexUpdates {
    pub fn new(asset_index_updates: Vec<AssetIndexUpdate>) -> AssetIndexUpdates {
        Self {
            data: asset_index_updates,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetIndexUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "s")]
    pub asset_index_symbol: String,
    #[serde(rename = "i", deserialize_with = "deserialize_f64")]
    pub index_price: f64,
    #[serde(rename = "b", deserialize_with = "deserialize_f64")]
    pub bid_buffer: f64,
    #[serde(rename = "a", deserialize_with = "deserialize_f64")]
    pub ask_buffer: f64,
    #[serde(rename = "B", deserialize_with = "deserialize_f64")]
    pub bid_rate: f64,
    #[serde(rename = "A", deserialize_with = "deserialize_f64")]
    pub ask_rate: f64,
    #[serde(rename = "q", deserialize_with = "deserialize_f64")]
    pub auto_exchange_bid_buffer: f64,
    #[serde(rename = "g", deserialize_with = "deserialize_f64")]
    pub auto_exchange_ask_buffer: f64,
    #[serde(rename = "Q", deserialize_with = "deserialize_f64")]
    pub auto_exchange_bid_rate: f64,
    #[serde(rename = "G", deserialize_with = "deserialize_f64")]
    pub auto_exchange_ask_rate: f64,
}

/* USER DATA */

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderTradeUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "o")]
    pub order_data: OrderData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q", deserialize_with = "deserialize_f64")]
    pub original_quantity: f64,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    pub original_price: f64,
    #[serde(rename = "ap", deserialize_with = "deserialize_f64")]
    pub average_price: f64,
    #[serde(rename = "sp", deserialize_with = "deserialize_f64")]
    pub stop_price: f64,
    #[serde(rename = "x")]
    pub execution_type: ExecutionType,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: i64,
    #[serde(rename = "l", deserialize_with = "deserialize_f64")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "z", deserialize_with = "deserialize_f64")]
    pub order_filled_accumulated_quantity: f64,
    #[serde(rename = "L", deserialize_with = "deserialize_f64")]
    pub last_filled_price: f64,
    #[serde(rename = "N", default)]
    pub commission_asset: String,
    #[serde(rename = "n", default, deserialize_with = "deserialize_f64")]
    pub commission: f64,
    #[serde(rename = "T")]
    pub order_trade_time: i64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(rename = "b", deserialize_with = "deserialize_f64")]
    pub bids_notional: f64,
    #[serde(rename = "a", deserialize_with = "deserialize_f64")]
    pub ask_notional: f64,
    #[serde(rename = "m")]
    pub is_trade_maker_side: bool,
    #[serde(rename = "R")]
    pub is_reduce_only: bool,
    #[serde(rename = "wt")]
    pub stop_price_working_type: WorkingType,
    #[serde(rename = "ot")]
    pub original_order_type: OrderType,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "cp", default)]
    pub is_close_all: bool,
    #[serde(rename = "AP", default, deserialize_with = "deserialize_f64")]
    pub activation_price: f64,
    #[serde(rename = "cr", default, deserialize_with = "deserialize_f64")]
    pub callback_rate: f64,
    #[serde(rename = "pP")]
    pub is_price_protection_enabled: bool,
    #[serde(rename = "si")]
    pub ignore1: i64,
    #[serde(rename = "ss")]
    pub ignore2: i64,
    #[serde(rename = "rp", deserialize_with = "deserialize_f64")]
    pub realized_profit: f64,
    #[serde(rename = "V")]
    pub stp_mode: StpMode,
    #[serde(rename = "pm")]
    pub price_match_mode: PriceMatch,
    #[serde(rename = "gtd")]
    pub gtd_order_auto_cancel_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "a")]
    pub update_data: UpdateData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateData {
    #[serde(rename = "m")]
    pub event_reason_type: AccountUpdateReason,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", deserialize_with = "deserialize_f64")]
    pub wallet_balance: f64,
    #[serde(rename = "cw", deserialize_with = "deserialize_f64")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "bc", deserialize_with = "deserialize_f64")]
    pub balance_change: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", deserialize_with = "deserialize_f64")]
    pub position_amount: f64,
    #[serde(rename = "ep", deserialize_with = "deserialize_f64")]
    pub entry_price: f64,
    #[serde(rename = "bep", deserialize_with = "deserialize_f64")]
    pub breakeven_price: f64,
    #[serde(rename = "cr", deserialize_with = "deserialize_f64")]
    pub accumulated_realized: f64,
    #[serde(rename = "up", deserialize_with = "deserialize_f64")]
    pub unrealized_pnl: f64,
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    #[serde(rename = "iw", deserialize_with = "deserialize_f64")]
    pub isolated_wallet: f64,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarginCall {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "cw", deserialize_with = "deserialize_f64")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "p")]
    pub positions: Vec<MarginCallPosition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarginCallPosition {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "pa", deserialize_with = "deserialize_f64")]
    pub position_amount: f64,
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    #[serde(rename = "iw", deserialize_with = "deserialize_f64")]
    pub isolated_wallet: f64,
    #[serde(rename = "mp", deserialize_with = "deserialize_f64")]
    pub mark_price: f64,
    #[serde(rename = "up", deserialize_with = "deserialize_f64")]
    pub unrealized_pnl: f64,
    #[serde(rename = "mm", deserialize_with = "deserialize_f64")]
    pub maintenance_margin_required: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountConfigUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "ac")]
    pub account_config: Option<AccountConfig>,
    #[serde(rename = "ai")]
    pub account_info: Option<AccountInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountConfig {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "l")]
    pub leverage: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "j")]
    pub multi_assets_mode: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StrategyUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "su")]
    pub strategy: Strategy,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Strategy {
    #[serde(rename = "si")]
    pub strategy_id: i64,
    #[serde(rename = "st")]
    pub strategy_type: String,
    #[serde(rename = "ss")]
    pub strategy_status: StrategyStatus,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ut")]
    pub update_time: i64,
    #[serde(rename = "c")]
    pub op_code: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GridUpdate {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "T")]
    pub transaction_time: i64,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "gu")]
    pub grid: Grid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Grid {
    #[serde(rename = "si")]
    pub strategy_id: i64,
    #[serde(rename = "st")]
    pub strategy_type: String,
    #[serde(rename = "ss")]
    pub strategy_status: StrategyStatus,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "r", deserialize_with = "deserialize_f64")]
    pub realized_pnl: f64,
    #[serde(rename = "up", deserialize_with = "deserialize_f64")]
    pub unmatched_average_price: f64,
    #[serde(rename = "uq", deserialize_with = "deserialize_f64")]
    pub unmatched_qty: f64,
    #[serde(rename = "uf", deserialize_with = "deserialize_f64")]
    pub unmatched_fee: f64,
    #[serde(rename = "mp", deserialize_with = "deserialize_f64")]
    pub matched_pnl: f64,
    #[serde(rename = "ut")]
    pub update_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalOrderTriggerReject {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "T")]
    pub message_send_time: i64,
    #[serde(rename = "or")]
    pub order_reject: OrderReject,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderReject {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: i64,
    #[serde(rename = "r")]
    pub reject_reason: String,
}
