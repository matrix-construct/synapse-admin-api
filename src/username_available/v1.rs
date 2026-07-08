//! [GET /_synapse/admin/v1/username_available](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md)

use ruma::api::{auth_scheme::AccessToken, metadata, request, response};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/username_available",
}

#[request]
pub struct Request {
    /// The localpart to check the availability of.
    #[ruma_api(query)]
    pub username: String,
}

#[response]
pub struct Response {
    /// Whether the username is available.
    pub available: bool,
}

impl Request {
    /// Creates a `Request` with the given username.
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

impl Response {
    /// Creates a `Response` with the given availability.
    pub fn new(available: bool) -> Self {
        Self { available }
    }
}
