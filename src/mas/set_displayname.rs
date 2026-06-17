//! [POST /_synapse/mas/set_displayname](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/users.py)

use ruma::api::{auth_scheme::NoAuthentication, metadata, request, response};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/set_displayname",
}

#[request]
pub struct Request {
    /// Localpart of the user whose display name to set.
    pub localpart: String,

    /// New display name.
    pub displayname: String,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` for the given localpart and display name.
    pub fn new(localpart: String, displayname: String) -> Self {
        Self { localpart, displayname }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
