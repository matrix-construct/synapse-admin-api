//! [GET /_synapse/admin/v1/rooms/:room_id/block](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#get-block-status)

use ruma::{
    OwnedRoomId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/block",
}

#[request]
pub struct Request {
    /// ID of the room to query the block status of.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response]
pub struct Response {
    /// Whether the room is blocked.
    pub block: bool,

    /// The user that blocked the room.
    ///
    /// Present only when the room is blocked.
    #[serde(
        default,
        deserialize_with = "ruma::serde::empty_string_as_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_id: Option<OwnedUserId>,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

impl Response {
    /// Creates a `Response` with the given block status.
    pub fn new(block: bool) -> Self {
        Self { block, user_id: None }
    }
}
