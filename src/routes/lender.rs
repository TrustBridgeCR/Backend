use axum::{response::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct LenderResponse {
   message: String,
}

pub async fn handler() -> Json<LenderResponse> {
   Json(LenderResponse {
       message: "Lender access granted".to_string()
   })
}
