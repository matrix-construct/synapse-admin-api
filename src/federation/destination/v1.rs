//! [GET /_synapse/admin/v1/federation/destinations/:destination](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/federation.md#destination-details-api)
use ruma::{
    OwnedServerName,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::federation::Destination;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/federation/destinations/{destination}",
}

#[request]
pub struct Request {
    /// The remote server to show the details of.
    #[ruma_api(path)]
    pub destination: OwnedServerName,
}

#[response]
pub struct Response {
    /// Details about the destination.
    #[ruma_api(body)]
    pub destination: Destination,
}

impl Request {
    /// Creates a `Request` with the given destination.
    pub fn new(destination: OwnedServerName) -> Self {
        Self { destination }
    }
}

impl Response {
    /// Creates a `Response` with the given destination details.
    pub fn new(destination: Destination) -> Self {
        Self { destination }
    }
}
