use std::borrow::Borrow;

use axum::{response::IntoResponse, Json, http::{Response, StatusCode, Result}, body::BoxBody};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct GenericResponse<T> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

pub struct ErrorResponse {
    pub message: String
}

#[derive(Debug)]
pub struct ApiResponse<T: Serialize> {
    message: String,
    body: Option<T>,
    status: Option<StatusCode>
}


impl<T: Serialize> ApiResponse<T> {
    pub fn success(message: String, body: Option<T>, status_code: Option<StatusCode>) -> ApiResponse<T> {
        Self {
            body: body,
            message,
            status: Some(status_code.unwrap_or(StatusCode::OK))
        }            
    }
    pub fn error(message: String, status_code: Option<StatusCode>) -> ApiResponse<T> {
        Self {
            body: None,
            message,
            status: Some(status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        }      
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = self.status.unwrap_or(StatusCode::OK);
        let response = Json(serde_json::json!(GenericResponse {
            data: self.body.as_ref(),
            message: self.message.to_string()
        }));
        (status, response).into_response()
    }

}