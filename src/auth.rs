use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Profile, User};
use crate::{Error, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AgentInfo<'a> {
    name: &'a str,
    version: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateParams<'a> {
    username: Option<&'a str>,
    password: Option<&'a str>,
    client_token: Option<Uuid>,
    request_user: bool,
    agent: AgentInfo<'a>,
}

pub struct AuthenticateBuilder<'a> {
    params: AuthenticateParams<'a>,
    server: Url,
    endpoint: &'a str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub client_token: Uuid,
    pub available_profiles: Vec<Profile>,
    pub selected_profile: Option<Profile>,
    pub user: Option<User>,
}

impl<'a> AuthenticateBuilder<'a> {
    pub fn new() -> AuthenticateBuilder<'a> {
        AuthenticateBuilder {
            params: AuthenticateParams {
                username: None,
                password: None,
                client_token: None,
                request_user: false,
                agent: AgentInfo {
                    name: "Minecraft",
                    version: 1,
                },
            },
            server: Url::parse("https://authserver.mojang.com").unwrap(),
            endpoint: "/authenticate",
        }
    }

    pub fn username(&mut self, username: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.params.username = Some(username);
        self
    }

    pub fn password(&mut self, password: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.params.password = Some(password);
        self
    }

    pub fn client_token(&mut self, client_token: Uuid) -> &mut AuthenticateBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    pub fn request_user(&mut self) -> &mut AuthenticateBuilder<'a> {
        self.params.request_user = true;
        self
    }

    pub fn agent_name(&mut self, agent_name: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.params.agent.name = agent_name;
        self
    }

    pub fn agent_version(&mut self, agent_version: i32) -> &mut AuthenticateBuilder<'a> {
        self.params.agent.version = agent_version;
        self
    }

    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut AuthenticateBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    pub async fn request(&mut self) -> Result<AuthenticateResponse> {
        if self.params.username.is_none() {
            return Err(Error::MissingField("username"));
        }
        if self.params.password.is_none() {
            return Err(Error::MissingField("password"));
        }
        if self.params.client_token.is_none() {
            self.params.client_token = Some(Uuid::new_v4());
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
