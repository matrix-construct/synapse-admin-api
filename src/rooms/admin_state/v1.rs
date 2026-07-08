//! [GET /_synapse/admin/v1/rooms/:room_id/state](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#room-state-api)

use ruma::{
    OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    events::AnyStateEvent,
    serde::Raw,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/state",
}

#[request]
pub struct Request {
    /// ID of the room to fetch the state of.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// Filter the returned state by event type.
    ///
    /// Defaults to no filtering.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub event_type: Option<String>,
}

#[response]
pub struct Response {
    /// The full current state of the room.
    pub state: Vec<Raw<AnyStateEvent>>,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id, event_type: None }
    }
}

impl Response {
    /// Creates a `Response` with the given state events.
    pub fn new(state: Vec<Raw<AnyStateEvent>>) -> Self {
        Self { state }
    }
}
