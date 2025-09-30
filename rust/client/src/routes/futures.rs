use bpx_api_types::futures::FuturePosition;

use crate::error::Result;
use crate::BpxClient;

#[doc(hidden)]
pub const API_FUTURES_POSITION: &str = "/api/v1/position";

impl BpxClient {
    pub async fn get_open_future_positions(&self, symbol: Option<&str>) -> Result<Vec<FuturePosition>> {
        let mut url = format!("{}{}", self.base_url, API_FUTURES_POSITION);
        if let Some(s) = symbol {
            url.push_str(&format!("?symbol={s}"));
        }
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }
}
