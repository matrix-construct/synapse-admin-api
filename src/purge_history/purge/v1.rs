//! [POST /_synapse/admin/v1/purge_history/:room_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/purge_history_api.md)

use ruma::{
    MilliSecondsSinceUnixEpoch, OwnedEventId, OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/purge_history/{room_id}",
}

#[request]
pub struct Request {
    /// ID of the room to purge history from.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// Whether to also delete local events.
    ///
    /// Defaults to `false`.
    #[serde(default)]
    pub delete_local_events: bool,

    /// Purge all history up to and excluding this timestamp, in milliseconds
    /// since the Unix epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purge_up_to_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// Purge all history up to and excluding this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purge_up_to_event_id: Option<OwnedEventId>,
}

#[response]
pub struct Response {
    /// The identifier of the scheduled purge task.
    pub purge_id: String,
}

impl Request {
    /// Creates a `Request` with the given room ID.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self {
            room_id,
            delete_local_events: false,
            purge_up_to_ts: None,
            purge_up_to_event_id: None,
        }
    }
}

impl Response {
    /// Creates a `Response` with the given purge ID.
    pub fn new(purge_id: String) -> Self {
        Self { purge_id }
    }
}
