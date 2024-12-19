use axum::{async_trait, extract::{FromRequestParts, Path}, http::request::Parts};
use serde::de::DeserializeOwned;

use crate::error::ApiError;


/// Request Path Variable Extractor
pub struct RequestPath<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for RequestPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<T>::from_request_parts(parts, state)
            .await
            .map(|value| Self(value.0))
            .map_err(|rejection| ApiError::RequestParamError(rejection.body_text()))
    }
}