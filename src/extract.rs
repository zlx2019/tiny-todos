// use crate::error::ApiError;
// use axum::{
//     async_trait,
//     extract::{
//         rejection::JsonRejection,
//         FromRequest, FromRequestParts, MatchedPath, Path, Request,
//     },
//     http::request::Parts,
//     RequestPartsExt,
// };
// use serde::de::DeserializeOwned;

// /// Request Body Json Extractor
// pub struct RequestBody<T>(pub T);

// #[async_trait]
// impl<S, T> FromRequest<S> for RequestBody<T>
// where
//     axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
//     S: Send + Sync,
// {
//     type Rejection = ApiError;
//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let (mut parts, body) = req.into_parts();
//         let _path = parts
//             .extract::<MatchedPath>()
//             .await
//             .map(|path| path.as_str().to_owned())
//             .ok();
//         let req = Request::from_parts(parts, body);
//         match axum::Json::<T>::from_request(req, state).await {
//             Ok(value) => Ok(Self(value.0)),
//             Err(rejection) => {
//                 // Body 反序列化失败
//                 let message = rejection.body_text();
//                 Err(ApiError::RequestBodyError(message))
//             } //     let error = match rejection {
//               //         JsonRejection::MissingJsonContentType(ct) => {
//               //             ApiError::RequestUnsupportedMediaType(ct.to_string())
//               //         },
//               //         // Body 反序列化错误
//               //         JsonRejection::JsonDataError(e) => {
//               //             let message =  format!("Invalid request body: {}", e);
//               //             error!(message);
//               //             ApiError::RequestBodyError(message)
//               //         }
//               //         // Body Json 语法错误
//               //         JsonRejection::JsonSyntaxError(e) => {
//               //             let message: String =  JsonSyntaxError::body_text(&e);
//               //             ApiError::RequestBodyJsonSyntax(message)
//               //             // if let Some(source) = e.source().and_then(|s| s.downcast_ref::<serde_json::Error>()){
//               //             //     // match source.classify() {
//               //             //     //     serde_json::error::Category::Io => todo!(),
//               //             //     //     serde_json::error::Category::Syntax => todo!(),
//               //             //     //     serde_json::error::Category::Data => todo!(),
//               //             //     //     serde_json::error::Category::Eof => todo!(),
//               //             //     // }

//               //             //     ApiError::RequestBodyJsonSyntaxDetails { message: source.to_string(), location: JsonParseLocation{
//               //             //         line: source.line(),
//               //             //         column: source.column(),
//               //             //         message: format!("{}", source)
//               //             //     }}

//               //             // } else {
//               //             //     ApiError::RequestBodyJsonSyntax
//               //             // }
//               //         },
//               //         // 其他错误
//               //         _ => ApiError::SysError,
//               //     };
//               //     Err(error)
//               // }
//         }
//     }
// }

// // Request Path Variable Extractor
// // 从请求 URI 中提取请求参数
// pub struct RequestPath<T>(pub T);

// #[async_trait]
// impl<S, T> FromRequestParts<S> for RequestPath<T>
// where
//     T: DeserializeOwned + Send,
//     S: Send + Sync,
// {
//     type Rejection = ApiError;

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         match Path::<T>::from_request_parts(parts, state).await {
//             Ok(path) => Ok(Self(path.0)),
//             Err(reject) => {
//                 let message = reject.body_text();
//                 Err(ApiError::RequestBodyError(message))
//                 // let (status, body) = match reject {
//                 //     PathRejection::FailedToDeserializePathParams(inner) => {
//                 //         // Path 参数反序列化失败
//                 //     },
//                 //     PathRejection::MissingPathParams(err) => {
//                 //         // Path 参数缺少
//                 //         (StatusCode::INTERNAL_SERVER_ERROR, e)
//                 //     },
//                 //     _ => {
//                 //         (StatusCode::INTERNAL_SERVER_ERROR, PathError{message: format!("Unhandled path rejection: {reject}"), location: None })
//                 //     }
//                 // };
//                 // Err((status, Json(body)))
//             }
//         }
//     }
// }
