//! [POST /_synapse/admin/v1/deactivate/:user_id](https://github.com/element-hq/synapse/blob/develop/docs/admin_api/user_admin_api.md#deactivate-account)

use ruma::{
    OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/deactivate/{user_id}",
}

#[request]
pub struct Request {
    /// User ID
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// Flag whether to erase the account.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    pub erase: bool,
}

#[response]
pub struct Response {
    /// Result of unbinding the user's third-party IDs from the identity server.
    ///
    /// Typically `"success"` or `"no-support"`.
    pub id_server_unbind_result: String,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id, erase: false }
    }
}

impl Response {
    /// Creates a `Response` with the given identity-server unbind result.
    pub fn new(id_server_unbind_result: String) -> Self {
        Self { id_server_unbind_result }
    }
}
