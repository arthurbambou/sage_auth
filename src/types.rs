//! Common types and conversion functions.

use serde::{de::Deserializer, ser::Serializer};
use serde_derive::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

/// Mojang account information
///
/// You can request this information in [auth](crate::auth) or
/// [refresh](crate::refresh) by calling `request_user()`.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User identifier.
    pub id: Uuid,

    /// Username, format is `user@example.com`.
    pub username: String,
}

/// Account profile
///
/// Currently each account will only have one single profile, multiple profiles
/// per account are however planned in the future. If a user attempts to log
/// into a valid Mojang account with no attached Minecraft license, the
/// authentication will be successful, but the response will not contain a
/// `selected_profile` field, and the `available_profiles` array will be empty.
///
/// See also [https://wiki.vg/Authentication].
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// Presumably same value as you sent in authenticate.
    pub agent: Option<String>,

    /// Profile identifier.
    pub id: Uuid,

    /// Profile name.
    pub name: String,

    /// Only appears in the response if `true`. Default to `false`. Redundant to the newer legacyProfile.
    #[serde(default)]
    pub legacy: bool,
}

/// Mojang API error response
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    /// Short description of the error.
    pub error: String,

    /// Longer description which can be shown to the user.
    pub error_message: String,

    pub cause: Option<String>,
}

/// Convert Mojang special key-value format to [HashMap]
///
/// Mojang account properties are given in the following format:
///
/// ```json
/// [
///     {
///         "name": "preferredLanguage",
///         "value": "en"
///     },
///     {
///         "name": "twitch_access_token",
///         "value": "twitch oauth token"
///     }
/// ]
/// ```
///
/// The function is used to convert this format into [HashMap].
fn properties_parser<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{SeqAccess, Visitor};
    use std::fmt;

    struct PropertiesParser;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Property {
        name: String,
        value: String,
    }

    impl<'de> Visitor<'de> for PropertiesParser {
        type Value = HashMap<String, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("mojang properties, which separate name and value into two fields")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut map = HashMap::new();

            while let Some(property) = seq.next_element::<Property>()? {
                map.insert(property.name, property.value);
            }

            Ok(map)
        }
    }

    deserializer.deserialize_seq(PropertiesParser)
}

/// Serialize Uuid to string without hyphens
pub(crate) fn serialize_uuid_simple<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&uuid.to_simple().to_string())
}
