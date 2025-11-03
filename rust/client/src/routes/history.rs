use crate::{BpxClient, Result};
use bpx_api_types::history::{FillHistorySearchParams, HistoricFill, HistoricOrder, OrderHistorySearchParams};

#[doc(hidden)]
pub const API_FILL_HISTORY: &str = "/wapi/v1/history/fills";
pub const API_ORDER_HISTORY: &str = "/wapi/v1/history/orders";

impl BpxClient {
    /// Retrieves historical fills, with optional filtering for a specific order or symbol.
    pub async fn get_fill_history(&self, search_params: FillHistorySearchParams) -> Result<Vec<HistoricFill>> {
        let mut query_vec = Vec::new();

        if let Some(order_id) = search_params.order_id {
            query_vec.push(format!("order_id={order_id}"))
        }
        if let Some(strategy_id) = search_params.strategy_id {
            query_vec.push(format!("strategy_id={strategy_id}"))
        }
        if let Some(from) = search_params.from {
            query_vec.push(format!("from={from}"))
        }
        if let Some(to) = search_params.to {
            query_vec.push(format!("to={to}"))
        }
        if let Some(symbol) = search_params.symbol {
            query_vec.push(format!("symbol={symbol}"))
        }
        if let Some(limit) = search_params.limit {
            query_vec.push(format!("limit={limit}"))
        }
        if let Some(offset) = search_params.offset {
            query_vec.push(format!("offset={offset}"))
        }
        if let Some(fill_type) = search_params.fill_type {
            query_vec.push(format!("fill_type={fill_type}"))
        }
        if let Some(market_type) = search_params.market_type {
            query_vec.push(format!("market_type={market_type}"))
        }
        if let Some(sort_direction) = search_params.sort_direction {
            query_vec.push(format!("sort_direction={sort_direction}"))
        }
        let query_string = if query_vec.is_empty() {
            "".to_string()
        } else {
            format!("?{}", query_vec.join("&"))
        };
        let url = format!("{}{}{}", self.base_url, API_FILL_HISTORY, query_string);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves the order history for the user. This includes orders that have been filled and
    /// are no longer on the book. It may include orders that are on the book, but the `/orders`
    /// endpoint contains more up to date data.
    pub async fn get_order_history(&self, search_params: OrderHistorySearchParams) -> Result<Vec<HistoricOrder>> {
        let mut query_vec = Vec::new();

        if let Some(order_id) = search_params.order_id {
            query_vec.push(format!("order_id={order_id}"))
        }
        if let Some(strategy_id) = search_params.strategy_id {
            query_vec.push(format!("strategy_id={strategy_id}"))
        }
        if let Some(symbol) = search_params.symbol {
            query_vec.push(format!("symbol={symbol}"))
        }
        if let Some(limit) = search_params.limit {
            query_vec.push(format!("limit={limit}"))
        }
        if let Some(offset) = search_params.offset {
            query_vec.push(format!("offset={offset}"))
        }
        if let Some(market_type) = search_params.market_type {
            query_vec.push(format!("market_type={market_type}"))
        }
        if let Some(sort_direction) = search_params.sort_direction {
            query_vec.push(format!("sort_direction={sort_direction}"))
        }
        let query_string = if query_vec.is_empty() {
            "".to_string()
        } else {
            format!("?{}", query_vec.join("&"))
        };
        let url = format!("{}{}{}", self.base_url, API_ORDER_HISTORY, query_string);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
