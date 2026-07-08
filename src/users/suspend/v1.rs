//! [PUT /_synapse/admin/v1/suspend/:user_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#suspendunsuspend-account)

use std::fmt;

use ruma::{
    OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
};
use serde::{
    Deserialize, Serialize,
    de::{Error as _, MapAccess, Visitor},
    ser::SerializeMap,
};

metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v1/suspend/{user_id}",
}

#[request]
pub struct Request {
    /// The user to suspend or unsuspend.
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// Whether the account should be suspended.
    pub suspend: bool,
}

#[response]
pub struct Response {
    /// The resulting suspension state of the account.
    #[ruma_api(body)]
    pub result: Suspended,
}

impl Request {
    /// Creates a `Request` with the given user ID and suspension state.
    pub fn new(user_id: OwnedUserId, suspend: bool) -> Self {
        Self { user_id, suspend }
    }
}

impl Response {
    /// Creates a `Response` with the given suspension result.
    pub fn new(result: Suspended) -> Self {
        Self { result }
    }
}

/// The suspension state of an account, carried over the wire as a single dynamic map entry of the
/// form `{"user_<user_id>_suspended": <bool>}`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct Suspended {
    /// The user whose suspension state this describes.
    pub user_id: OwnedUserId,

    /// Whether the account is suspended.
    pub suspended: bool,
}

impl Suspended {
    /// Creates a `Suspended` with the given user ID and suspension state.
    pub fn new(user_id: OwnedUserId, suspended: bool) -> Self {
        Self { user_id, suspended }
    }
}

impl Serialize for Suspended {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let key = format!("user_{}_suspended", self.user_id);
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&key, &self.suspended)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Suspended {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SuspendedVisitor;

        impl<'de> Visitor<'de> for SuspendedVisitor {
            type Value = Suspended;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map with a single `user_<user_id>_suspended` entry")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let (key, suspended): (String, bool) = map
                    .next_entry()?
                    .ok_or_else(|| A::Error::custom("expected a single map entry"))?;

                let inner = key
                    .strip_prefix("user_")
                    .and_then(|rest| rest.strip_suffix("_suspended"))
                    .ok_or_else(|| A::Error::custom("unexpected suspend key format"))?;

                let user_id = OwnedUserId::try_from(inner).map_err(A::Error::custom)?;

                Ok(Suspended { user_id, suspended })
            }
        }

        deserializer.deserialize_map(SuspendedVisitor)
    }
}

#[cfg(test)]
mod tests {
    use ruma::OwnedUserId;
    use serde_json::json;

    use super::Suspended;

    #[test]
    fn suspended_dynamic_key_round_trip() {
        let user_id = OwnedUserId::try_from("@alice:example.com").unwrap();
        let suspended = Suspended::new(user_id.clone(), true);

        let value = serde_json::to_value(&suspended).unwrap();
        assert_eq!(value, json!({ "user_@alice:example.com_suspended": true }));

        let parsed: Suspended = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, suspended);
        assert_eq!(parsed.user_id, user_id);
        assert!(parsed.suspended);
    }
}
