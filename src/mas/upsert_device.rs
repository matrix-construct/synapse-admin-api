//! [POST /_synapse/mas/upsert_device](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/devices.py)
//!
//! Create or update a device. Synapse returns 201 on create and 200 when the
//! device already exists; the handler sets `Response::created` and the
//! hand-rolled `OutgoingResponse` maps it to the status (the `#[response]`
//! macro always emits 200). MAS ignores the status, but parity is free here.

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
    path: "/_synapse/mas/upsert_device",
}

#[request]
pub struct Request {
    /// Localpart of the device's owner.
    pub localpart: String,

    /// Device to create or update.
    pub device_id: OwnedDeviceId,

    /// Device display name, if set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// Empty body whose HTTP status reflects whether the device was created.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Response {
    /// `true` selects `201 Created`, `false` selects `200 OK`. Carried in the
    /// HTTP status, not the JSON body.
    pub created: bool,
}

impl Request {
    /// Creates a `Request` for the given localpart and device.
    pub fn new(localpart: String, device_id: OwnedDeviceId) -> Self {
        Self { localpart, device_id, display_name: None }
    }
}

impl Response {
    /// Creates a `Response`; `created` selects `201` (true) or `200` (false).
    pub fn new(created: bool) -> Self {
        Self { created }
    }
}

impl OutgoingResponse for Response {
    type Body = EmptyBody<false>;

    fn try_into_http_response_inner(self) -> Result<http::Response<Self::Body>, IntoHttpError> {
        let status = if self.created { http::StatusCode::CREATED } else { http::StatusCode::OK };

        http::Response::builder().status(status).body(EmptyBody).map_err(Into::into)
    }
}

impl IncomingResponse for Response {
    type EndpointError = Error;

    fn try_from_http_response_inner(
        response: http::Response<&[u8]>,
    ) -> Result<Self, DeserializationError> {
        Ok(Self { created: response.status() == http::StatusCode::CREATED })
    }
}

#[cfg(test)]
mod tests {
    use ruma::api::{IncomingResponseExt, OutgoingResponseExt};

    use super::Response;

    #[test]
    fn response_wire() {
        for (created, status) in [(true, 201), (false, 200)] {
            let response = Response::new(created).try_into_http_response::<Vec<u8>>().unwrap();

            assert_eq!(response.status(), status);
            assert_eq!(response.body(), b"{}");
            assert_eq!(response.headers()["content-type"], "application/json");
            let decoded = Response::try_from_http_response(response.map(|_| &b"{}"[..])).unwrap();

            assert_eq!(decoded.created, created);
        }
    }
}
