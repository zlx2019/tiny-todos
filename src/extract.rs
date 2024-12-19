
use axum::{
    async_trait, extract::{path::ErrorKind, rejection::{JsonRejection, PathRejection}, FromRequest, FromRequestParts, MatchedPath, Request}, http::{request::Parts, StatusCode}, RequestPartsExt
};
use serde::{de::DeserializeOwned, Serialize};
use tracing::error;
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

pub struct RequestPath<T> (pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for RequestPath<T>
where 
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection>{
        todo!()
    }
}


// We define our own `Path` extractor that customizes the error from `axum::extract::Path`
struct Path<T>(T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<PathError>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let mut status = StatusCode::BAD_REQUEST;

                        let kind = inner.into_kind();
                        let body = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => PathError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::ParseErrorAtKey { key, .. } => PathError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::ParseErrorAtIndex { index, .. } => PathError {
                                message: kind.to_string(),
                                location: Some(index.to_string()),
                            },

                            ErrorKind::ParseError { .. } => PathError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::InvalidUtf8InPathParam { key } => PathError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::UnsupportedType { .. } => {
                                // this error is caused by the programmer using an unsupported type
                                // (such as nested maps) so respond with `500` instead
                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                PathError {
                                    message: kind.to_string(),
                                    location: None,
                                }
                            }

                            ErrorKind::Message(msg) => PathError {
                                message: msg.clone(),
                                location: None,
                            },

                            _ => PathError {
                                message: format!("Unhandled deserialization error: {kind}"),
                                location: None,
                            },
                        };

                        (status, body)
                    }
                    PathRejection::MissingPathParams(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        PathError {
                            message: error.to_string(),
                            location: None,
                        },
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        PathError {
                            message: format!("Unhandled path rejection: {rejection}"),
                            location: None,
                        },
                    ),
                };

                Err((status, axum::Json(body)))
            }
        }
    }
}


#[derive(Serialize)]
struct PathError {
    message: String,
    location: Option<String>,
}