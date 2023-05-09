use axum::extract::FromRequest;
use super::response::ApiResponse;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiResponse<String>))]
pub struct Json<T>(pub T);

