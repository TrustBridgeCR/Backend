use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use validator::Validate;

#[async_trait]
impl<B, T> FromRequest<B> for T
where
    T: Validate + serde::de::DeserializeOwned,
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid JSON".to_string()))?;

        value.validate().map_err(|e| {
            (StatusCode::BAD_REQUEST, e.to_string())
        })?;

        Ok(value)
    }
}