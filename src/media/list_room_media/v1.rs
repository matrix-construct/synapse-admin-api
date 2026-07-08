//! [GET /_synapse/admin/v1/room/:room_id/media](https://github.com/element-hq/synapse/blob/master/docs/admin_api/media_admin_api.md#list-all-media-in-a-room)

use ruma::{
    OwnedMxcUri, OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/room/{room_id}/media",
}

#[request]
pub struct Request {
    /// The room whose media to list.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response]
pub struct Response {
    /// The MXC URIs of local media in the room.
    pub local: Vec<OwnedMxcUri>,

    /// The MXC URIs of remote media in the room.
    pub remote: Vec<OwnedMxcUri>,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

impl Response {
    /// Creates a `Response` with the given local and remote media URIs.
    pub fn new(local: Vec<OwnedMxcUri>, remote: Vec<OwnedMxcUri>) -> Self {
        Self { local, remote }
    }
}
