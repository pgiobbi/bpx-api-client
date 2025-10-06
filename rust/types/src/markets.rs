use crate::margin::MarginFunction;
use crate::Blockchain;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// An asset is most of the time a crypto coin that can have multiple representations
/// across different blockchains. For example, USDT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// Identifier
    symbol: String,
    /// See [`Token`]
    tokens: Vec<Token>,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum MarketType {
    Spot,
    Perp,
    Iperp,
    Dated,
    Prediction,
    Rfq,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub first_price: Decimal,
    pub last_price: Decimal,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Decimal,
    pub trades: String,
}

/// Sent by an exchange to indicate a change in the order book, such as the execution of a bid or ask.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub ask_price: Decimal,

    #[serde(rename = "A")]
    pub ask_quantity: Decimal,

    #[serde(rename = "b")]
    pub bid_price: Decimal,

    #[serde(rename = "B")]
    pub bid_quantity: Decimal,

    /// Update ID of event
    #[serde(rename = "u")]
    pub update_id: u64,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: u64,
}

/// Ticker stream pushes 24hr rolling statistics for a single symbol every second.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerStatisticsUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// First price
    #[serde(rename = "o")]
    pub first_price: Decimal,

    /// Last price
    #[serde(rename = "c")]
    pub last_price: Decimal,

    /// High price
    #[serde(rename = "h")]
    pub high_price: Decimal,

    /// Low price
    #[serde(rename = "l")]
    pub low_price: Decimal,

    /// Base asset volume
    #[serde(rename = "v")]
    pub base_asset_volume: Decimal,

    /// Quote asset volume
    #[serde(rename = "V")]
    pub quote_asset_volume: Decimal,

    /// Number of trades
    #[serde(rename = "n")]
    pub number_of_trades: u64,
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "PascalCase")]
#[serde(rename_all = "PascalCase")]
pub enum OrderBookState {
    Open,
    Closed,
    CancelOnly,
    LimitOnly,
    PostOnly,
}

/// A market is where two assets are exchanged. Most notably, in a `BTC/USDC` pair
/// `BTC` is the base and `USDC` is the quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    /// The `Market` identifier.
    pub symbol: String,
    /// The base asset.
    pub base_symbol: String,
    /// The quote asset for the market.
    pub quote_symbol: String,
    /// The type of the market, either `Spot` or `Perp`.
    pub market_type: MarketType,
    /// See [`MarketFilters`].
    pub filters: MarketFilters,
    /// IMF function.
    pub imf_function: Option<MarginFunction>,
    /// MMF function.
    pub mmf_function: Option<MarginFunction>,
    /// Funding interval for perpetuals in milliseconds.
    pub funding_interval: Option<u64>,
    /// Funding rate upper bound for perpetual markets. In basis points. E.g. 10 = 10bps
    pub funding_rate_upper_bound: Option<Decimal>,
    /// Funding rate lower bound for perpetual markets. In basis points. E.g. -10 = -10bps
    pub funding_rate_lower_bound: Option<Decimal>,
    /// Maximum open interest limit for the market if the market is a future.
    pub open_interest_limit: Option<Decimal>,
    /// The order book state.
    pub order_book_state: OrderBookState,
    /// Market created at time.
    pub created_at: chrono::NaiveDateTime,
}

impl Market {
    /// Returns the decimal places this market supports on the price.
    /// We error if a price with more decimal places is provided.
    /// `Price decimal too long`
    pub const fn price_decimal_places(&self) -> u32 {
        self.filters.price.tick_size.scale()
    }

