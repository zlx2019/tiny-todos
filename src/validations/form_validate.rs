use axum::{async_trait, extract::{rejection::FormRejection, FromRequest, Request}, Form};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::ApiError;




#[derive(Debug, Clone, Copy, Default)]
pub struct ValidateForm<T> (pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidateForm<T>
where 
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection>{
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidateForm(value))
    }
}

