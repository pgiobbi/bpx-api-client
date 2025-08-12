use bpx_api_types::borrow_lend::{BorrowLendMarket, BorrowLendPosition};

use crate::{BpxClient, Result};

#[doc(hidden)]
pub const API_BORROW_LEND_POSITIONS: &str = "/api/v1/borrowLend/positions";
#[doc(hidden)]
pub const API_BORROW_LEND_MARKETS: &str = "/api/v1/borrowLend/markets";

impl BpxClient {
    /// Retrieves all the open borrow lending positions for the account.
    pub async fn get_borrow_lend_positions(&self) -> Result<Vec<BorrowLendPosition>> {
        let url = format!("{}{}", self.base_url, API_BORROW_LEND_POSITIONS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Retrieves all borrow lending markets.
    pub async fn get_borrow_lend_markets(&self) -> Result<Vec<BorrowLendMarket>> {
        let url = format!("{}{}", self.base_url, API_BORROW_LEND_MARKETS);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
