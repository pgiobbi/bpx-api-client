use crate::{BpxClient, Result};
use bpx_api_types::strategies::{Strategy, StrategyHistorySearchParams};

#[doc(hidden)]
pub const API_STRATEGY_HISTORY: &str = "/wapi/v1/history/strategies";

impl BpxClient {
    /// Retrieves the strategy history for the user. This returns strategies that are no longer
    /// active as they have either been completed, cancelled by the user or cancelled by the
    /// system.
    pub async fn get_strategy_history(&self, search_params: StrategyHistorySearchParams) -> Result<Vec<Strategy>> {
        let mut query_vec = Vec::new();

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
        let url = format!("{}{}{}", self.base_url, API_STRATEGY_HISTORY, query_string);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
