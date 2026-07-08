//! [POST /_synapse/admin/v1/registration_tokens/new](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/registration_tokens.md#create-token)

use ruma::{
    MilliSecondsSinceUnixEpoch, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::registration_tokens::RegistrationToken;

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/registration_tokens/new",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// The token string to create.
    ///
    /// A random one is generated when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// The length of the token to generate.
    ///
    /// Used only when `token` is omitted; defaults to 16.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<UInt>,

    /// The number of times the token can be used.
    ///
    /// Omitting it, or setting it to `None`, allows unlimited uses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses_allowed: Option<UInt>,

    /// The point in time at which the token expires.
    ///
    /// Omitting it, or setting it to `None`, means the token never expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<MilliSecondsSinceUnixEpoch>,
}

#[response]
pub struct Response {
    /// The created registration token.
    #[ruma_api(body)]
    pub token: RegistrationToken,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given registration token.
    pub fn new(token: RegistrationToken) -> Self {
        Self { token }
    }
}
