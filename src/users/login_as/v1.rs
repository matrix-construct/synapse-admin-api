//! [POST /_synapse/admin/v1/users/:user_id/login](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#login-as-a-user)

use ruma::{
    MilliSecondsSinceUnixEpoch, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/login",
}

#[request]
pub struct Request {
    /// The user to obtain an access token for.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// The timestamp, in milliseconds, at which the returned token expires.
    ///
    /// When omitted the token never expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until_ms: Option<MilliSecondsSinceUnixEpoch>,
}

#[response]
pub struct Response {
    /// The access token for the user.
    pub access_token: String,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id, valid_until_ms: None }
    }
}

impl Response {
    /// Creates a `Response` with the given access token.
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}
