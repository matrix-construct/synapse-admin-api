//! [GET /_synapse/admin/v1/users/:user_id/pushers](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#list-all-pushers)

use ruma::{
    OwnedDeviceId, OwnedUserId, UInt,
    api::{auth_scheme::AccessToken, metadata, request, response},
    serde::JsonObject,
};
use serde::{Deserialize, Serialize};

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/users/{user_id}/pushers",
}

#[request]
pub struct Request {
    /// The user to list the pushers of.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response]
pub struct Response {
    /// The pushers registered for the user.
    pub pushers: Vec<Pusher>,

    /// The number of pushers registered for the user.
    pub total: UInt,
}

impl Request {
    /// Creates a `Request` with the given user ID.
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

impl Response {
    /// Creates a `Response` with the given pushers and total count.
    pub fn new(pushers: Vec<Pusher>, total: UInt) -> Self {
        Self { pushers, total }
    }
}

/// A pusher registered for a user.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct Pusher {
    /// A string that allows the user to identify what application owns this pusher.
    pub app_display_name: String,

    /// A reverse-DNS style identifier for the application.
    pub app_id: String,

    /// A dictionary of information for the pusher implementation itself.
    ///
    /// The keys depend on the pusher `kind`; an HTTP pusher carries `url` and
    /// `format`. Modeled as a raw object so no keys are lost on round-trip.
    pub data: Option<JsonObject>,

    /// A string that allows the user to identify what device owns this pusher.
    pub device_display_name: String,

    /// The kind of pusher (e.g. `http`).
    pub kind: String,

    /// The preferred language for receiving notifications.
    pub lang: Option<String>,

    /// A string determining which set of device-specific rules this pusher executes.
    pub profile_tag: String,

    /// A unique identifier for this pusher.
    pub pushkey: String,

    /// Whether the pusher is enabled.
    pub enabled: bool,

    /// The device ID that owns this pusher.
    pub device_id: Option<OwnedDeviceId>,
}

impl Pusher {
    /// Creates a `Pusher` with the given required fields and all others defaulted.
    pub fn new(
        app_display_name: String,
        app_id: String,
        device_display_name: String,
        kind: String,
        profile_tag: String,
        pushkey: String,
        enabled: bool,
    ) -> Self {
        Self {
            app_display_name,
            app_id,
            data: None,
            device_display_name,
            kind,
            lang: None,
            profile_tag,
            pushkey,
            enabled,
            device_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Pusher;

    #[test]
    fn pusher_null_fields_round_trip() {
        let json = json!({
            "app_display_name": "Element",
            "app_id": "im.vector.app",
            "data": null,
            "device_display_name": "phone",
            "kind": "http",
            "lang": null,
            "profile_tag": "",
            "pushkey": "abcd",
            "enabled": true,
            "device_id": null
        });

        let pusher: Pusher = serde_json::from_value(json).unwrap();
        assert!(pusher.data.is_none());
        assert!(pusher.lang.is_none());
        assert!(pusher.device_id.is_none());
        assert!(pusher.enabled);

        // The nullable keys are serialized as explicit nulls, mirroring Synapse.
        let value = serde_json::to_value(&pusher).unwrap();
        assert_eq!(value["data"], json!(null));
        assert_eq!(value["lang"], json!(null));
        assert_eq!(value["device_id"], json!(null));
    }
}
