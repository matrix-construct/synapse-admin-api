//! [GET /_synapse/admin/v1/rooms/:room_id/timestamp_to_event](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#room-timestamp-to-event-api)

use ruma::{
    MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id}/timestamp_to_event",
}

#[request]
pub struct Request {
    /// ID of the room to search in.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The timestamp to search from, in milliseconds since the Unix epoch.
    #[ruma_api(query)]
    pub ts: MilliSecondsSinceUnixEpoch,

    /// The direction in which to search.
    ///
    /// Defaults to forward.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,
}

#[response]
pub struct Response {
    /// The ID of the event found.
    pub event_id: OwnedEventId,

    /// The timestamp of the event found.
    pub origin_server_ts: MilliSecondsSinceUnixEpoch,
}

impl Request {
    /// Creates a `Request` with the given room ID and timestamp.
    pub fn new(room_id: OwnedRoomId, ts: MilliSecondsSinceUnixEpoch) -> Self {
        Self { room_id, ts, dir: None }
    }
}

impl Response {
    /// Creates a `Response` with the given event ID and timestamp.
    pub fn new(event_id: OwnedEventId, origin_server_ts: MilliSecondsSinceUnixEpoch) -> Self {
        Self { event_id, origin_server_ts }
    }
}
