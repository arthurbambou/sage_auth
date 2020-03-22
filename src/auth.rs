//! Authenticate request

use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::consts::DEFAULT_SERVER;
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

/// `AuthenticateBuilder` is used to generate a authenticate request
///
/// For example:
/// ```no_run
/// # use sage_auth::auth::AuthenticateBuilder;
/// # use sage_auth::error::Result;
/// # async fn anonymous() -> Result<()> {
/// let resp = AuthenticateBuilder::new()
///     .username("USERNAME")
///     .password("PASSWORD")
///     .request()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct AuthenticateBuilder<'a> {
    params: AuthenticateParams<'a>,
    server: Url,
    endpoint: &'a str,
}

/// Response body from Mojang server
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateResponse {
    /// Hexadecimal or JSON-Web-Token (unconfirmed) [The normal accessToken
    /// can be found in the payload of the JWT (second by '.' separated part
    /// as Base64 encoded JSON object), in key "yggt"].
    pub access_token: String,

    /// The same as sent.
    pub client_token: Uuid,

    /// Available profiles, only present if the agent field was received.
    pub available_profiles: Vec<Profile>,

    pub selected_profile: Option<Profile>,

    /// Only present if `request_user` is set.
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
            server: (*DEFAULT_SERVER).clone(),
            endpoint: "/authenticate",
        }
    }

    /// Set username
    pub fn username(&mut self, username: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.params.username = Some(username);
        self
    }

    /// Set password
    pub fn password(&mut self, password: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.params.password = Some(password);
        self
    }

    /// Specify a client token.
    /// If is not provided, it will be generated when making a request.
    pub fn client_token(&mut self, client_token: Uuid) -> &mut AuthenticateBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    /// Set to request user profile.
    /// If set, `AuthenticateResponse` will contain the user profile.
    pub fn request_user(&mut self) -> &mut AuthenticateBuilder<'a> {
        self.params.request_user = true;
        self
    }

    /// Set agent name, default is `Minecraft`
    pub fn agent_name(&mut self, agent_name: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.params.agent.name = agent_name;
        self
    }

    /// Set agent version, default is `1`
    pub fn agent_version(&mut self, agent_version: i32) -> &mut AuthenticateBuilder<'a> {
        self.params.agent.version = agent_version;
        self
    }

    /// Set base url, default is `https://authserver.mojang.com`.
    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut AuthenticateBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    /// set endpoint, default is `/authenticate`.
    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut AuthenticateBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    /// Make a request with the given parameters.
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
