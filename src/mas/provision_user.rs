//! [POST /_synapse/mas/provision_user](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/users.py)
//!
//! Create or update a user. Synapse returns 201 on create and 200 on update;
//! MAS reads that status, so the handler sets `Response::created` and the
//! hand-rolled `OutgoingResponse` maps it to the status (the `#[response]`
//! macro always emits 200). The `set_*`/`unset_*` pairs are mutually exclusive,
//! a handler-side check.

use ruma::{
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
    path: "/_synapse/mas/provision_user",
}

#[request]
pub struct Request {
    /// Localpart of the user to create or update.
    pub localpart: String,

    /// Set the display name. Mutually exclusive with `unset_displayname`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_displayname: Option<String>,

    /// Clear the display name. Mutually exclusive with `set_displayname`.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    pub unset_displayname: bool,

    /// Set the avatar URL. Mutually exclusive with `unset_avatar_url`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_avatar_url: Option<String>,

    /// Clear the avatar URL. Mutually exclusive with `set_avatar_url`.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    pub unset_avatar_url: bool,

    /// Replace the user's email 3PIDs. Mutually exclusive with `unset_emails`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_emails: Option<Vec<String>>,

    /// Clear the user's email 3PIDs. Mutually exclusive with `set_emails`.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    pub unset_emails: bool,

    /// Lock (`true`) or unlock (`false`) the account. `None` leaves it
    /// unchanged. MAS always sends this field; the others it omits when unset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// Empty body whose HTTP status reflects whether the user was created.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Response {
    /// `true` selects `201 Created`, `false` selects `200 OK`. Carried in the
    /// HTTP status, not the JSON body.
    pub created: bool,
}

impl Request {
    /// Creates a `Request` for the given localpart with every change unset.
    pub fn new(localpart: String) -> Self {
        Self {
            localpart,
            set_displayname: None,
            unset_displayname: false,
            set_avatar_url: None,
            unset_avatar_url: false,
            set_emails: None,
            unset_emails: false,
            locked: None,
        }
    }
}

impl Response {
    /// Creates a `Response`; `created` selects `201` (true) or `200` (false).
    pub fn new(created: bool) -> Self {
        Self { created }
    }
}

impl OutgoingResponse for Response {
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError> {
        let status = if self.created { http::StatusCode::CREATED } else { http::StatusCode::OK };

        let mut body = T::default();
        body.put_slice(b"{}");
        http::Response::builder()
            .status(status)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(body)
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

        Ok(Self { created: response.status() == http::StatusCode::CREATED })
    }
}
