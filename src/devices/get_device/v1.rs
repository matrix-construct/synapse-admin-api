//! [GET /_synapse/admin/v2/users/:user_id/devices/:device_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#show-a-device)

use ruma::{
    OwnedDeviceId, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};

pub use crate::devices::Device;

metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/users/{user_id}/devices/{device_id}",
}

#[request]
pub struct Request {
    /// The user who owns the device.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// The device to show.
    #[ruma_api(path)]
    pub device_id: OwnedDeviceId,
}

#[response]
pub struct Response {
    /// The requested device.
    #[ruma_api(body)]
    pub device: Device,
}

impl Request {
    /// Creates a `Request` with the given user ID and device ID.
    pub fn new(user_id: OwnedUserId, device_id: OwnedDeviceId) -> Self {
        Self { user_id, device_id }
    }
}

impl Response {
    /// Creates a `Response` with the given device.
    pub fn new(device: Device) -> Self {
        Self { device }
    }
}
