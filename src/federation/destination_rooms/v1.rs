//! [GET /_synapse/admin/v1/federation/destinations/:destination/rooms](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/federation.md#destination-rooms)
use ruma::{
    OwnedRoomId, OwnedServerName, UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/federation/destinations/{destination}/rooms",
}

#[request]
pub struct Request {
    /// The remote server to list the rooms of.
    #[ruma_api(path)]
    pub destination: OwnedServerName,

    /// Offset in the returned list. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<UInt>,

    /// Maximum amount of rooms to return. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// Sort direction of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,
}

#[response]
pub struct Response {
    /// The list of rooms the destination participates in.
    pub rooms: Vec<DestinationRoom>,

    /// Total amount of rooms.
    pub total: UInt,

    /// Token to receive the next batch of rooms.
    ///
    /// Omitted when there are no further rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Request {
    /// Creates a `Request` with the given destination.
    pub fn new(destination: OwnedServerName) -> Self {
        Self { destination, from: None, limit: None, dir: None }
    }
}

impl Response {
    /// Creates a `Response` with the given rooms and total count.
    pub fn new(rooms: Vec<DestinationRoom>, total: UInt) -> Self {
        Self { rooms, total, next_token: None }
    }
}

/// A room a destination participates in.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct DestinationRoom {
    /// The room ID.
    pub room_id: OwnedRoomId,

    /// Stream ordering of the most recent PDU successfully sent to the destination in this room.
    pub stream_ordering: UInt,
}

impl DestinationRoom {
    /// Construct a `DestinationRoom` with the given room ID and stream ordering.
    pub fn new(room_id: OwnedRoomId, stream_ordering: UInt) -> Self {
        Self { room_id, stream_ordering }
    }
}
