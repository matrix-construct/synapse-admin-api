//! Endpoints in the `/_synapse/admin/v<x>/users/` scope.

pub mod account_data;
pub mod allow_cross_signing_replacement;
pub mod create_or_modify;
pub mod deactivate_account;
pub mod get_details;
pub mod is_user_admin;
pub mod list_joined_rooms;
pub mod list_users;
pub mod login_as;
pub mod lookup_threepid;
pub mod memberships;
pub mod pushers;
pub mod redact;
pub mod redact_status;
pub mod reset_password;
pub mod suspend;

use ruma::{MilliSecondsSinceUnixEpoch, SecondsSinceUnixEpoch, thirdparty::ThirdPartyIdentifier};
use serde::{Deserialize, Serialize};

/// User details
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UserDetails {
    /// The user's name.
    pub name: String,

    /// Is the account a guest
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub is_guest: bool,

    /// Is the user a server admin
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub admin: bool,

    /// Is the account deactivated
    #[serde(deserialize_with = "crate::serde::bool_or_uint")]
    pub deactivated: bool,

    /// Is the account locked
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub locked: bool,

    /// Is the account shadow banned
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub shadow_banned: bool,

    /// Is the account suspended
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub suspended: bool,

    /// Whether the account has been erased following deactivation.
    #[serde(default, deserialize_with = "crate::serde::bool_or_uint")]
    pub erased: bool,

    /// The version of the terms of service the user last consented to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_version: Option<String>,

    /// Whether a server notice about the terms of service has been sent to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_server_notice_sent: Option<bool>,

    /// The time the user consented to the terms of service, in milliseconds.
    pub consent_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// The application service ID that owns this account, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appservice_id: Option<String>,

    /// Creation date for the account, in seconds.
    pub creation_ts: Option<SecondsSinceUnixEpoch>,

    /// The time the user was last seen, in milliseconds.
    pub last_seen_ts: Option<MilliSecondsSinceUnixEpoch>,

    /// The Synapse user type of the account (e.g. `support`, `bot`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    /// Whether the account has been approved, present only when MSC3866 is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,

    /// The user's display name, if set.
    pub displayname: Option<String>,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// A list of third party identifiers the homeserver has associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threepids: Vec<ThirdPartyIdentifier>,

    /// A list of external auth identifiers the homeserver has associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_ids: Vec<ExternalId>,
}

impl UserDetails {
    /// Construct a `UserDetails` with the given user name and all the other fields set to their
    /// default value.
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_guest: false,
            admin: false,
            deactivated: false,
            locked: false,
            shadow_banned: false,
            suspended: false,
            erased: false,
            consent_version: None,
            consent_server_notice_sent: None,
            consent_ts: None,
            appservice_id: None,
            creation_ts: None,
            last_seen_ts: None,
            user_type: None,
            approved: None,
            displayname: None,
            avatar_url: None,
            threepids: Vec::new(),
            external_ids: Vec::new(),
        }
    }
}

/// An external ID associated with a user
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ExternalId {
    /// The authentication provider to which the user is associated.
    pub auth_provider: String,

    /// The ID known to the auth provider associated with this user.
    pub external_id: String,
}

impl ExternalId {
    /// Construct an `ExternalId` with the given authentication provider and ID.
    pub fn new(auth_provider: String, external_id: String) -> Self {
        Self { auth_provider, external_id }
    }
}
