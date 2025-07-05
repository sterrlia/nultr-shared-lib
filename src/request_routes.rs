use rust_api_kit::{define_http_routes, http::client::BearerToken};

use crate::request::{AuthenticatedUnexpectedErrorResponse, CreatePrivateRoomErrorResponse, CreatePrivateRoomRequest, CreatePrivateRoomResponse, GetMessagesErrorResponse, GetMessagesRequest, GetMessagesResponse, GetRoomsErrorResponse, GetRoomsRequest, GetRoomsResponse, GetUsersErrorResponse, GetUsersRequest, GetUsersResponse, LoginErrorResponse, LoginRequest, LoginResponse, UnexpectedErrorResponse};

define_http_routes! {
    group (
        path "api";
        auth BearerToken;
        error AuthenticatedUnexpectedErrorResponse;

        GET "get-users" GetUsersRequest => GetUsersResponse | GetUsersErrorResponse;
        GET "get-messages" GetMessagesRequest => GetMessagesResponse | GetMessagesErrorResponse;
        GET "get-rooms" GetRoomsRequest => GetRoomsResponse | GetRoomsErrorResponse;

        POST "create-private-room" CreatePrivateRoomRequest => CreatePrivateRoomResponse | CreatePrivateRoomErrorResponse;
    );

    group (
        path "api";
        error UnexpectedErrorResponse;

        POST "login" LoginRequest => LoginResponse | LoginErrorResponse;
    );
}

