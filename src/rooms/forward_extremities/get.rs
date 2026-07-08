//! [GET /_synapse/admin/v1/rooms/:room_id_or_alias/forward_extremities](https://github.com/element-hq/synapse/blob/master/docs/admin_api/rooms.md#check-for-forward-extremities)

use ruma::{
    OwnedEventId, OwnedRoomOrAliasId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
    uint,
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/rooms/{room_id_or_alias}/forward_extremities",
}

#[request]
pub struct Request {
    /// Alias or ID of the room to query the forward extremities of.
    #[ruma_api(path)]
    pub room_id_or_alias: OwnedRoomOrAliasId,
}

#[response]
pub struct Response {
    /// The number of forward extremities in the room.
    pub count: UInt,

    /// The forward extremities of the room.
    pub results: Vec<ForwardExtremity>,
}

impl Request {
    /// Creates a `Request` with the given room or alias ID.
    pub fn new(room_id_or_alias: OwnedRoomOrAliasId) -> Self {
        Self { room_id_or_alias }
    }
}

impl Response {
    /// Creates a `Response` with the given count and forward extremities.
    pub fn new(count: UInt, results: Vec<ForwardExtremity>) -> Self {
        Self { count, results }
    }
}

/// A single forward extremity of a room.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ForwardExtremity {
    /// The event ID of the forward extremity.
    pub event_id: OwnedEventId,

    /// The state group of the forward extremity, if known.
    pub state_group: Option<UInt>,

    /// The depth of the forward extremity.
    pub depth: UInt,

    /// The timestamp at which the event was received, in milliseconds.
    pub received_ts: UInt,
}

impl ForwardExtremity {
    /// Creates a `ForwardExtremity` with the given event ID and default values.
    pub fn new(event_id: OwnedEventId) -> Self {
        Self { event_id, state_group: None, depth: uint!(0), received_ts: uint!(0) }
    }
}
