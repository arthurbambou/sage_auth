use serde::de::Deserializer;
use serde_derive::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(deserialize_with = "properties_parser")]
    pub properties: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub agent: Option<String>,
    pub id: Uuid,
    pub name: String,
    #[serde(default = "default_false")]
    pub legacy: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error: String,
    pub error_message: String,
    pub cause: Option<String>,
}

fn default_false() -> bool {
    false
}

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
