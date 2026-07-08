//! Endpoints in the `/_synapse/admin/v<x>/users/:user_id/devices/` scope.

pub mod create_device;
pub mod delete_device;
pub mod delete_devices;
pub mod get_device;
pub mod list_devices;
pub mod update_device;

use ruma::{MilliSecondsSinceUnixEpoch, OwnedDeviceId, OwnedUserId};
use serde::{Deserialize, Serialize};

/// A single device belonging to a user.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct Device {
    /// Identifier of the device.
    pub device_id: OwnedDeviceId,

    /// Display name set by the owner for this device.
    ///
    /// `None` when the owner has not set one.
    pub display_name: Option<String>,

    /// The user who owns this device.
    pub user_id: OwnedUserId,

    /// The IP address where this device was last seen.
    ///
    /// `None` if the device has never been seen.
    pub last_seen_ip: Option<String>,

    /// The user agent reported the last time this device was seen.
    ///
    /// `None` if the device has never been seen.
    pub last_seen_user_agent: Option<String>,

    /// The timestamp at which this device was last seen.
    ///
    /// `None` if the device has never been seen.
    pub last_seen_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// Whether this is a dehydrated device.
    ///
    /// Only present when the owner has a dehydrated device; absent from every
    /// device otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dehydrated: Option<bool>,
}

impl Device {
    /// Construct a `Device` with the given device ID and owner, and all the
    /// other fields set to their default value.
    pub fn new(device_id: OwnedDeviceId, user_id: OwnedUserId) -> Self {
        Self {
            device_id,
            display_name: None,
            user_id,
            last_seen_ip: None,
            last_seen_user_agent: None,
            last_seen_ts: None,
            dehydrated: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Device;

    #[test]
    fn device_never_seen_round_trip() {
        let json = json!({
            "device_id": "ABCDEFGHIJ",
            "display_name": null,
            "user_id": "@alice:example.org",
            "last_seen_ip": null,
            "last_seen_user_agent": null,
            "last_seen_ts": null,
        });

        let device: Device = serde_json::from_value(json.clone()).unwrap();
        assert!(device.display_name.is_none());
        assert!(device.last_seen_ts.is_none());
        assert!(device.dehydrated.is_none());

        // Nulls stay on the wire; the absent `dehydrated` key stays absent.
        assert_eq!(serde_json::to_value(&device).unwrap(), json);
    }

    #[test]
    fn device_with_dehydrated_flag_round_trip() {
        let json = json!({
            "device_id": "ABCDEFGHIJ",
            "display_name": "My phone",
            "user_id": "@alice:example.org",
            "last_seen_ip": "1.2.3.4",
            "last_seen_user_agent": "Mozilla/5.0",
            "last_seen_ts": 1_595_376_300_000_u64,
            "dehydrated": false,
        });

        let device: Device = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(device.dehydrated, Some(false));
        assert!(device.last_seen_ts.is_some());

        assert_eq!(serde_json::to_value(&device).unwrap(), json);
    }
}
