//! [GET /_synapse/admin/v1/threepid/:medium/users/:address](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#find-a-user-based-on-their-third-party-id-threepid)

use ruma::{
    OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/threepid/{medium}/users/{address}",
}

#[request]
pub struct Request {
    /// The medium of the third-party identifier (e.g. `email` or `msisdn`).
    #[ruma_api(path)]
    pub medium: String,

    /// The address of the third-party identifier.
    #[ruma_api(path)]
    pub address: String,
}

#[response]
pub struct Response {
    /// The user associated with the third-party identifier.
    pub user_id: OwnedUserId,
}

impl Request {
    /// Creates a `Request` with the given medium and address.
    pub fn new(medium: String, address: String) -> Self {
        Self { medium, address }
    }
}

impl Response {
    /// Creates a `Response` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}
