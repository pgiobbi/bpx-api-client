use crate::markets::MarketType;
use crate::order::{SelfTradePrevention, Side, SlippageToleranceType, TimeInForce};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyHistorySearchParams {
    /// Filter to the given strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    /// Filter to the given symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Maximum number to return. Default 100, maximum 1000.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "StrategyHistorySearchParams::default_limit"
    )]
    pub limit: Option<u64>,
    /// Offset. Default 0.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "StrategyHistorySearchParams::default_offset"
    )]
    pub offset: Option<u64>,
    /// Market type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<MarketType>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

/// Default values for the strategy history search params.
impl StrategyHistorySearchParams {
    fn default_limit() -> Option<u64> {
        Some(100)
    }

    fn default_offset() -> Option<u64> {
        Some(0)
    }
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Strategy {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub executed_quantity: Option<Decimal>,
    pub executed_quote_quantity: Option<Decimal>,
    pub cancel_reason: Option<StrategyCancelReason>,
    pub strategy_type: String,
    pub quantity: Option<Decimal>,
    pub self_trade_prevention: SelfTradePrevention,
    pub status: StrategyStatus,
    pub side: Side,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub client_strategy_id: Option<u32>,
    pub duration: u64,
    pub interval: u64,
    pub randomized_interval_quantity: bool,
    pub slippage_tolerance: Option<Decimal>,
    pub slippage_tolerance_type: Option<SlippageToleranceType>,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum StrategyCancelReason {
    Expired,
    FillOrKill,
    InsufficientBorrowableQuantity,
    InsufficientFunds,
    InsufficientLiquidity,
    InvalidPrice,
    InvalidQuantity,
    InsufficientMargin,
    Liquidation,
    PriceOutOfBounds,
    ReduceOnlyNotReduced,
    SelfTradePrevention,
    Unknown,
    UserPermissions,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum StrategyStatus {
    Running,
    Completed,
    Cancelled,
    Terminated,
}
