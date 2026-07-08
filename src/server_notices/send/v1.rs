//! [POST /_synapse/admin/v1/send_server_notice](https://github.com/element-hq/synapse/blob/master/docs/admin_api/server_notices.md)
use ruma::{
    OwnedEventId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    events::room::message::RoomMessageEventContent,
    serde::Raw,
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/send_server_notice",
}

#[request]
pub struct Request {
    /// The user to send the server notice to.
    pub user_id: OwnedUserId,

    /// The content of the event to send.
    pub content: Raw<RoomMessageEventContent>,

    /// The type of the event to send.
    ///
    /// Defaults to `m.room.message` when omitted.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,

    /// When set, the notice is sent as a state event with this state key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_key: Option<String>,
}

#[response]
pub struct Response {
    /// The event ID of the sent notice.
    pub event_id: OwnedEventId,
}

impl Request {
    /// Creates a `Request` with the given target user and content.
    pub fn new(user_id: OwnedUserId, content: Raw<RoomMessageEventContent>) -> Self {
        Self { user_id, content, event_type: None, state_key: None }
    }
}

impl Response {
    /// Creates a `Response` with the given event ID.
    pub fn new(event_id: OwnedEventId) -> Self {
        Self { event_id }
    }
}
