//! [GET /_synapse/admin/v1/users/:user_id/memberships](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-room-memberships-of-a-user)

use std::collections::BTreeMap;

use ruma::{
    OwnedRoomId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    events::room::member::MembershipState,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/memberships",
}

#[request]
pub struct Request {
    /// The user to list the room memberships of.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// Map of room ID to the user's membership state in that room.
    pub memberships: BTreeMap<OwnedRoomId, MembershipState>,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given membership map.
    pub fn new(memberships: BTreeMap<OwnedRoomId, MembershipState>) -> Self {
        Self { memberships }
    }
}
