//! [POST /_synapse/admin/v1/join/:room_id_or_alias](https://github.com/element-hq/synapse/blob/master/docs/admin_api/room_membership.md)

use ruma::{
    OwnedRoomId, OwnedRoomOrAliasId, OwnedServerName, OwnedUserId,
    api::{auth_scheme::AccessToken, request, response},
    metadata,
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/join/{room_id_or_alias}",
}

#[request]
pub struct Request {
    /// Alias or ID of the room to join.
    #[ruma_api(path)]
    pub room_id_or_alias: OwnedRoomOrAliasId,

    /// The servers to attempt to join the room through.
    ///
    /// Used as remote-join candidates when the room is not known locally.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[ruma_api(query)]
    pub server_name: Vec<OwnedServerName>,

    /// User to join the room.
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// Room ID of the joined room.
    pub room_id: OwnedRoomId,
}

impl Request {
    /// Creates a new `Request` with the given room or alias ID and user id.
    pub fn new(room_id_or_alias: OwnedRoomOrAliasId, user_id: OwnedUserId) -> Self {
        Self { room_id_or_alias, server_name: Vec::new(), user_id }
    }
}

impl Response {
    /// Creates a new `Response` with the given room id
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}
