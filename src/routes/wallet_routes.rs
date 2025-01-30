use axum::{Json, Router, routing::post};
use crate::models::wallet::WalletIntegration;
use crate::services::authenticate_wallet;

pub fn wallet_routes() -> Router {
    Router::new()
        .route("/authenticate_wallet", post(authenticate_wallet_handler))
}

async fn authenticate_wallet_handler(Json(wallet): Json<WalletIntegration>) -> Json<bool> {
    let result = authenticate_wallet(&wallet).await.unwrap_or(false);
    Json(result)
}
