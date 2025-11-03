use crate::markets::MarketType;
use crate::order::Side;
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

/// Default values for the strategy history search params.
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
pub struct Fill {
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
