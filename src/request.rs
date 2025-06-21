use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetUsersRequest {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersResponse(pub Vec<UserResponse>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GetUsersErrorResponse {}

#[derive(Serialize, Deserialize)]
pub struct GetMessagesRequest {
    pub user_id: i32,
    // :TODO: serde flatten does not work
    pub page: u64,
    pub page_size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageResponse {
    pub id: Uuid,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetMessagesResponse(pub Vec<MessageResponse>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GetMessagesErrorResponse {
    UserNotFound,
    AccessDenied,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub user_id: i32,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LoginErrorResponse {
    AccessDenied,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum WsRequest {
    Message(WsMessageRequest),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WsMessageResponse {
    pub id: Uuid,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsMessageRequest {
    pub id: Uuid,
    pub user_id: i32,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WsOkResponse {
    Message(WsMessageResponse),
    MessageSent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WsErrorResponse {
    WrongFormat,
    WrongJsonFormat,
    UserNotFound,
    Fatal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum WsResponse {
    Ok(WsOkResponse),
    Err(WsErrorResponse),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum AuthError {
    InvalidToken,
}

pub type AuthToken = String;

#[derive(Debug, Clone)]
pub struct AuthUserData {
    pub user_id: i32,
    pub token: AuthToken,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum AuthenticatedUnexpectedErrorResponse {
    InternalServerError,
    InvalidToken,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum UnexpectedErrorResponse {
    InternalServerError,
}

