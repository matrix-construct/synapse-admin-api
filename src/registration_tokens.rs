//! Endpoints in the `/_synapse/admin/v<x>/registration_tokens/` scope.

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use ruma::{MilliSecondsSinceUnixEpoch, UInt, uint};
use serde::{Deserialize, Serialize};

/// A registration token and its usage state.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct RegistrationToken {
    /// The token that can be presented during registration.
    pub token: String,

    /// The number of times this token can be used.
    ///
    /// `None` means the token can be used an unlimited number of times.
    pub uses_allowed: Option<UInt>,

    /// The number of pending uses.
    ///
    /// These are uses for which authentication has completed but registration
    /// has not yet finished.
    pub pending: UInt,

    /// The number of completed uses.
    pub completed: UInt,

    /// The point in time at which the token expires.
    ///
    /// `None` means the token never expires.
    pub expiry_time: Option<MilliSecondsSinceUnixEpoch>,
}

impl RegistrationToken {
    /// Construct a `RegistrationToken` with the given token and all the other
    /// fields set to their default value.
    pub fn new(token: String) -> Self {
        Self {
            token,
            uses_allowed: None,
            pending: uint!(0),
            completed: uint!(0),
            expiry_time: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::RegistrationToken;

    #[test]
    fn unlimited_token_round_trip() {
        let json = json!({
            "token": "abcd1234",
            "uses_allowed": null,
            "pending": 0,
            "completed": 0,
            "expiry_time": null,
        });

        let token: RegistrationToken = serde_json::from_value(json.clone()).unwrap();
        assert!(token.uses_allowed.is_none());
        assert!(token.expiry_time.is_none());

        // The unlimited/never sentinels are explicit nulls on the wire.
        assert_eq!(serde_json::to_value(&token).unwrap(), json);
    }

    #[test]
    fn capped_token_round_trip() {
        let json = json!({
            "token": "abcd1234",
            "uses_allowed": 100,
            "pending": 5,
            "completed": 10,
            "expiry_time": 1_595_376_300_000_u64,
        });

        let token: RegistrationToken = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(token.uses_allowed, Some(ruma::UInt::from(100_u32)));
        assert!(token.expiry_time.is_some());

        assert_eq!(serde_json::to_value(&token).unwrap(), json);
    }
}
