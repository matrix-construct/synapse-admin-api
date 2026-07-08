//! [PUT /_synapse/admin/v1/registration_tokens/:token](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/registration_tokens.md#update-token)

use ruma::{
    JsOption, MilliSecondsSinceUnixEpoch, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::registration_tokens::RegistrationToken;

metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/registration_tokens/{token}",
}

#[request]
pub struct Request {
    /// The token to update.
    #[ruma_api(path)]
    pub token: String,

    /// The new number of times the token can be used.
    ///
    /// Omit to leave the cap unchanged; an explicit `null` clears it so the
    /// token can be used an unlimited number of times.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uses_allowed: JsOption<UInt>,

    /// The new point in time at which the token expires.
    ///
    /// Omit to leave the expiry unchanged; an explicit `null` clears it so the
    /// token never expires.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub expiry_time: JsOption<MilliSecondsSinceUnixEpoch>,
}

#[response]
pub struct Response {
    /// The updated registration token.
    #[ruma_api(body)]
    pub token: RegistrationToken,
}

impl Request {
    /// Creates a `Request` with the given token.
    pub fn new(token: String) -> Self {
        Self { token, uses_allowed: JsOption::Undefined, expiry_time: JsOption::Undefined }
    }
}

impl Response {
    /// Creates a `Response` with the given registration token.
    pub fn new(token: RegistrationToken) -> Self {
        Self { token }
    }
}
