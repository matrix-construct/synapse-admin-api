//! [GET /_synapse/admin/v2/rooms/:room_id/delete_status](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#status-of-deleting-rooms)

use ruma::{
    OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::rooms::delete_status::by_delete_id::{DeleteStatus, DeleteStatusKind};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/rooms/{room_id}/delete_status",
}

#[request]
pub struct Request {
    /// ID of the room whose deletion tasks to query.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response]
pub struct Response {
    /// The deletion tasks for the room.
    pub results: Vec<DeleteStatus>,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

impl Response {
    /// Creates a `Response` with the given deletion tasks.
    pub fn new(results: Vec<DeleteStatus>) -> Self {
        Self { results }
    }
}
