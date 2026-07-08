//! [PUT /_synapse/admin/v2/users/:user_id](https://github.com/element-hq/synapse/blob/master/docs/admin_api/user_admin_api.md#create-or-modify-account)

use ruma::{
    JsOption, OwnedUserId,
    api::{auth_scheme::AccessToken, metadata, request, response},
    thirdparty::Medium,
};
use serde::{Deserialize, Serialize};

pub use crate::users::{ExternalId, UserDetails};

metadata! {
    method: PUT,
    rate_limited: false,
    authentication: AccessToken,
    path: "/_synapse/admin/v2/users/{user_id}",
}

#[request]
pub struct Request {
    /// User ID for the account to renew
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    /// This is an optional parameter. Add this parameter to create an account or set this
    /// password as new one for an existing account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Whether to log the user out of all their devices when the password is changed.
    ///
    /// Only has an effect when a password is provided. Defaults to true.
    #[serde(default = "ruma::serde::default_true", skip_serializing_if = "ruma::serde::is_true")]
    pub logout_devices: bool,

    // NOTE: Server explodes if attributes are not omitted but specified as null, like the default
    // Serde case.
    /// Defaults to user_id, or the current value if user already exists
    /// Some("") is treated as setting it to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayname: Option<String>,

    /// Defaults to empty, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threepids: Option<Vec<ThirdPartyIdentifier>>,

    /// Defaults to empty, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<ExternalId>>,

    /// The user's avatar URL, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// Should the user be a server admin
    /// defaults to false, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,

    /// Should the user be deactivated
    /// defaults to false, or the current value if user already exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,

    /// Whether the user should be locked.
    ///
    /// Defaults to false, or the current value if user already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    /// The Synapse user type of the account (e.g. `support`, `bot`).
    ///
    /// This is tri-state: absent leaves the current value untouched, an explicit null clears it,
    /// and a value sets it.
    #[serde(default, skip_serializing_if = "ruma::JsOption::is_undefined")]
    pub user_type: JsOption<String>,

    /// Whether the account should be approved.
    ///
    /// Only parsed when MSC3866 support is enabled on the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
}

#[response]
pub struct Response {
    /// Details about the user.
    #[ruma_api(body)]
    pub details: UserDetails,
}

// todo following to does are from synadminctl
// TODO: returns 200 if account-exist-and-was-updated,
// but 201 CREATED if a new account was created.
// However, ruma does throw away this information.

// TODO: what do the EndpointErrors?
// -> can I add custom code, which converts http::Response into ruma embedded error type
// The error is necessary at least at all endpoints which need auth, because a invalid login
// response such an error
// TODO: Should this be the real error like at ruma client api error, is Void-Default enough?
// TODO: ruma api serialize is Ok if status code < 400, else error. That should be discussed.
// The redirect 300 area is Ok too.

impl Request {
    /// Creates a Request with the user ID and the optional password.
    pub fn new(user_id: OwnedUserId, password: Option<String>) -> Self {
        Self {
            user_id,
            password,
            logout_devices: true,
            displayname: None,
            threepids: None,
            external_ids: None,
            avatar_url: None,
            admin: None,
            deactivated: None,
            locked: None,
            user_type: JsOption::Undefined,
            approved: None,
        }
    }
}

impl Response {
    /// Creates a new `Response` with the user details.
    pub fn new(details: UserDetails) -> Self {
        Self { details }
    }
}

/// An identifier external to Matrix.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ThirdPartyIdentifier {
    /// The third party identifier address.
    pub address: String,

    /// The medium of third party identifier.
    pub medium: Medium,
}

#[cfg(test)]
mod tests {
    use ruma::JsOption;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    /// Mirrors the tri-state `user_type` field so the serde attributes can be exercised
    /// independently of the ruma request macro.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct UserTypeField {
        #[serde(default, skip_serializing_if = "ruma::JsOption::is_undefined")]
        user_type: JsOption<String>,
    }

    #[test]
    fn user_type_tri_state_serializes() {
        let undefined = UserTypeField { user_type: JsOption::Undefined };
        assert_eq!(serde_json::to_value(&undefined).unwrap(), json!({}));

        let null = UserTypeField { user_type: JsOption::Null };
        assert_eq!(serde_json::to_value(&null).unwrap(), json!({ "user_type": null }));

        let some = UserTypeField { user_type: JsOption::Some("bot".to_owned()) };
        assert_eq!(serde_json::to_value(&some).unwrap(), json!({ "user_type": "bot" }));
    }

    #[test]
    fn user_type_tri_state_deserializes() {
        assert_eq!(
            serde_json::from_value::<UserTypeField>(json!({})).unwrap().user_type,
            JsOption::Undefined
        );
        assert_eq!(
            serde_json::from_value::<UserTypeField>(json!({ "user_type": null }))
                .unwrap()
                .user_type,
            JsOption::Null
        );
        assert_eq!(
            serde_json::from_value::<UserTypeField>(json!({ "user_type": "bot" }))
                .unwrap()
                .user_type,
            JsOption::Some("bot".to_owned())
        );
    }
}
