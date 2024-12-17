
use axum::{
    async_trait, extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request}, RequestPartsExt
};
use tracing::error;
use crate::error::ApiError;

/// Custom Request Body Json Extract
pub struct RequestBody<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for RequestBody<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        // 请求 api path
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // 错误原因断
            Err(rejection) => {
                let path = path.unwrap();
                let error = match rejection {
                    JsonRejection::MissingJsonContentType(ct) => {
                        ApiError::RequestUnsupportedMediaType(ct.to_string())
                    },
                    JsonRejection::JsonDataError(e) => {
                        error!("path: {}, error: {:?}", &path, e);
                        ApiError::RequestParamError
                    }
                    JsonRejection::JsonSyntaxError(e) => {
                        error!("path: {}, error: {:?}", &path, e);
                        ApiError::RequestParamError
                    },
                    _ => ApiError::SysError,
                };
                Err(error)
            }
        }
    }
}

