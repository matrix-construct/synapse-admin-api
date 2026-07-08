//! [GET /_synapse/admin/v1/registration_tokens](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/registration_tokens.md#list-all-tokens)

use ruma::api::{auth_scheme::AccessToken, metadata, request, response};

pub use crate::registration_tokens::RegistrationToken;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/registration_tokens",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// Filter tokens by validity.
    ///
    /// Omitting it returns all tokens, `true` returns only valid tokens, and
    /// `false` returns only expired or exhausted tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub valid: Option<bool>,
}

#[response]
pub struct Response {
    /// The list of registration tokens.
    pub registration_tokens: Vec<RegistrationToken>,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given registration tokens.
    pub fn new(registration_tokens: Vec<RegistrationToken>) -> Self {
        Self { registration_tokens }
    }
}
