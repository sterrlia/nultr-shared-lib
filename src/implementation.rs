use crate::request::{AuthenticatedUnexpectedErrorResponse, UnexpectedErrorResponse, WsErrorResponse, WsOkResponse, WsResponse};

impl From<anyhow::Error> for WsErrorResponse {
    fn from(value: anyhow::Error) -> Self {
        tracing::error!("Fatal error {:?}", value);

        WsErrorResponse::Fatal
    }
}

impl From<Result<WsOkResponse, WsErrorResponse>> for WsResponse {
    fn from(value: Result<WsOkResponse, WsErrorResponse>) -> Self {
        match value {
            Ok(ok) => WsResponse::Ok(ok),
            Err(err) => WsResponse::Err(err),
        }
    }
}

impl From<anyhow::Error> for UnexpectedErrorResponse {
    fn from(_: anyhow::Error) -> Self {
        UnexpectedErrorResponse::InternalServerError
    }
}

impl From<anyhow::Error> for AuthenticatedUnexpectedErrorResponse {
    fn from(_: anyhow::Error) -> Self {
        AuthenticatedUnexpectedErrorResponse::InternalServerError
    }
}
