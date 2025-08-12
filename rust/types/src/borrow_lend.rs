use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use crate::margin::MarginFunction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendPosition {
    pub cumulative_interest: Decimal,
    pub id: String,
    pub symbol: String,
    pub imf: Decimal,
    pub imf_function: MarginFunction,
    pub mark_price: Decimal,
    pub mmf: Decimal,
    pub mmf_function: MarginFunction,
    pub net_exposure_notional: Decimal,
    pub net_exposure_quantity: Decimal,
    pub net_quantity: Decimal,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum BorrowLendMarketState {
    Open,
    Closed,
    RepayOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendMarket {
    pub state: BorrowLendMarketState,
    pub asset_mark_price: Decimal,
    pub borrow_interest_rate: Decimal,
    pub borrowed_quantity: Decimal,
    pub fee: Decimal,
    pub lend_interest_rate: Decimal,
    pub lent_quantity: Decimal,
    pub max_utilization: Decimal,
    pub open_borrow_lend_limit: Decimal,
    pub optimal_utilization: Decimal,
    pub symbol: String,
    pub timestamp: chrono::DateTime<chrono::FixedOffset>,
    pub throttle_utilization_threshold: Decimal,
    pub throttle_utilization_bound: Decimal,
    pub throttle_update_fraction: Decimal,
    pub utilization: Decimal,
    pub step_size: Decimal,
}