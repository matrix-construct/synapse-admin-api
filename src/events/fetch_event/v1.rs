//! [GET /_synapse/admin/v1/fetch_event/:event_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/fetch_event.md)
use ruma::{
    OwnedEventId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    serde::{JsonObject, Raw},
};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/fetch_event/{event_id}",
}

#[request]
pub struct Request {
    /// The event to fetch.
    #[ruma_api(path)]
    pub event_id: OwnedEventId,
}

#[response]
pub struct Response {
    /// The event, served unredacted in raw federation format.
    pub event: Raw<JsonObject>,
}

impl Request {
    /// Creates a `Request` with the given event ID.
    pub fn new(event_id: OwnedEventId) -> Self {
        Self { event_id }
    }
}

impl Response {
    /// Creates a `Response` with the given event.
    pub fn new(event: Raw<JsonObject>) -> Self {
        Self { event }
    }
}
