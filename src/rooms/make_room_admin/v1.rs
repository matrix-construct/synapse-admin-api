//! [POST /_synapse/admin/v1/rooms/:room_id_or_alias/make_room_admin](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#make-room-admin-api)

use ruma::{
    OwnedRoomOrAliasId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id_or_alias}/make_room_admin",
}

#[request]
pub struct Request {
    /// Alias or ID of the room to grant admin powers in.
    #[ruma_api(path)]
    pub room_id_or_alias: OwnedRoomOrAliasId,

    /// The user to grant room admin powers to.
    ///
    /// Defaults to the requesting admin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<OwnedUserId>,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given room or alias ID.
    pub fn new(room_id_or_alias: OwnedRoomOrAliasId) -> Self {
        Self { room_id_or_alias, user_id: None }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
