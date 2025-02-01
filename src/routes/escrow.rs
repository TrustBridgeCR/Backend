use crate::models::escrow::{Escrow, EscrowStatus};
use crate::services::escrow::EscrowService;

use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use std::sync::Arc;
pub struct AppState {
    escrow_service: Arc<EscrowService>,
}

pub fn escrow_routes(escrow_service: EscrowService) -> Router {
    let shared_state = Arc::new(AppState {
        escrow_service: Arc::new(escrow_service),
    });

    Router::new()
        .route("/escrows", post(create_escrow))
        .route("/escrows/:id", get(get_escrow))
        .route("/escrows/:id/status", put(update_status))
        .route("/escrows/:id/cancel", post(cancel_and_refund))
        .route("/escrows/:id/release", post(release_funds))
        .route("/escrows/:id/lock", post(lock_funds))
        .with_state(shared_state)
}

async fn create_escrow(
    State(state): State<Arc<AppState>>,
    Json(escrow): Json<Escrow>,
) -> Result<Json<Escrow>, String> {
    state.escrow_service.create_escrow(escrow).await.map(Json)
}

async fn get_escrow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Escrow>, String> {
    state.escrow_service.get_escrow(id).await.map(Json)
}

async fn update_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(status): Json<String>,
) -> Result<Json<Escrow>, String> {
    let new_status = EscrowStatus::from_string(&status)?;
    state
        .escrow_service
        .update_status(id, new_status)
        .await
        .map(Json)
}

async fn cancel_and_refund(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Escrow>, String> {
    state.escrow_service.cancel_and_refund(id).await.map(Json)
}

async fn release_funds(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Escrow>, String> {
    state.escrow_service.release_funds(id).await.map(Json)
}

async fn lock_funds(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(amount): Json<i64>,
) -> Result<Json<Escrow>, String> {
    state.escrow_service.lock_funds(id, amount).await.map(Json)
}
