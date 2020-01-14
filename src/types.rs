use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error: String,
    pub error_message: String,
    pub cause: Option<String>,
}
