use bpx_api_types::markets::{
    Asset, FundingRate, Kline, KlineInterval, KlinePriceType, MarkPrice, Market, OrderBookDepth, Ticker,
};

use crate::error::Result;
use crate::BpxClient;

const API_ASSETS: &str = "/api/v1/assets";
const API_MARKET: &str = "/api/v1/market";
const API_MARKETS: &str = "/api/v1/markets";
const API_TICKER: &str = "/api/v1/ticker";
const API_TICKERS: &str = "/api/v1/tickers";
const API_DEPTH: &str = "/api/v1/depth";
const API_KLINES: &str = "/api/v1/klines";
const API_FUNDING: &str = "/api/v1/fundingRates";
const API_MARK_PRICES: &str = "/api/v1/markPrices";

impl BpxClient {
    /// Fetches available assets and their associated tokens.
    pub async fn get_assets(&self) -> Result<Vec<Asset>> {
        let url = format!("{}{}", self.base_url, API_ASSETS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves the market for a given symbol.
    pub async fn get_market(&self, symbol: &str) -> Result<Market> {
        let url = format!("{}{}?symbol={}", self.base_url, API_MARKET, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves a list of available markets.
    pub async fn get_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}{}", self.base_url, API_MARKETS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves mark price, index price and the funding rate for the current interval for all symbols, or the symbol specified.
    pub async fn get_all_mark_prices(&self) -> Result<Vec<MarkPrice>> {
        let url = format!("{}{}", self.base_url, API_MARK_PRICES);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the ticker information for a given symbol.
    pub async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let url = format!("{}{}?symbol={}", self.base_url, API_TICKER, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the ticker information for all symbols.
    pub async fn get_tickers(&self) -> Result<Vec<Ticker>> {
        let url = format!("{}{}", self.base_url, API_TICKERS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves the order book depth for a given symbol.
    pub async fn get_order_book_depth(&self, symbol: &str) -> Result<OrderBookDepth> {
        let url = format!("{}{}?symbol={}", self.base_url, API_DEPTH, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Funding interval rate history for futures.
    pub async fn get_funding_interval_rates(&self, symbol: &str) -> Result<Vec<FundingRate>> {
        let url = format!("{}{}?symbol={}", self.base_url, API_FUNDING, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches historical K-line (candlestick) data for a given symbol and interval.
    pub async fn get_k_lines(
        &self,
        symbol: &str,
        interval: KlineInterval,
        start_time: i64,
        end_time: Option<i64>,
        price_type: Option<KlinePriceType>,
    ) -> Result<Vec<Kline>> {
        let mut url = format!(
            "{}{}?symbol={}&interval={}&startTime={}",
            self.base_url, API_KLINES, symbol, interval, start_time
        );

        if let Some(end_time) = end_time {
            url.push_str(&format!("&endTime={end_time}"));
        }
        if let Some(price_type) = price_type {
            url.push_str(&format!("&priceType={price_type}"));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
