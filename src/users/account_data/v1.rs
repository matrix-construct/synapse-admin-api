//! [GET /_synapse/admin/v1/users/:user_id/accountdata](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#account-data)

use std::collections::BTreeMap;

use ruma::{
    OwnedRoomId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    events::{AnyGlobalAccountDataEventContent, AnyRoomAccountDataEventContent},
    serde::Raw,
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/accountdata",
}

#[request]
pub struct Request {
    /// The user to fetch the account data of.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// The global and per-room account data of the user.
    pub account_data: AccountData,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given account data.
    pub fn new(account_data: AccountData) -> Self {
        Self { account_data }
    }
}

/// The account data associated with a user.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct AccountData {
    /// Global account data, keyed by event type.
    #[serde(default)]
    pub global: BTreeMap<String, Raw<AnyGlobalAccountDataEventContent>>,

    /// Per-room account data, keyed by room ID then by event type.
    #[serde(default)]
    pub rooms: BTreeMap<OwnedRoomId, BTreeMap<String, Raw<AnyRoomAccountDataEventContent>>>,
}

impl AccountData {
    /// Creates an empty `AccountData`.
    pub fn new() -> Self {
        Default::default()
    }
}
