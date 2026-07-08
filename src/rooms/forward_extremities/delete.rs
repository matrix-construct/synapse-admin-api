//! [DELETE /_synapse/admin/v1/rooms/:room_id_or_alias/forward_extremities](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#deleting-forward-extremities)

use ruma::{
    OwnedRoomOrAliasId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id_or_alias}/forward_extremities",
}

#[request]
pub struct Request {
    /// Alias or ID of the room to delete the extra forward extremities of.
    #[ruma_api(path)]
    pub room_id_or_alias: OwnedRoomOrAliasId,
}

#[response]
pub struct Response {
    /// The number of forward extremities deleted.
    pub deleted: UInt,
}

impl Request {
    /// Creates a `Request` with the given room or alias ID.
    pub fn new(room_id_or_alias: OwnedRoomOrAliasId) -> Self {
        Self { room_id_or_alias }
    }
}

impl Response {
    /// Creates a `Response` with the given number of deleted extremities.
    pub fn new(deleted: UInt) -> Self {
        Self { deleted }
    }
}
