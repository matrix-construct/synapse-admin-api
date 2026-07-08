//! [GET /_synapse/admin/v1/rooms/:room_id/context/:event_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#event-context-api)

use ruma::{
    OwnedEventId, OwnedRoomId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
    events::{AnyStateEvent, AnyTimelineEvent},
    serde::Raw,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/context/{event_id}",
}

#[request]
pub struct Request {
    /// ID of the room the event is in.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The event to get context around.
    #[ruma_api(path)]
    pub event_id: OwnedEventId,

    /// The maximum number of context events to return.
    ///
    /// This applies to the sum of the `events_before` and `events_after`
    /// arrays. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// A JSON-encoded `RoomEventFilter` to filter the returned events with.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub filter: Option<String>,
}

#[derive(Default)]
#[response]
pub struct Response {
    /// A token that can be used to paginate backwards.
    pub start: String,

    /// A token that can be used to paginate forwards.
    pub end: String,

    /// The events preceding the requested event, in reverse-chronological
    /// order.
    #[serde(default)]
    pub events_before: Vec<Raw<AnyTimelineEvent>>,

    /// The requested event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Raw<AnyTimelineEvent>>,

    /// The events following the requested event, in chronological order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events_after: Vec<Raw<AnyTimelineEvent>>,

    /// The state of the room at the last event returned.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<Raw<AnyStateEvent>>,
}

impl Request {
    /// Creates a `Request` with the given room and event IDs.
    pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId) -> Self {
        Self { room_id, event_id, limit: None, filter: None }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Default::default()
    }
}
