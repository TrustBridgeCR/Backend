#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WalletIntegration {
    pub user_id: String,
    pub wallet_address: String,
    pub wallet_provider: WalletProvider,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum WalletProvider {
    Albedo,
    Freighter,
}
