//! [GET /_synapse/mas/is_localpart_available](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/users.py)
//!
//! Availability is conveyed by the status and errcode, not the body: 200 when
//! available; otherwise 400 with one of `M_INVALID_USERNAME`, `M_USER_IN_USE`,
//! or `M_EXCLUSIVE`, which MAS maps to "unavailable". There is no 404 path.

use ruma::api::{auth_scheme::NoAuthentication, metadata, request, response};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/is_localpart_available",
}

#[request]
pub struct Request {
    /// Localpart whose availability to check.
    #[ruma_api(query)]
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
