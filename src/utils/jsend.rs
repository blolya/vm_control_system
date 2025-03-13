use anyhow::anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize, Serializer};
use serde_json;

use crate::errors::Error;

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum JSend<T> {
    Success {
        data: Option<T>,
    },
    Fail {
        data: T,
    },
    Error {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<T>,
    },
}
impl<T> JSend<T> {
    pub fn success(data: Option<T>) -> Self {
        Self::Success { data }
    }
    pub fn fail(data: T) -> Self {
        Self::Fail { data }
    }
    pub fn error(message: String, code: Option<u16>, data: Option<T>) -> Self {
        Self::Error {
            message,
            code,
            data,
        }
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Success { data, .. } => data.as_ref(),
            Self::Fail { data, .. } => Some(data),
            Self::Error { data, .. } => data.as_ref(),
        }
    }
    pub fn code(&self) -> Option<&u16> {
        match self {
            Self::Error { code, .. } => code.as_ref(),
            _ => None,
        }
    }
    pub fn message(&self) -> Option<&String> {
        match self {
            Self::Error { message, .. } => Some(message),
            _ => None,
        }
    }
}

impl<T> IntoResponse for JSend<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status_code = match &self {
            Self::Success { .. } => StatusCode::OK,
            Self::Fail { .. } => StatusCode::BAD_REQUEST,
            Self::Error { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (
            status_code,
            serde_json::to_string(&self).map_err(|e| Error::Anyhow(anyhow!(e))),
        )
            .into_response()
    }
}

#[cfg(test)]
mod jsend {
    mod serialize {
        use super::super::JSend;
        use axum::http::StatusCode;
        use std::collections::HashMap;

        #[test]
        fn serialize_success() {
            let value = JSend::success(Some(42));
            let result = serde_json::to_string(&value).unwrap();
            let expected_result = "{\"status\":\"success\",\"data\":42}";

            assert_eq!(expected_result, result)
        }
        #[test]
        fn serialize_success_empty() {
            let value: JSend<i32> = JSend::success(None);
            let result = serde_json::to_string(&value).unwrap();
            let expected_result = "{\"status\":\"success\",\"data\":null}";

            assert_eq!(expected_result, result)
        }
        #[test]
        fn serialize_fail() {
            let mut data: HashMap<String, Vec<String>> = HashMap::with_capacity(1);
            data.insert(
                String::from("key"),
                vec![String::from("val_1"), String::from("val_2")],
            );

            let value = JSend::fail(data);
            let result = serde_json::to_string(&value).unwrap();
            let expected_result = "{\"status\":\"fail\",\"data\":{\"key\":[\"val_1\",\"val_2\"]}}";

            assert_eq!(expected_result, result)
        }
        #[test]
        fn serialize_error_data_empty() {
            let value: JSend<String> = JSend::error(
                "Internal server error".to_string(),
                Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                None,
            );
            let result = serde_json::to_string(&value).unwrap();
            let expected_result =
                "{\"status\":\"error\",\"message\":\"Internal server error\",\"code\":500}";

            assert_eq!(expected_result, result)
        }
        #[test]
        fn serialize_error_code_empty() {
            let value: JSend<String> = JSend::error(
                "Internal server error".to_string(),
                None,
                Some("data".to_string()),
            );
            let result = serde_json::to_string(&value).unwrap();
            let expected_result =
                "{\"status\":\"error\",\"message\":\"Internal server error\",\"data\":\"data\"}";

            assert_eq!(expected_result, result)
        }
    }
}
