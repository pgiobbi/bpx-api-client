use crate::markets::MarketType;
use crate::order::{OrderStatus, OrderType, SelfTradePrevention, Side, SlippageToleranceType, TimeInForce, TriggerBy};
use crate::SortDirection;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillHistorySearchParams {
    /// Filter to the given order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Filter to the given strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    /// Filter to minimum time (milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// Filter to maximum time (milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// Filter to the given symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Maximum number to return. Default 100, maximum 1000.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "FillHistorySearchParams::default_limit"
    )]
    pub limit: Option<u64>,
    /// Offset. Default 0.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "FillHistorySearchParams::default_offset"
    )]
    pub offset: Option<u64>,
    /// Fill type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type: Option<FillType>,
    /// Market type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<MarketType>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

/// Default values for the fill history search params.
impl FillHistorySearchParams {
    fn default_limit() -> Option<u64> {
        Some(100)
    }

    fn default_offset() -> Option<u64> {
        Some(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricFill {
    /// Client id of the order.
    pub client_id: Option<String>,
    /// The fee charged on the fill.
    pub fee: Decimal,
    /// The asset that is charged as a fee.
    pub fee_symbol: String,
    /// Whether the fill was made by the maker.
    pub is_maker: bool,
    /// The order ID of the fill.
    pub order_id: String,
    /// The price of the fill.
    pub price: Decimal,
    /// The quantity of the fill.
    pub quantity: Decimal,
    /// The side of the fill.
    pub side: Side,
    /// The market symbol of the fill.
    pub symbol: String,
    /// The type of system order that triggered the fill.
    pub system_order_type: Option<SystemOrderType>,
    /// The timestamp of the fill (UTC).
    pub timestamp: chrono::NaiveDateTime,
    /// The trade ID of the fill.
    pub trade_id: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistorySearchParams {
    /// Filter to the given order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Filter to the given strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    /// Filter to the given symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Maximum number to return. Default 100, maximum 1000.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "OrderHistorySearchParams::default_limit"
    )]
    pub limit: Option<u64>,
    /// Offset. Default 0.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "OrderHistorySearchParams::default_offset"
    )]
    pub offset: Option<u64>,
    /// Market type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<MarketType>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,
}

/// Default values for the fill history search params.
impl OrderHistorySearchParams {
    fn default_limit() -> Option<u64> {
        Some(100)
    }

    fn default_offset() -> Option<u64> {
        Some(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: use a HistoricOrder enum instead of this struct, similarly to `Order`
pub struct HistoricOrder {
    /// Unique ID of the order.
    pub id: String,
    /// Time the order was created.
    pub created_at: chrono::NaiveDateTime,
    /// Quantity of the order that has been filled.
    pub executed_quantity: Option<Decimal>,
    /// Quantity of the order that has been filled in the quote asset.
    pub executed_quote_quantity: Option<Decimal>,
    /// Order expiry reason.
    pub expiry_reason: Option<OrderExpiryReason>,
    /// Type of order.
    pub order_type: HistoricOrderType,
    /// Whether the order is post only or not.
    pub post_only: Option<bool>,
    /// Price that the order was submitted at (if orderType is Limit)
    pub price: Option<Decimal>,
    /// Quantity of the order.
    pub quantity: Option<Decimal>,
    /// Quantity of the order in quote the quote asset.
    pub quote_quantity: Option<Decimal>,
    /// Self trade prevention setting of the order. Default is `RejectTaker`.
    pub self_trade_prevention: SelfTradePrevention,
    /// Status of the order.
    pub status: OrderStatus,
    /// Side of the order.
    pub side: Side,
    /// Stop loss price (price the stop loss order will be triggered at).
    pub stop_loss_trigger_price: Option<Decimal>,
    /// Stop loss limit price. If set the stop loss will be a limit order, otherwise it will be a market order.
    pub stop_loss_limit_price: Option<Decimal>,
    /// Reference price that should trigger the stop loss order.
    pub stop_loss_trigger_by: Option<TriggerBy>,
    /// The market symbol of the fill.
    pub symbol: String,
    /// Take profit price (price the take profit order will be triggered at).
    pub take_profit_trigger_price: Option<Decimal>,
    /// Take profit limit price. If set the take profit will be a limit order, otherwise it will be a market order.
    pub take_profit_limit_price: Option<Decimal>,
    /// Reference price that should trigger the take profit order.
    pub take_profit_trigger_by: Option<TriggerBy>,
    /// Time in force of the order.
    pub time_in_force: Option<TimeInForce>,
    /// Reference price that should trigger the order.
    pub trigger_by: Option<TriggerBy>,
    /// Price the order was set to trigger at.
    pub trigger_price: Option<Decimal>,
    /// Trigger quantity.
    pub trigger_quantity: Option<Decimal>,
    /// Custom order ID.
    pub client_id: Option<u32>,
    /// The type of system order, if applicable.
    pub system_order_type: Option<SystemOrderType>,
    /// Strategy ID of the order, if any.
    pub strategy_id: Option<String>,
    /// Slippage tolerance allowed for the order.
    pub slippage_tolerance: Option<Decimal>,
    /// Slippage tolerance type.
    pub slippage_tolerance_type: Option<SlippageToleranceType>,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum FillType {
    User,
    BookLiquidation,
    Adl,
    Backstop,
    Liquidation,
    AllLiquidation,
    CollateralConversion,
    CollateralConversionAndSpotLiquidation,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum SystemOrderType {
    CollateralConversion,
    FutureExpiry,
    LiquidatePositionOnAdl,
    LiquidatePositionOnBook,
    LiquidatePositionOnBackstop,
    OrderBookClosed,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderExpiryReason {
    AccountTradingSuspended,
    BorrowRequiresLendRedeem,
    FillOrKill,
    InsufficientBorrowableQuantity,
    InsufficientFunds,
    InsufficientLiquidity,
    InvalidPrice,
    InvalidQuantity,
    ImmediateOrCancel,
    InsufficientMargin,
    Liquidation,
    NegativeEquity,
    PostOnlyMode,
    PostOnlyTaker,
    PriceOutOfBounds,
    ReduceOnlyNotReduced,
    SelfTradePrevention,
    StopWithoutPosition,
    PriceImpact,
    Unknown,
    UserPermissions,
    MaxStopOrdersPerPosition,
    PositionLimit,
    SlippageToleranceExceeded,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum HistoricOrderType {
    Limit,
    Market,
}
