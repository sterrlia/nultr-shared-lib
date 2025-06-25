use rust_api_kit::{define_http_routes, http::client::BearerToken};

use crate::request::{AuthenticatedUnexpectedErrorResponse, GetMessagesErrorResponse, GetMessagesRequest, GetMessagesResponse, GetUsersErrorResponse, GetUsersRequest, GetUsersResponse, LoginErrorResponse, LoginRequest, LoginResponse, UnexpectedErrorResponse};

define_http_routes! {
    group (
        auth BearerToken;
        error AuthenticatedUnexpectedErrorResponse;

        GET "api/get-messages" GetMessagesRequest => GetMessagesResponse | GetMessagesErrorResponse;
        GET "api/get-users" GetUsersRequest => GetUsersResponse | GetUsersErrorResponse;
    );

    group (
        error UnexpectedErrorResponse;

        POST "api/login" LoginRequest => LoginResponse | LoginErrorResponse;
    );
}

