//! [DELETE /_synapse/admin/v2/rooms/:room_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#version-2-new-version)

use ruma::{
    OwnedRoomId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/rooms/{room_id}",
}

#[request]
pub struct Request {
    /// ID of the room to delete.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// If set, a local user to make the owner of a new room created as a
    /// replacement for the deleted one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_room_user_id: Option<OwnedUserId>,

    /// The name of the replacement room.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,

    /// A message to send to the users kicked from the room.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Whether to block the room, preventing it from being joined again.
    ///
    /// Defaults to `false`.
    #[serde(default)]
    pub block: bool,

    /// Whether to remove all traces of the room from the database.
    ///
    /// Defaults to `true`.
    #[serde(default = "ruma::serde::default_true")]
    pub purge: bool,

    /// Whether to force the purge even if local users are still joined.
    ///
    /// Only has an effect when `purge` is `true`. Defaults to `false`.
    #[serde(default)]
    pub force_purge: bool,
}

#[response]
pub struct Response {
    /// The identifier of the scheduled deletion task.
    pub delete_id: String,
}

impl Request {
    /// Creates a `Request` with the given room ID and the default flags.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self {
            room_id,
            new_room_user_id: None,
            room_name: None,
            message: None,
            block: false,
            purge: true,
            force_purge: false,
        }
    }
}

impl Response {
    /// Creates a `Response` with the given delete ID.
    pub fn new(delete_id: String) -> Self {
        Self { delete_id }
    }
}
