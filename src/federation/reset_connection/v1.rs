//! [POST /_synapse/admin/v1/federation/destinations/:destination/reset_connection](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/federation.md#reset-connection-timeout)
use ruma::{
    OwnedServerName,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/federation/destinations/{destination}/reset_connection",
}

#[request]
pub struct Request {
    /// The remote server whose connection timeout to reset.
    #[ruma_api(path)]
    pub destination: OwnedServerName,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given destination.
    pub fn new(destination: OwnedServerName) -> Self {
        Self { destination }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
