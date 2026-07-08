//! [GET /_synapse/admin/v1/rooms/:room_id/hierarchy](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#admin-space-hierarchy-endpoint)

use ruma::{
    OwnedRoomId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
    room::RoomSummary,
    serde::Raw,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/hierarchy",
}

#[request]
pub struct Request {
    /// ID of the space to fetch the hierarchy for.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// A pagination token from a previous request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<String>,

    /// The maximum number of rooms to return per page.
    ///
    /// Defaults to and is capped at 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// How far to recurse into the space.
    ///
    /// Defaults to no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub max_depth: Option<UInt>,
}

#[response]
pub struct Response {
    /// A paginated chunk of the space hierarchy.
    ///
    /// Each entry is a client-server space hierarchy room chunk: a room summary
    /// together with the room's stripped `m.space.child` events under
    /// `children_state`. It is carried as raw JSON because this crate does not
    /// depend on `ruma-client-api`.
    pub rooms: Vec<Raw<RoomSummary>>,

    /// A token to fetch the next chunk of the hierarchy.
    ///
    /// Omitted when there are no further results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_batch: Option<String>,
}

impl Request {
    /// Creates a `Request` with the given space ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id, from: None, limit: None, max_depth: None }
    }
}

impl Response {
    /// Creates a `Response` with the given hierarchy chunk.
    pub fn new(rooms: Vec<Raw<RoomSummary>>) -> Self {
        Self { rooms, next_batch: None }
    }
}
