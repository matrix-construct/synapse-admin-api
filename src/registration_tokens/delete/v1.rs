//! [DELETE /_synapse/admin/v1/registration_tokens/:token](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/registration_tokens.md#delete-token)

use ruma::api::{auth_scheme::AccessToken, metadata, request, response};

metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/registration_tokens/{token}",
}

#[request]
pub struct Request {
    /// The token to delete.
    #[ruma_api(path)]
    pub token: String,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given token.
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
