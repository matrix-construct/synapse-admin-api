//! [POST /_synapse/admin/v1/register](https://github.com/element-hq/synapse/blob/master/docs/admin_api/register_api.md)

use std::time::Duration;

#[cfg(feature = "shared-secret-registration-mac")]
use hmac::{Hmac, Mac, digest::InvalidLength};
use ruma::{
    OwnedDeviceId, OwnedServerName, OwnedUserId,
    api::{auth_scheme::NoAuthentication, metadata, request, response},
};
#[cfg(feature = "shared-secret-registration-mac")]
use sha1::Sha1;

#[cfg(feature = "shared-secret-registration-mac")]
type HmacSha1 = Hmac<Sha1>;

metadata! {
    method: POST,
    rate_limited: false,
    authentication: NoAuthentication,
    path: "/_synapse/admin/v1/register",
}

#[request]
#[derive(Default)]
pub struct Request {
    /// The nonce retrieved from the nonce endpoint.
    pub nonce: String,

    /// Localpart for the account.
    pub username: String,

    /// Display name for the account.
    ///
    /// Defaults to the localpart when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displayname: Option<String>,

    /// Password for the account.
    pub password: String,

    /// Whether the account should be an admin.
    #[serde(default)]
    pub admin: bool,

    /// Synapse user type (e.g. `support`, `bot`).
    ///
    /// Folded into the MAC input when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// The MAC is the hex digest output of the HMAC-SHA1 algorithm, with
    /// the key being the shared secret and the content being the nonce,
    /// user, password, either the string "admin" or "notadmin", and
    /// optionally the user_type each separated by NULs.
    pub mac: String,

    /// If `true`, no `access_token`, `device_id`, or `refresh_token` is
    /// returned and no device is created.
    #[serde(default)]
    pub inhibit_login: bool,

    /// If `true`, a refresh token is issued alongside the access token.
    #[serde(default)]
    pub refresh_token: bool,

    /// Device ID to assign. A new one is generated when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<OwnedDeviceId>,

    /// Initial display name for the newly created device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_device_display_name: Option<String>,
}

#[response]
pub struct Response {
    /// Registered user id.
    pub user_id: OwnedUserId,

    /// Homeserver name.
    pub home_server: OwnedServerName,

    /// Access token for the new device.
    ///
    /// Omitted when the request set `inhibit_login`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,

    /// ID of the device created alongside the account.
    ///
    /// Omitted when the request set `inhibit_login`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<OwnedDeviceId>,

    /// Refresh token for the new device.
    ///
    /// Present only when the request asked for one and `inhibit_login` was
    /// not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    /// Lifetime of the access token, in milliseconds.
    ///
    /// `None` if the access token does not expire.
    #[serde(
        with = "ruma::serde::duration::opt_ms",
        default,
        skip_serializing_if = "Option::is_none",
        rename = "expires_in_ms"
    )]
    pub expires_in: Option<Duration>,
}

impl Request {
    /// Creates a `Request` with the given required fields.
    pub fn new(nonce: String, username: String, password: String, mac: String) -> Self {
        Self { nonce, username, password, mac, ..Default::default() }
    }
}

impl Response {
    /// Creates a `Response` with the given required fields.
    pub fn new(user_id: OwnedUserId, home_server: OwnedServerName) -> Self {
        Self {
            user_id,
            home_server,
            access_token: None,
            device_id: None,
            refresh_token: None,
            expires_in: None,
        }
    }
}

/// Calculate the MAC based on the given inputs.
///
/// See <https://github.com/element-hq/synapse/blob/master/docs/admin_api/register_api.md> for details.
#[cfg(feature = "shared-secret-registration-mac")]
pub fn hmac(
    registration_shared_secret: &str,
    nonce: &str,
    username: &str,
    password: &str,
    admin: bool,
    user_type: Option<&str>,
) -> Result<String, InvalidLength> {
    let mut mac = HmacSha1::new_from_slice(registration_shared_secret.as_bytes())?;
    mac.update(nonce.as_bytes());
    mac.update(b"\x00");
    mac.update(username.as_bytes());
    mac.update(b"\x00");
    mac.update(password.as_bytes());
    mac.update(b"\x00");
    mac.update(if admin { b"admin" } else { b"notadmin" });
    if let Some(user_type) = user_type {
        mac.update(b"\x00");
        mac.update(user_type.as_bytes());
    }
    let mac = mac.finalize();
    let mac = hex::encode(mac.into_bytes());

    Ok(mac)
}
