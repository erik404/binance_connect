use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Holds all the enums used by and with Binance operations

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum StrategyStatus {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "WORKING")]
    Working,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "EXPIRED")]
    Expired,
}

impl StrategyStatus {
    pub fn to_str(&self) -> &str {
        match self {
            StrategyStatus::New => "NEW",
            StrategyStatus::Working => "WORKING",
            StrategyStatus::Cancelled => "CANCELLED",
            StrategyStatus::Expired => "EXPIRED",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum AccountUpdateReason {
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[serde(rename = "WITHDRAW")]
    Withdraw,
    #[serde(rename = "ORDER")]
    Order,
    #[serde(rename = "FUNDING_FEE")]
    FundingFee,
    #[serde(rename = "WITHDRAW_REJECT")]
    WithdrawReject,
    #[serde(rename = "ADJUSTMENT")]
    Adjustment,
    #[serde(rename = "INSURANCE_CLEAR")]
    InsuranceClear,
    #[serde(rename = "ADMIN_DEPOSIT")]
    AdminDeposit,
    #[serde(rename = "ADMIN_WITHDRAW")]
    AdminWithdraw,
    #[serde(rename = "MARGIN_TRANSFER")]
    MarginTransfer,
    #[serde(rename = "MARGIN_TYPE_CHANGE")]
    MarginTypeChange,
    #[serde(rename = "ASSET_TRANSFER")]
    AssetTransfer,
    #[serde(rename = "OPTIONS_PREMIUM_FEE")]
    OptionsPremiumFee,
    #[serde(rename = "OPTIONS_SETTLE_PROFIT")]
    OptionsSettleProfit,
    #[serde(rename = "AUTO_EXCHANGE")]
    AutoExchange,
    #[serde(rename = "COIN_SWAP_DEPOSIT")]
    CoinSwapDeposit,
    #[serde(rename = "COIN_SWAP_WITHDRAW")]
    CoinSwapWithdraw,
}

impl AccountUpdateReason {
    pub fn to_str(&self) -> &str {
        match self {
            AccountUpdateReason::Deposit => "DEPOSIT",
            AccountUpdateReason::Withdraw => "WITHDRAW",
            AccountUpdateReason::Order => "ORDER",
            AccountUpdateReason::FundingFee => "FUNDING_FEE",
            AccountUpdateReason::WithdrawReject => "WITHDRAW_REJECT",
            AccountUpdateReason::Adjustment => "ADJUSTMENT",
            AccountUpdateReason::InsuranceClear => "INSURANCE_CLEAR",
            AccountUpdateReason::AdminDeposit => "ADMIN_DEPOSIT",
            AccountUpdateReason::AdminWithdraw => "ADMIN_WITHDRAW",
            AccountUpdateReason::MarginTransfer => "MARGIN_TRANSFER",
            AccountUpdateReason::MarginTypeChange => "MARGIN_TYPE_CHANGE",
            AccountUpdateReason::AssetTransfer => "ASSET_TRANSFER",
            AccountUpdateReason::OptionsPremiumFee => "OPTIONS_PREMIUM_FEE",
            AccountUpdateReason::OptionsSettleProfit => "OPTIONS_SETTLE_PROFIT",
            AccountUpdateReason::AutoExchange => "AUTO_EXCHANGE",
            AccountUpdateReason::CoinSwapDeposit => "COIN_SWAP_DEPOSIT",
            AccountUpdateReason::CoinSwapWithdraw => "COIN_SWAP_WITHDRAW",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum MarginType {
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "crossed")]
    Crossed,
}

impl MarginType {
    pub fn to_str(&self) -> &str {
        match self {
            MarginType::Isolated => "isolated",
            MarginType::Crossed => "crossed",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum PriceMatch {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OPPONENT")]
    Opponent,
    #[serde(rename = "OPPONENT_5")]
    Opponent5,
    #[serde(rename = "OPPONENT_10")]
    Opponent10,
    #[serde(rename = "OPPONENT_20")]
    Opponent20,
    #[serde(rename = "QUEUE")]
    Queue,
    #[serde(rename = "QUEUE_5")]
    Queue5,
    #[serde(rename = "QUEUE_10")]
    Queue10,
    #[serde(rename = "QUEUE_20")]
    Queue20,
}

impl PriceMatch {
    pub fn to_str(&self) -> &str {
        match self {
            PriceMatch::None => "NONE",
            PriceMatch::Opponent => "OPPONENT",
            PriceMatch::Opponent5 => "OPPONENT_5",
            PriceMatch::Opponent10 => "OPPONENT_10",
            PriceMatch::Opponent20 => "OPPONENT_20",
            PriceMatch::Queue => "QUEUE",
            PriceMatch::Queue5 => "QUEUE_5",
            PriceMatch::Queue10 => "QUEUE_10",
            PriceMatch::Queue20 => "QUEUE_20",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum StpMode {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "EXPIRE_TAKER")]
    ExpireTaker,
    #[serde(rename = "EXPIRE_BOTH")]
    ExpireBoth,
    #[serde(rename = "EXPIRE_MAKER")]
    ExpireMaker,
}

impl StpMode {
    pub fn to_str(&self) -> &str {
        match self {
            StpMode::None => "NONE",
            StpMode::ExpireTaker => "EXPIRE_TAKER",
            StpMode::ExpireBoth => "EXPIRE_BOTH",
            StpMode::ExpireMaker => "EXPIRE_MAKER",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum ContractType {
    #[serde(rename = "PERPETUAL")]
    Perpetual,
    #[serde(rename = "CURRENT_MONTH")]
    CurrentMonth,
    #[serde(rename = "NEXT_MONTH")]
    NextMonth,
    #[serde(rename = "CURRENT_QUARTER")]
    CurrentQuarter,
    #[serde(rename = "NEXT_QUARTER")]
    NextQuarter,
    #[serde(rename = "PERPETUAL_DELIVERING")]
    PerpetualDelivering,
}

impl ContractType {
    pub fn to_str(&self) -> &str {
        match self {
            ContractType::Perpetual => "PERPETUAL",
            ContractType::CurrentMonth => "CURRENT_MONTH",
            ContractType::NextMonth => "NEXT_MONTH",
            ContractType::CurrentQuarter => "CURRENT_QUARTER",
            ContractType::NextQuarter => "NEXT_QUARTER",
            ContractType::PerpetualDelivering => "PERPETUAL_DELIVERING",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum ContractStatus {
    #[serde(rename = "PENDING_TRADING")]
    PendingTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "PRE_DELIVERING")]
    PreDelivering,
    #[serde(rename = "DELIVERING")]
    Delivering,
    #[serde(rename = "DELIVERED")]
    Delivered,
    #[serde(rename = "PRE_SETTLE")]
    PreSettle,
    #[serde(rename = "SETTLING")]
    Settling,
    #[serde(rename = "CLOSE")]
    Close,
}

impl ContractStatus {
    pub fn to_str(&self) -> &str {
        match self {
            ContractStatus::PendingTrading => "PENDING_TRADING",
            ContractStatus::Trading => "TRADING",
            ContractStatus::PreDelivering => "PRE_DELIVERING",
            ContractStatus::Delivering => "DELIVERING",
            ContractStatus::Delivered => "DELIVERED",
            ContractStatus::PreSettle => "PRE_SETTLE",
            ContractStatus::Settling => "SETTLING",
            ContractStatus::Close => "CLOSE",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum OrderStatus {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "EXPIRED")]
    Expired,
}

impl OrderStatus {
    pub fn to_str(&self) -> &str {
        match self {
            OrderStatus::New => "NEW",
            OrderStatus::PartiallyFilled => "PARTIALLY_FILLED",
            OrderStatus::Filled => "FILLED",
            OrderStatus::Canceled => "CANCELED",
            OrderStatus::Rejected => "REJECTED",
            OrderStatus::Expired => "EXPIRED",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum WorkingType {
    #[serde(rename = "MARK_PRICE")]
    MarkPrice,
    #[serde(rename = "CONTRACT_PRICE")]
    ContractPrice,
}

impl WorkingType {
    pub fn to_str(&self) -> &str {
        match self {
            WorkingType::MarkPrice => "MARK_PRICE",
            WorkingType::ContractPrice => "CONTRACT_PRICE",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
    GTD,
}

impl TimeInForce {
    pub fn to_str(&self) -> &str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::GTX => "GTX",
            TimeInForce::GTD => "GTD",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum ExecutionType {
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "CALCULATED")]
    Calculated,
    #[serde(rename = "EXPIRED")]
    Expired,
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "AMENDMENT")]
    Amendment,
}

impl ExecutionType {
    pub fn to_str(&self) -> &str {
        match self {
            ExecutionType::New => "NEW",
            ExecutionType::Canceled => "CANCELED",
            ExecutionType::Calculated => "CALCULATED",
            ExecutionType::Expired => "EXPIRED",
            ExecutionType::Trade => "TRADE",
            ExecutionType::Amendment => "AMENDMENT",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "STOP_MARKET")]
    StopMarket,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[serde(rename = "TRAILING_STOP_MARKET")]
    TrailingStopMarket,
}

impl OrderType {
    pub fn to_str(&self) -> &str {
        match self {
            OrderType::Limit => "LIMIT",
            OrderType::Market => "MARKET",
            OrderType::Stop => "STOP",
            OrderType::StopMarket => "STOP_MARKET",
            OrderType::TakeProfit => "TAKE_PROFIT",
            OrderType::TakeProfitMarket => "TAKE_PROFIT_MARKET",
            OrderType::TrailingStopMarket => "TRAILING_STOP_MARKET",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

impl Side {
    pub fn to_str(&self) -> &str {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, EnumString, PartialEq)]
pub enum PositionSide {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "SHORT")]
    Short,
    #[serde(rename = "BOTH")]
    Both,
}

impl PositionSide {
    pub fn to_str(&self) -> &str {
        match self {
            PositionSide::Long => "LONG",
            PositionSide::Short => "SHORT",
            PositionSide::Both => "BOTH",
        }
    }
}

/* CONFIG */

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum MarkPriceUpdateSpeed {
    #[serde(rename = "1s")]
    Seconds1,
    #[serde(rename = "3s")]
    Seconds3,
}

impl MarkPriceUpdateSpeed {
    pub fn to_str(&self) -> &str {
        match self {
            MarkPriceUpdateSpeed::Seconds1 => "1s",
            MarkPriceUpdateSpeed::Seconds3 => "3s",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum KlineInterval {
    #[serde(rename = "1m")]
    Minutes1,
    #[serde(rename = "3m")]
    Minutes3,
    #[serde(rename = "5m")]
    Minutes5,
    #[serde(rename = "15m")]
    Minutes15,
    #[serde(rename = "30m")]
    Minutes30,
    #[serde(rename = "1h")]
    Hours1,
    #[serde(rename = "2h")]
    Hours2,
    #[serde(rename = "4h")]
    Hours4,
    #[serde(rename = "6h")]
    Hours6,
    #[serde(rename = "8h")]
    Hours8,
    #[serde(rename = "12h")]
    Hours12,
    #[serde(rename = "1d")]
    Days1,
    #[serde(rename = "3d")]
    Days3,
    #[serde(rename = "1w")]
    Weeks1,
    #[serde(rename = "1M")]
    Months1,
}

impl KlineInterval {
    pub fn to_str(&self) -> &str {
        match self {
            KlineInterval::Minutes1 => "1m",
            KlineInterval::Minutes3 => "3m",
            KlineInterval::Minutes5 => "5m",
            KlineInterval::Minutes15 => "15m",
            KlineInterval::Minutes30 => "30m",
            KlineInterval::Hours1 => "1h",
            KlineInterval::Hours2 => "2h",
            KlineInterval::Hours4 => "4h",
            KlineInterval::Hours6 => "6h",
            KlineInterval::Hours8 => "8h",
            KlineInterval::Hours12 => "12h",
            KlineInterval::Days1 => "1d",
            KlineInterval::Days3 => "3d",
            KlineInterval::Weeks1 => "1w",
            KlineInterval::Months1 => "1M",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum KlineContractType {
    #[serde(rename = "PERPETUAL")]
    Perpetual,
    #[serde(rename = "CURRENT_QUARTER")]
    CurrentQuarter,
    #[serde(rename = "NEXT_QUARTER")]
    NextQuarter,
}

impl KlineContractType {
    pub fn to_str(&self) -> &str {
        match self {
            KlineContractType::Perpetual => "perpetual",
            KlineContractType::CurrentQuarter => "current_quarter",
            KlineContractType::NextQuarter => "next_quarter",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum PartialBookDepthLevel {
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "10")]
    Ten,
    #[serde(rename = "20")]
    Twenty,
}

impl PartialBookDepthLevel {
    pub fn to_str(&self) -> &str {
        match self {
            PartialBookDepthLevel::Five => "5",
            PartialBookDepthLevel::Ten => "10",
            PartialBookDepthLevel::Twenty => "20",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum BookDepthUpdateSpeed {
    /// Updates every 100 milliseconds.
    #[serde(rename = "100ms")]
    Millis100,
    /// Updates every 250 milliseconds.
    #[serde(rename = "250ms")]
    Millis250,
    /// Updates every 500 milliseconds.
    #[serde(rename = "500ms")]
    Millis500,
}

impl BookDepthUpdateSpeed {
    pub fn to_str(&self) -> &str {
        match self {
            BookDepthUpdateSpeed::Millis100 => "100ms",
            BookDepthUpdateSpeed::Millis250 => "250ms",
            BookDepthUpdateSpeed::Millis500 => "500ms",
        }
    }
}
