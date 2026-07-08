//! [PUT /_synapse/admin/v2/users/:user_id/devices/:device_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#update-a-device)

use ruma::{
    OwnedDeviceId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/users/{user_id}/devices/{device_id}",
}

#[request]
pub struct Request {
    /// The user who owns the device.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// The device to update.
    #[ruma_api(path)]
    pub device_id: OwnedDeviceId,

    /// The new display name for the device.
    ///
    /// Omitting it leaves the display name unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Default)]
#[response]
pub struct Response {}

impl Request {
    /// Creates a `Request` with the given user ID and device ID.
    pub fn new(user_id: OwnedUserId, device_id: OwnedDeviceId) -> Self {
        Self { user_id, device_id, display_name: None }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Self {}
    }
}
