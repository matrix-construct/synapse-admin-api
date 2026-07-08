//! [PUT /_synapse/admin/v1/rooms/:room_id/block](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#block-or-unblock-a-room)

use ruma::{
    OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/block",
}

#[request]
pub struct Request {
    /// ID of the room to block or unblock.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// Whether to block or unblock the room.
    pub block: bool,
}

#[response]
pub struct Response {
    /// Whether the room is blocked.
    pub block: bool,
}

impl Request {
    /// Creates a `Request` with the given room ID and block flag.
    pub fn new(room_id: OwnedRoomId, block: bool) -> Self {
        Self { room_id, block }
    }
}

impl Response {
    /// Creates a `Response` with the given block status.
    pub fn new(block: bool) -> Self {
        Self { block }
    }
}
