use crate::error::Result;
use crate::BpxClient;
use bpx_api_types::account::{
    AccountMaxBorrow, AccountMaxOrder, AccountMaxWithdrawal, AccountSettings, ConvertDustPayload, UpdateAccountPayload,
};
use bpx_api_types::order::Side;
use rust_decimal::Decimal;

#[doc(hidden)]
pub const API_ACCOUNT: &str = "/api/v1/account";
#[doc(hidden)]
pub const API_ACCOUNT_MAX_BORROW: &str = "/api/v1/account/limits/borrow";
#[doc(hidden)]
pub const API_ACCOUNT_MAX_ORDER: &str = "/api/v1/account/limits/order";
#[doc(hidden)]
pub const API_ACCOUNT_MAX_WITHDRAWAL: &str = "/api/v1/account/limits/withdrawal";
#[doc(hidden)]
pub const API_ACCOUNT_CONVERT_DUST: &str = "/api/v1/account/convertDust";

impl BpxClient {
    /// Fetches the account's settings.
    pub async fn get_account(&self) -> Result<AccountSettings> {
        let url = format!("{}{}", self.base_url, API_ACCOUNT);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the account's maximum borrow amount for a given symbol.
    pub async fn get_account_max_borrow(&self, symbol: &str) -> Result<AccountMaxBorrow> {
        let url = format!("{}{}?symbol={}", self.base_url, API_ACCOUNT_MAX_BORROW, symbol);
        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the maximum quantity an account can trade for a given symbol based on the
    /// account's balances, existing exposure and margin requirements.
    pub async fn get_account_max_order_quantity(
        &self,
        symbol: &str,
        side: Side,
        price: Option<Decimal>,
        reduce_only: Option<bool>,
        auto_borrow: Option<bool>,
        auto_borrow_repay: Option<bool>,
        auto_lend_redeem: Option<bool>,
    ) -> Result<AccountMaxOrder> {
        let mut url = format!(
            "{}{}?symbol={}&side={}",
            self.base_url, API_ACCOUNT_MAX_ORDER, symbol, side
        );
        if let Some(price) = price {
            url.push_str(&format!("&price={price}"));
        }
        if let Some(reduce_only) = reduce_only {
            url.push_str(&format!("&reduceOnly={reduce_only}"));
        }
        if let Some(auto_borrow) = auto_borrow {
            url.push_str(&format!("&autoBorrow={auto_borrow}"));
        }
        if let Some(auto_borrow_repay) = auto_borrow_repay {
            url.push_str(&format!("&autoBorrowRepay={auto_borrow_repay}"));
        }
        if let Some(auto_lend_redeem) = auto_lend_redeem {
            url.push_str(&format!("&autoLendRedeem={auto_lend_redeem}"));
        }

        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Fetches the account's maximum withdrawal amount for a given symbol.
    pub async fn get_account_max_withdrawal(
        &self,
        symbol: &str,
        auto_borrow: Option<bool>,
        auto_lend_redeem: Option<bool>,
    ) -> Result<AccountMaxWithdrawal> {
        let mut url = format!("{}{}?symbol={}", self.base_url, API_ACCOUNT_MAX_WITHDRAWAL, symbol);
        if let Some(auto_borrow) = auto_borrow {
            url.push_str(&format!("&autoBorrow={auto_borrow}"));
        }
        if let Some(auto_lend_redeem) = auto_lend_redeem {
            url.push_str(&format!("&autoLendRedeem={auto_lend_redeem}"));
        }

        let res = self.get(url).await?;
        res.json().await.map_err(Into::into)
    }

    /// Updates the account's settings.
    pub async fn update_account(&self, payload: UpdateAccountPayload) -> Result<()> {
        let url = format!("{}{}", self.base_url, API_ACCOUNT);
        self.patch(url, payload).await?;

        Ok(())
    }

    /// Converts a dust balance to USDC. The balance (including lend) must be less
    /// than the minimum quantity tradable on the spot order book.
    pub async fn convert_dust_balance(&self, payload: ConvertDustPayload) -> Result<()> {
        let url = format!("{}{}", self.base_url, API_ACCOUNT_CONVERT_DUST);
        self.post(url, payload).await?;

        Ok(())
    }
}
