//! [GET /_synapse/admin/v1/rooms/:room_id/messages](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#room-messages-api)

use ruma::{
    OwnedRoomId, UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
    events::{AnyStateEvent, AnyTimelineEvent},
    serde::Raw,
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/messages",
}

#[request]
pub struct Request {
    /// ID of the room to fetch messages from.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The token to start returning events from.
    ///
    /// When omitted, the server starts from the beginning or the end of the
    /// room history, depending on `dir`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<String>,

    /// The token to stop returning events at.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub to: Option<String>,

    /// The maximum number of events to return.
    ///
    /// Defaults to 10 and is capped at 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// The direction to return events in.
    ///
    /// Defaults to forward.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,

    /// A JSON-encoded `RoomEventFilter` to filter the returned events with.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub filter: Option<String>,
}

#[derive(Default)]
#[response]
pub struct Response {
    /// The token the pagination starts from.
    pub start: String,

    /// The token the pagination ends at.
    ///
    /// Omitted when the timeline has been exhausted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// The returned events.
    #[serde(default)]
    pub chunk: Vec<Raw<AnyTimelineEvent>>,

    /// State events relevant to the returned events.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<Raw<AnyStateEvent>>,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id, from: None, to: None, limit: None, dir: None, filter: None }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Default::default()
    }
}
