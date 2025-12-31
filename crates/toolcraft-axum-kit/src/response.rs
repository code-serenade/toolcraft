use core::str;

use axum::{Json, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Default, Clone)]
pub struct Empty;

pub type CommonOk = CommonResponse<Empty>;
pub type ApiError = (StatusCode, Json<CommonError>);

pub trait IntoCommonResponse<T>
where
    T: Serialize + ToSchema,
{
    fn into_common_response(self) -> CommonResponse<T>;
}

impl<T> IntoCommonResponse<T> for T
where
    T: Serialize + ToSchema,
{
    fn into_common_response(self) -> CommonResponse<T> {
        CommonResponse {
            code: 0,
            data: self,
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommonResponse<T>
where
    T: Serialize + ToSchema,
{
    pub code: i16,
    pub data: T,
    pub message: String,
}

impl<T> CommonResponse<T>
where
    T: Serialize + ToSchema,
{
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl<T> Default for CommonResponse<T>
where
    T: Serialize + ToSchema + Default,
{
    fn default() -> Self {
        CommonResponse {
            code: 0,
            data: T::default(),
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CommonError {
    pub code: i16,
    pub message: String,
}

impl CommonError {
    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl From<(i16, &str)> for CommonError {
    fn from(value: (i16, &str)) -> Self {
        CommonError {
            code: value.0,
            message: value.1.to_string(),
        }
    }
}

pub type ResponseResult<T> = core::result::Result<Json<CommonResponse<T>>, ApiError>;
