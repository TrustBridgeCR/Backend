use crate::models::escrow::{Escrow, EscrowStatus};
use crate::services::escrow::EscrowService;
use sqlx::PgPool;
use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use std::sync::Arc;
use tracing::error;

pub struct AppState {
    escrow_service: Arc<EscrowService>,
    db_pool: Arc<PgPool>,
}

pub fn escrow_routes(escrow_service: EscrowService, db_pool: PgPool) -> Router {
    let shared_state = Arc::new(AppState {
        escrow_service: Arc::new(escrow_service),
        db_pool: Arc::new(db_pool),
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
    let mut tx = state.db_pool.begin().await.map_err(|e| e.to_string())?;
    let result = state.escrow_service.create_escrow(&mut tx, escrow).await;
    match result {
        Ok(escrow) => {
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(Json(escrow))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            Err(e)
        }
    }
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
    let mut tx = state.db_pool.begin().await.map_err(|e| e.to_string())?;
    let result = state.escrow_service.cancel_and_refund(&mut tx, id).await;
    match result {
        Ok(escrow) => {
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(Json(escrow))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            Err(e)
        }
    }
}

async fn release_funds(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Escrow>, String> {
    let mut tx = state.db_pool.begin().await.map_err(|e| e.to_string())?;
    let result = state.escrow_service.release_funds(&mut tx, id).await;
    match result {
        Ok(escrow) => {
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(Json(escrow))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            Err(e)
        }
    }
}

async fn lock_funds(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(amount): Json<i64>,
) -> Result<Json<Escrow>, String> {
    let mut tx = state.db_pool.begin().await.map_err(|e| e.to_string())?;
    let result = state.escrow_service.lock_funds(&mut tx, id, amount).await;
    match result {
        Ok(escrow) => {
            tx.commit().await.map_err(|e| e.to_string())?;
            Ok(Json(escrow))
        }
        Err(e) => {
            let _ = tx.rollback().await;
            Err(e)
        }
    }
}
