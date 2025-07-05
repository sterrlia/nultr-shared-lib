use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub type Identifier = i32;
pub type UuidIdentifier = uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetUsersRequest {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub id: Identifier,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetUsersResponse(pub Vec<UserResponse>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GetUsersErrorResponse {}

#[derive(Serialize, Deserialize)]
pub struct GetMessagesRequest {
    pub room_id: Identifier,
    // :TODO: serde flatten does not work
    pub page: u64,
    pub page_size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageResponse {
    pub uuid: UuidIdentifier,
    pub user_id: Identifier,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub read: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreatePrivateRoomRequest {
    pub receiver_user_id: Identifier,
    pub name: Option<String>,
}

pub type CreatePrivateRoomResponse = RoomResponse;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CreatePrivateRoomErrorResponse {
    UserNotFound,
}

#[derive(Serialize, Deserialize)]
pub struct GetRoomsRequest {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RoomResponse {
    pub id: Identifier,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRoomsResponse(pub Vec<RoomResponse>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GetRoomsErrorResponse {
    UserNotFound,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetMessagesResponse(pub Vec<MessageResponse>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GetMessagesErrorResponse {
    RoomNotFound,
    NotMemberOfRoom,
    AccessDenied,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub user_id: Identifier,
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
    MessagesRead(WsMarkMessagesReadRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsMarkMessagesReadRequest {
    pub room_id: Identifier,
    pub message_uuids: Vec<UuidIdentifier>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WsMessageResponse {
    pub uuid: UuidIdentifier,
    pub user_id: Identifier,
    pub content: String,
    pub created_at: NaiveDateTime,
}

pub type WsMessagesReadResponse = WsMarkMessagesReadRequest;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsMessageRequest {
    pub uuid: UuidIdentifier,
    pub room_id: Identifier,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum WsOkResponse {
    Message(WsMessageResponse),
    MessagesRead(WsMessagesReadResponse),
    MessageReceived(UuidIdentifier),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WsErrorResponse {
    WrongFormat,
    WrongJsonFormat,
    UserNotFound,
    NotMemberOfRoom,
    MessageNotFound(UuidIdentifier),
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
    pub user_id: Identifier,
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
