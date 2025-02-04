use axum::{response::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct BorrowerResponse {
   message: String,
}

pub async fn handler() -> Json<BorrowerResponse> {
   Json(BorrowerResponse {
       message: "Borrower access granted".to_string()
   })
}