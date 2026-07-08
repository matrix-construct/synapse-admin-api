//! [POST /_synapse/admin/v1/user/:user_id/redact](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#redact-events-of-a-user)

use ruma::{
    MilliSecondsSinceUnixEpoch, OwnedRoomId, OwnedUserId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/user/{user_id}/redact",
}

#[request]
pub struct Request {
    /// The user whose events should be redacted.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// The rooms in which to redact the user's events.
    ///
    /// An empty list redacts the user's events in every room they are currently joined to or
    /// banned from.
    pub rooms: Vec<OwnedRoomId>,

    /// The reason to attach to each redaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// The maximum number of events to redact per room, newest first.
    ///
    /// Defaults to 1000 when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<UInt>,

    /// Whether to send the redactions as the admin rather than as the target user.
    #[serde(default, skip_serializing_if = "ruma::serde::is_default")]
    pub use_admin: bool,

    /// Only redact events that happened before this timestamp, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// Only redact events that happened after this timestamp, in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_ts: Option<MilliSecondsSinceUnixEpoch>,
}

#[response]
pub struct Response {
    /// An opaque ID with which to poll the redaction status.
    pub redact_id: String,
}

impl Request {
    /// Creates a `Request` with the given user ID and rooms.
    pub fn new(user_id: OwnedUserId, rooms: Vec<OwnedRoomId>) -> Self {
        Self {
            user_id,
            rooms,
            reason: None,
            limit: None,
            use_admin: false,
            before_ts: None,
            after_ts: None,
        }
    }
}

impl Response {
    /// Creates a `Response` with the given redaction ID.
    pub fn new(redact_id: String) -> Self {
        Self { redact_id }
    }
}
