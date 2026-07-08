//! [GET /_synapse/admin/v1/federation/destinations](https://github.com/element-hq/synapse/blob/master/docs/usage/administration/admin_api/federation.md#list-of-destinations)
use ruma::{
    UInt,
    api::{Direction, auth_scheme::AccessToken, metadata, request, response},
    serde::StringEnum,
};

pub use crate::federation::Destination;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/federation/destinations",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// Offset in the returned list. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub from: Option<UInt>,

    /// Maximum amount of destinations to return. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<UInt>,

    /// Filter destinations by a substring of the server name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub destination: Option<String>,

    /// Sort order of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub order_by: Option<DestinationSortOrder>,

    /// Sort direction of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub dir: Option<Direction>,
}

#[response]
pub struct Response {
    /// The list of destinations.
    pub destinations: Vec<Destination>,

    /// Total amount of destinations.
    pub total: UInt,

    /// Token to receive the next batch of destinations.
    ///
    /// Omitted when there are no further destinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a `Response` with the given destinations and total count.
    pub fn new(destinations: Vec<Destination>, total: UInt) -> Self {
        Self { destinations, total, next_token: None }
    }
}

/// Enum to define the sorting method of destinations.
#[derive(Clone, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DestinationSortOrder {
    /// Sort by destination server name.
    Destination,

    /// Sort by the timestamp of the last retry attempt.
    RetryLastTs,

    /// Sort by the retry interval.
    RetryInterval,

    /// Sort by the timestamp of the first failure in the current streak.
    FailureTs,

    /// Sort by the stream ordering of the last successfully sent PDU.
    LastSuccessfulStreamOrdering,

    #[doc(hidden)]
    _Custom(crate::PrivOwnedStr),
}
