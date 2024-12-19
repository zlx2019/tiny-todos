use axum::{async_trait, extract::{rejection::JsonRejection, FromRequest, Request}};

use crate::error::ApiError;

/// Request Body Json Extractor
pub struct RequestBody<T>(pub T);


#[async_trait]
impl<S, T> FromRequest<S> for RequestBody<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let req = Request::from_parts(parts, body);
        axum::Json::<T>::from_request(req, state)
            .await
        .map(|value| Self(value.0))
        .map_err(|rejection| ApiError::RequestBodyError(rejection.body_text()))
    }
}


