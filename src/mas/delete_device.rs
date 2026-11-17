//! [POST /_synapse/mas/delete_device](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/devices.py)
//!
//! Deletes a device. Synapse answers `204 No Content`; the `#[response]` macro
//! always emits `200`, so `Response` hand-rolls `OutgoingResponse` to set the
//! fixed status (MAS ignores it, but parity is free here).

use ruma::{
    OwnedDeviceId,
    api::{
        EmptyBody, IncomingResponse, OutgoingResponse,
        auth_scheme::NoAuthentication,
        error::{DeserializationError, Error, IntoHttpError},
        metadata, request,
    },
    exports::http,
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/delete_device",
}

#[request]
pub struct Request {
    /// Localpart of the device's owner.
    pub localpart: String,

    /// Device to delete.
    pub device_id: OwnedDeviceId,
}

/// Empty response carrying a fixed `204 No Content` status.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Response {}

impl Request {
    /// Creates a `Request` for the given localpart and device.
    pub fn new(localpart: String, device_id: OwnedDeviceId) -> Self {
        Self { localpart, device_id }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}

impl OutgoingResponse for Response {
    type Body = EmptyBody;

    fn try_into_http_response_inner(self) -> Result<http::Response<Self::Body>, IntoHttpError> {
        http::Response::builder()
            .status(http::StatusCode::NO_CONTENT)
            .body(EmptyBody)
            .map_err(Into::into)
    }
}

impl IncomingResponse for Response {
    type EndpointError = Error;

    fn try_from_http_response_inner(
        _response: http::Response<&[u8]>,
    ) -> Result<Self, DeserializationError> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use ruma::api::{IncomingResponseExt, OutgoingResponseExt};

    use super::Response;

    #[test]
    fn response_wire() {
        let response = Response::new().try_into_http_response::<Vec<u8>>().unwrap();

        assert_eq!(response.status(), 204);
        assert!(response.body().is_empty());
        assert!(!response.headers().contains_key("content-type"));
        assert!(Response::try_from_http_response(response.map(|_| &b""[..])).is_ok());
    }
}
