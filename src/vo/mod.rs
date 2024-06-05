use std::fmt::Debug;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::error::Error;

pub mod user_vo;
pub mod role_vo;
pub mod menu_vo;

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
    where T: Serialize + Debug
{
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl <T>IntoResponse for BaseResponse<T>
    where T: Serialize + Debug
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

impl<T> From<Result<T, Error>> for BaseResponse<T>
    where T: Serialize + Debug,
{
    fn from(value: Result<T, Error>) -> Self {
        match value {
            Ok(data) => Self::from(data),
            Err(e) => Self::from(e),
        }
    }
}

impl<T> From<Error> for BaseResponse<T>
    where T: Serialize + Debug,
{
    fn from(e: Error) -> Self {
        match e {
            Error::E(msg) => Self {
                code: "1".to_string(),
                msg: Some(msg),
                data: None,
            },
            Error::Code(code, msg) => Self {
                code: code.to_string(),
                msg: Some(msg),
                data: None,
            },
            _ => Self {
                code: "2".to_string(),
                msg: Some("未知错误".to_string()),
                data: None,
            }
        }
    }
}

impl<T> From<T> for BaseResponse<T>
    where T: Serialize + Debug,
{
    fn from(data: T) -> Self {
        Self {
            code: "0".to_string(),
            msg: Some("操作成功".to_string()),
            data: Some(data),
        }
    }
}


impl<T> BaseResponse<T>
    where
        T: Serialize + Debug,
{
    pub fn json(self) -> axum::Json<BaseResponse<T>> {
        axum::Json(self)
    }
}

// 处理统一返回
pub fn handle_result<T>(result: Result<T, impl Into<Error>>) -> impl IntoResponse
    where T: Serialize + Debug
{
    match result {
        Ok(d) => {
            BaseResponse {
                code: "0".to_string(),
                msg: Some("操作成功".to_string()),
                data: Some(d),
            }
        }
        Err(err) => {
            BaseResponse::from(err.into())
        }
    }

}

impl <T>IntoResponse for Response<T>
where T: Serialize+Debug,
{
    fn into_response(self) -> axum::response::Response {
        BaseResponse::from(self.0).into_response()
        
    }
}


/// 统一返回分页
#[derive(Serialize, Debug, Clone)]
pub struct ResponsePage<T>
    where T: Serialize + Debug
{
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub data: Option<T>,
}

pub fn ok_result_page<T: Serialize + Debug>(data: T, total: u64) -> ResponsePage<T> {
    ResponsePage {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(data),
        total,
    }
}

pub fn err_result_page<T: Serialize + Debug>(data: T, msg: impl ToString) -> ResponsePage<T> {
    ResponsePage {
        msg: msg.to_string(),
        code: 1,
        data: Some(data),
        total: 0,
    }
}