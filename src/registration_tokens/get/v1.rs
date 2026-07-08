//! [GET /_synapse/admin/v1/registration_tokens/:token](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/registration_tokens.md#get-one-token)

use ruma::api::{auth_scheme::AccessToken, metadata, request, response};

pub use crate::registration_tokens::RegistrationToken;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/registration_tokens/{token}",
}

#[request]
pub struct Request {
    /// The token to show.
    #[ruma_api(path)]
    pub token: String,
}

#[response]
pub struct Response {
    /// The requested registration token.
    #[ruma_api(body)]
    pub token: RegistrationToken,
}

impl Request {
    /// Creates a `Request` with the given token.
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Response {
    /// Creates a `Response` with the given registration token.
    pub fn new(token: RegistrationToken) -> Self {
        Self { token }
    }
}
