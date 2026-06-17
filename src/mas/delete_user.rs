//! [POST /_synapse/mas/delete_user](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/users.py)

use ruma::api::{auth_scheme::NoAuthentication, metadata, request, response};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/delete_user",
}

#[request]
pub struct Request {
    /// Localpart of the user to deactivate.
    pub localpart: String,

    /// Whether to also erase the user's data.
    pub erase: bool,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` for the given localpart and erase flag.
    pub fn new(localpart: String, erase: bool) -> Self {
        Self { localpart, erase }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
