//! [GET /_synapse/admin/v2/rooms/delete_status/:delete_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#status-of-deleting-rooms)

use ruma::{
    OwnedRoomId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    serde::StringEnum,
};
use serde::{Deserialize, Serialize};

use crate::rooms::delete_room::v1::ShutdownRoom;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/rooms/delete_status/{delete_id}",
}

#[request]
pub struct Request {
    /// The identifier of the deletion task to query.
    #[ruma_api(path)]
    pub delete_id: String,
}

#[response]
pub struct Response {
    /// The status of the deletion task.
    #[ruma_api(body)]
    pub status: DeleteStatus,
}

impl Request {
    /// Creates a `Request` with the given delete ID.
    pub fn new(delete_id: String) -> Self {
        Self { delete_id }
    }
}

impl Response {
    /// Creates a `Response` with the given deletion status.
    pub fn new(status: DeleteStatus) -> Self {
        Self { status }
    }
}

/// The status of an asynchronous room deletion task.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct DeleteStatus {
    /// The identifier of the deletion task.
    pub delete_id: String,

    /// The room targeted by the deletion task.
    pub room_id: OwnedRoomId,

    /// The stage the deletion task has reached.
    pub status: DeleteStatusKind,

    /// The outcome of the shutdown, present once the task has finished.
    pub shutdown_room: Option<ShutdownRoom>,
}

impl DeleteStatus {
    /// Creates a `DeleteStatus` with the given IDs and stage.
    pub fn new(delete_id: String, room_id: OwnedRoomId, status: DeleteStatusKind) -> Self {
        Self { delete_id, room_id, status, shutdown_room: None }
    }
}

/// The stage reached by an asynchronous room deletion task.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeleteStatusKind {
    /// The deletion has been scheduled but has not yet started.
    Scheduled,

    /// The deletion is in progress.
    Active,

    /// The deletion has completed successfully.
    Complete,

    /// The deletion has failed.
    Failed,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}
