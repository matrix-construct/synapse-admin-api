//! [GET /_synapse/mas/query_user](https://github.com/element-hq/synapse/blob/develop/synapse/rest/synapse/mas/users.py)

use ruma::{
    OwnedUserId,
    api::{auth_scheme::NoAuthentication, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/mas/query_user",
}

#[request]
pub struct Request {
    /// Localpart of the user to query.
    #[ruma_api(query)]
    pub localpart: String,
}

#[response]
pub struct Response {
    /// Full Matrix user ID (`@localpart:server`).
    pub user_id: OwnedUserId,

    /// Display name, if set. Serialized even when absent, mirroring Synapse.
    pub display_name: Option<String>,

    /// Avatar URL, if set. Serialized even when absent, mirroring Synapse.
    pub avatar_url: Option<String>,

    /// Whether the account is suspended.
    pub is_suspended: bool,

    /// Whether the account is deactivated.
    pub is_deactivated: bool,
}

impl Request {
    /// Creates a `Request` for the given localpart.
    pub fn new(localpart: String) -> Self {
        Self { localpart }
    }
}

impl Response {
    /// Creates a `Response` for the given user ID with empty profile fields.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self {
            user_id,
            display_name: None,
            avatar_url: None,
            is_suspended: false,
            is_deactivated: false,
        }
    }
}