    /// Returns the decimal places this market supports on the quantity.
    /// if you provide a more precise quantity you will get an error
    /// `Quantity decimal too long`
    pub const fn quantity_decimal_places(&self) -> u32 {
        self.filters.quantity.step_size.scale()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketFilters {
    /// Defines the price rules for the order book.
    pub price: PriceFilters,
    /// Defines the quantity rules for the order book.
    pub quantity: QuantityFilters,
    pub leverage: Option<LeverageFilters>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceBandMarkPrice {
    /// Maximum allowed multiplier move from mean price.
    pub max_multiplier: Decimal,
    /// Minimum allowed multiplier move from mean price.
    pub min_multiplier: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceBandMeanPremium {
    /// Maximum allowed deviation from the mean premium. E.g. if tolerance_pct is 0.05 (5%), and
    /// the mean premium is 5%, then orders will be prevented from being placed if the premium
    /// exceeds 10%.
    pub tolerance_pct: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilters {
    /// Minimum price the order book will allow.
    pub min_price: Decimal,
    /// Maximum price the order book will allow.
    pub max_price: Option<Decimal>,
    /// Price increment.
    pub tick_size: Decimal,
    /// Maximum allowed multiplier from last active price.
    pub max_multiplier: Option<Decimal>,
    /// Minimum allowed multiplier from last active price.
    pub min_multiplier: Option<Decimal>,
    /// Maximum allowed impact multiplier from best offer. This determines how far above the best
    /// ask a market buy can penetrate.
    pub max_impact_multiplier: Option<Decimal>,
    /// Minimum allowed impact multiplier from best bid. This determines how far below the best
    /// bid a market sell can penetrate.
    pub min_impact_multiplier: Option<Decimal>,
    /// Futures price band. Used to determine how far the price is allowed to deviate from the mean
    /// mark price.
    pub mean_mark_price_band: Option<PriceBandMarkPrice>,
    /// Futures price band. Used to determine how far the premium is allowed to deviate from the
    /// mean premium.
    pub mean_premium_band: Option<PriceBandMeanPremium>,
    /// Maximum allowed multiplier move from last active price without incurring an entry fee for
    /// spot margin.
    pub borrow_entry_fee_max_multiplier: Option<Decimal>,
    /// Minimum allowed multiplier move from last active price without incurring an entry fee for
    /// spot margin.
    pub borrow_entry_fee_min_multiplier: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityFilters {
    /// Minimum quantity the order book will allow. For futures, this will be the threshold at
    /// which a position gets closed and so it should be as close as possible, preferably equal, to
    /// the step_size.
    pub min_quantity: Decimal,
    /// Maximum quantity the order book will allow.
    pub max_quantity: Option<Decimal>,
    /// Quantity increment.
    pub step_size: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilters {
    pub min_leverage: Decimal,
    pub max_leverage: Decimal,
    pub step_size: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub blockchain: Blockchain,
    pub deposit_enabled: bool,
    pub minimum_deposit: Decimal,
    pub withdraw_enabled: bool,
    pub minimum_withdrawal: Decimal,
    pub maximum_withdrawal: Option<Decimal>,
    pub withdrawal_fee: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDepth {
    pub asks: Vec<(Decimal, Decimal)>,
    pub bids: Vec<(Decimal, Decimal)>,
    pub last_update_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookDepthUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub timestamp: i64,

    /// First update ID in event
    #[serde(rename = "U")]
    pub first_update_id: u64,

    /// Last update ID in event
    #[serde(rename = "u")]
    pub last_update_id: u64,

    /// Asks
    #[serde(rename = "a")]
    pub asks: Vec<(Decimal, Decimal)>,

    /// Bids
    #[serde(rename = "b")]
    pub bids: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    pub start: String,
    pub open: Option<Decimal>,
    pub high: Option<Decimal>,
    pub low: Option<Decimal>,
    pub close: Option<Decimal>,
    pub end: Option<String>,
    pub volume: Decimal,
    pub trades: u64,
}

/// Ticker stream pushes 24hr rolling statistics for a single symbol every second.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// K-Line start time in seconds
    #[serde(rename = "t")]
    pub start: chrono::NaiveDateTime,

    /// K-Line close time in seconds
    #[serde(rename = "T")]
    pub end: chrono::NaiveDateTime,

    /// Open price
    #[serde(rename = "o")]
    pub open: Decimal,

    /// Open price
    #[serde(rename = "c")]
    pub close: Decimal,

    /// High price
    #[serde(rename = "h")]
    pub high: Decimal,

    /// Low price
    #[serde(rename = "l")]
    pub low: Decimal,

    /// Base asset volume
    #[serde(rename = "v")]
    pub base_asset_volume: Decimal,

    /// Number of trades
    #[serde(rename = "n")]
    pub number_of_trades: u64,

    /// Is this k-line closed?
    #[serde(rename = "X")]
    pub is_closed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub interval_end_timestamp: String,
    pub funding_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    pub funding_rate: Decimal,
    pub index_price: Decimal,
    pub mark_price: Decimal,
    pub next_funding_timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceUpdate {
    /// Event type
    #[serde(rename = "e")]
    pub event_type: String,

    /// Event timestamp in microseconds
    #[serde(rename = "E")]
    pub event_time: i64,

    /// Symbol
    #[serde(rename = "s")]
    pub symbol: String,

    /// Mark Price
    #[serde(rename = "p")]
    pub mark_price: Decimal,

    /// Estimated funding rate
    #[serde(rename = "f")]
    pub funding_rate: Decimal,

    /// Index Price
    #[serde(rename = "i")]
    pub index_price: Decimal,

    /// Next funding timestamp in microseconds
    #[serde(rename = "n")]
    pub funding_timestamp: u64,

    /// Engine timestamp in microseconds
    #[serde(rename = "T")]
    pub engine_timestamp: i64,
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_decimal_macros::dec;

    fn get_test_market() -> Market {
        Market {
            symbol: "TEST_MARKET".to_string(),
            base_symbol: "TEST".to_string(),
            quote_symbol: "MARKET".to_string(),
            market_type: MarketType::Spot,
            filters: super::MarketFilters {
                price: PriceFilters {
                    min_price: dec!(0.0001),
                    max_price: None,
                    tick_size: dec!(0.0001),
                    max_multiplier: None,
                    min_multiplier: None,
                    max_impact_multiplier: None,
                    min_impact_multiplier: None,
                    mean_mark_price_band: None,
                    mean_premium_band: None,
                    borrow_entry_fee_max_multiplier: None,
                    borrow_entry_fee_min_multiplier: None,
                },
                quantity: QuantityFilters {
                    min_quantity: dec!(0.01),
                    max_quantity: None,
                    step_size: dec!(0.01),
                },
                leverage: None,
            },
            imf_function: None,
            mmf_function: None,
            funding_interval: None,
            funding_rate_upper_bound: None,
            funding_rate_lower_bound: None,
            open_interest_limit: None,
            order_book_state: OrderBookState::Open,
            created_at: Default::default(),
        }
    }

    #[test]
    fn test_decimal_places_on_price_filters_4() {
        let market = get_test_market();
        assert_eq!(market.price_decimal_places(), 4);
    }

    #[test]
    fn test_decimal_places_on_quantity_filters() {
        let market = get_test_market();
        assert_eq!(market.quantity_decimal_places(), 2);
    }

    #[test]
    fn test_mark_price_update_parse() {
        let data = r#"
{
	"E": 1747291031914525,
	"T": 1747291031910025,
	"e": "markPrice",
	"f": "-0.0000039641039274236048482914",
	"i": "173.44031179",
	"n": 1747296000000,
	"p": "173.35998175",
	"s": "SOL_USDC_PERP"
}
        "#;

        let mark_price_update: MarkPriceUpdate = serde_json::from_str(data).unwrap();
        assert_eq!(mark_price_update.symbol, "SOL_USDC_PERP".to_string());
        assert_eq!(mark_price_update.funding_rate, dec!(-0.0000039641039274236048482914));
        assert_eq!(mark_price_update.mark_price, dec!(173.35998175));
    }
}
