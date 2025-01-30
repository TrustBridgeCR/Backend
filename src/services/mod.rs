use crate::models::wallet::{WalletIntegration, WalletProvider};
use reqwest::Client;

pub async fn authenticate_wallet(wallet: &WalletIntegration) -> Result<bool, String> {
    let client = Client::new();
    match wallet.wallet_provider {
        WalletProvider::Albedo => {
            let response = client.post("https://albedo.api.endpoint")
                .json(wallet)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(response.status().is_success())
        }
        WalletProvider::Freighter => {
            let response = client.post("https://freighter.api.endpoint")
                .json(wallet)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            Ok(response.status().is_success())
        }
    }
}

pub mod escrow;
