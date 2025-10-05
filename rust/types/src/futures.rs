use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::margin::MarginFunction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuturePosition {
    pub break_even_price: Decimal,
    pub cumulative_funding_payment: Decimal,
    pub entry_price: Decimal,
    pub est_liquidation_price: Decimal,
    pub imf: Decimal,
    pub imf_function: MarginFunction,
    pub mark_price: Decimal,
    pub mmf: Decimal,
    pub mmf_function: MarginFunction,
    pub net_cost: Decimal,
    pub net_exposure_notional: Decimal,
    pub net_exposure_quantity: Decimal,
    pub net_quantity: Decimal,
    pub pnl_realized: Decimal,
    pub pnl_unrealized: Decimal,
    pub position_id: String,
    pub subaccount_id: Option<u64>,
    pub symbol: String,
    pub user_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PositionUpdateType {
    PositionAdjusted,
    PositionOpened,
    PositionClosed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: Option<PositionUpdateType>,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Break event price
    #[serde(rename = "b")]
    pub break_even_price: Decimal,

    /// Entry price
    #[serde(rename = "B")]
    pub entry_price: Decimal,

    /// Initial margin fraction
    #[serde(rename = "f")]
    pub imf: Decimal,

    /// Mark price
    #[serde(rename = "M")]
    pub mark_price: Decimal,

    /// Maintenance margin fraction
    #[serde(rename = "m")]
    pub mmf: Decimal,

    /// Net quantity
    #[serde(rename = "q")]
    pub net_quantity: Decimal,

    /// Net exposure quantity
    #[serde(rename = "Q")]
    pub net_exposure_quantity: Decimal,

    /// Net exposure notional
    #[serde(rename = "n")]
    pub net_exposure_notional: Decimal,

    /// Position ID
    #[serde(rename = "i")]
    pub position_id: u64,

    /// PnL realized
    #[serde(rename = "p")]
    pub pnl_realized: Decimal,

    /// PnL realized
    #[serde(rename = "P")]
    pub pnl_unrealized: Decimal,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: u64,

    /// Estimated liquidation price
    #[deprecated]
    #[serde(rename = "l")]
    pub est_liquidation_price: Option<Decimal>,
}

/// Open interest updates are pushed to the openInterest stream every 60 seconds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Open interest in contracts
    #[serde(rename = "o")]
    pub open_interest: Decimal,
}
