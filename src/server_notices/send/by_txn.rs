//! [PUT /_synapse/admin/v1/send_server_notice/:txn_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/server_notices.md)
use ruma::{
    OwnedTransactionId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request},
    events::room::message::RoomMessageEventContent,
    serde::Raw,
};

pub use crate::server_notices::send::v1::Response;

metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/send_server_notice/{txn_id}",
}

#[request]
pub struct Request {
    /// The transaction ID for this request.
    ///
    /// Ensures idempotency of repeated requests with the same ID.
    #[ruma_api(path)]
    pub txn_id: OwnedTransactionId,

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

impl Request {
    /// Creates a `Request` with the given transaction ID, target user and content.
    pub fn new(
        txn_id: OwnedTransactionId,
        user_id: OwnedUserId,
        content: Raw<RoomMessageEventContent>,
    ) -> Self {
        Self { txn_id, user_id, content, event_type: None, state_key: None }
    }
}
