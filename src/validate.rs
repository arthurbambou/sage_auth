use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::Serialize;
use uuid::Uuid;

use crate::consts::DEFAULT_SERVER;
use crate::{Error, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateParams<'a> {
    access_token: Option<&'a str>,
    client_token: Option<Uuid>,
}

pub struct ValidateBuilder<'a> {
    params: ValidateParams<'a>,
    server: Url,
    endpoint: &'a str,
}

impl<'a> ValidateBuilder<'a> {
    pub fn new() -> ValidateBuilder<'a> {
        ValidateBuilder {
            params: ValidateParams {
                access_token: None,
                client_token: None,
            },
            server: (*DEFAULT_SERVER).clone(),
            endpoint: "/validate",
        }
    }

    pub fn client_token(&mut self, client_token: Uuid) -> &mut ValidateBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    pub fn access_token(&mut self, access_token: &'a str) -> &mut ValidateBuilder<'a> {
        self.params.access_token = Some(access_token);
        self
    }

    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut ValidateBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut ValidateBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    pub async fn request(&mut self) -> Result<()> {
        if self.params.access_token.is_none() {
            return Err(Error::MissingField("access_token"));
        }
        if self.params.client_token.is_none() {
            return Err(Error::MissingField("client_token"));
        }

        let client = reqwest::Client::new();
        let response = client
            .post(self.server.join(self.endpoint)?)
            .json(&self.params)
            .send()
            .await?;

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(Error::from_response(response).await),
        }
    }
}
