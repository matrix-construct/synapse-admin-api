//! [POST /_synapse/mas/delete_device](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/devices.py)
//!
//! Deletes a device. Synapse answers `204 No Content`; the `#[response]` macro
//! always emits `200`, so `Response` hand-rolls `OutgoingResponse` to set the
//! fixed status (MAS ignores it, but parity is free here).

use ruma::{
    OwnedDeviceId,
    api::{
        EndpointError, IncomingResponse, OutgoingResponse,
        auth_scheme::NoAuthentication,
        error::{Error, FromHttpResponseError, IntoHttpError},
        metadata, request,
    },
    exports::{bytes::BufMut, http},
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
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError> {
        http::Response::builder()
            .status(http::StatusCode::NO_CONTENT)
            .body(T::default())
            .map_err(Into::into)
    }
}

impl IncomingResponse for Response {
    type EndpointError = Error;

    fn try_from_http_response<T: AsRef<[u8]>>(
        response: http::Response<T>,
    ) -> Result<Self, FromHttpResponseError<Self::EndpointError>> {
        if response.status().as_u16() >= 400 {
            return Err(FromHttpResponseError::Server(Error::from_http_response(response)));
        }

        Ok(Self {})
    }
}
