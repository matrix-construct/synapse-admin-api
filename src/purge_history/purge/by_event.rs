//! [POST /_synapse/admin/v1/purge_history/:room_id/:event_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/purge_history_api.md)

use ruma::{
    OwnedEventId, OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/purge_history/{room_id}/{event_id}",
}

#[request]
pub struct Request {
    /// ID of the room to purge history from.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The event up to and excluding which history is purged.
    #[ruma_api(path)]
    pub event_id: OwnedEventId,

    /// Whether to also delete local events.
    ///
    /// Defaults to `false`.
    #[serde(default)]
    pub delete_local_events: bool,
}

#[response]
pub struct Response {
    /// The identifier of the scheduled purge task.
    pub purge_id: String,
}

impl Request {
    /// Creates a `Request` with the given room and event IDs.
    pub fn new(room_id: OwnedRoomId, event_id: OwnedEventId) -> Self {
        Self { room_id, event_id, delete_local_events: false }
    }
}

impl Response {
    /// Creates a `Response` with the given purge ID.
    pub fn new(purge_id: String) -> Self {
        Self { purge_id }
    }
}
