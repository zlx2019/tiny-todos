use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, Request}};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::ApiError;

/// Request Body Json Extractor
pub struct RequestBody<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for RequestBody<T>
where
    T: DeserializeOwned,
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // let Json(value) = axum::Json::<T>::from_request(req, state).await?;
        // Ok(Self(value))
        Ok(Self(axum::Json::<T>::from_request(req, state).await?.0))
    }
}


/// Request Body JSON Extractor & Validater
pub struct ValidateRequestBody<T> (pub T);
#[async_trait]
impl <T, S> FromRequest<S> for ValidateRequestBody<T>
where 
    T: DeserializeOwned + Validate,
    S: Send + Sync
{
    type Rejection = ApiError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection>{
        // let Json(value) = axum::Json::<T>::from_request(req, state).await?;
        let value = axum::Json::<T>::from_request(req, state).await?.0;
        value.validate()?;
        Ok(Self(value))
    }
}