//! [POST /_synapse/mas/allow_cross_signing_reset](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/users.py)

use ruma::api::{auth_scheme::NoAuthentication, metadata, request, response};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/allow_cross_signing_reset",
}

#[request]
pub struct Request {
    /// Localpart of the user permitted to reset cross-signing without UIA.
    pub localpart: String,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` for the given localpart.
    pub fn new(localpart: String) -> Self {
        Self { localpart }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
