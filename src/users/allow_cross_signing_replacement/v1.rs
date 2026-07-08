//! [POST /_synapse/admin/v1/users/:user_id/_allow_cross_signing_replacement_without_uia](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#allow-replacing-master-cross-signing-key-without-user-interactive-auth)

use ruma::{
    MilliSecondsSinceUnixEpoch, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/_allow_cross_signing_replacement_without_uia",
}

#[request]
pub struct Request {
    /// The user whose master cross-signing key may be replaced.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// The timestamp, in milliseconds, before which the master cross-signing key can be replaced
    /// without user-interactive auth.
    pub updatable_without_uia_before_ms: MilliSecondsSinceUnixEpoch,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given deadline timestamp.
    pub fn new(updatable_without_uia_before_ms: MilliSecondsSinceUnixEpoch) -> Self {
        Self { updatable_without_uia_before_ms }
    }
}
