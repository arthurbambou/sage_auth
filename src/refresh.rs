use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::consts::DEFAULT_SERVER;
use crate::types::{Profile, User};
use crate::{Error, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RefreshParams<'a> {
    access_token: Option<&'a str>,
    client_token: Option<Uuid>,
    request_user: bool,
}

pub struct RefreshBuilder<'a> {
    params: RefreshParams<'a>,
    server: Url,
    endpoint: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub access_token: String,
    pub client_token: Uuid,
    pub selected_profile: Option<Profile>,
    pub user: Option<User>,
}

impl<'a> RefreshBuilder<'a> {
    pub fn new() -> RefreshBuilder<'a> {
        RefreshBuilder {
            params: RefreshParams {
                access_token: None,
                client_token: None,
                request_user: false,
            },
            server: (*DEFAULT_SERVER).clone(),
            endpoint: "/refresh",
        }
    }

    pub fn client_token(&mut self, client_token: Uuid) -> &mut RefreshBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    pub fn request_user(&mut self) -> &mut RefreshBuilder<'a> {
        self.params.request_user = true;
        self
    }

    pub fn access_token(&mut self, access_token: &'a str) -> &mut RefreshBuilder<'a> {
        self.params.access_token = Some(access_token);
        self
    }

    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut RefreshBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut RefreshBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    pub async fn request(&mut self) -> Result<RefreshResponse> {
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
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(Error::from_response(response).await),
        }
    }
}
