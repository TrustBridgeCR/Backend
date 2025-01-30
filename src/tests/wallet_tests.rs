#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_authenticate_wallet_with_albedo() {
        let _mock = mock("POST", "/")
            .with_status(200)
            .create();

        let wallet = WalletIntegration {
            user_id: "test_user".to_string(),
            wallet_address: "albedo_wallet".to_string(),
            wallet_provider: WalletProvider::Albedo,
        };

        let result = authenticate_wallet(&wallet).await;
        assert!(result.unwrap());
    }
}
