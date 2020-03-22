//! Validate request

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

/// `ValidateBuilder` is used to generate a validate request
///
/// It can check if an `access_token` is usable for authentication with a Minecraft server.
///
/// For more details, see [https://wiki.vg/Authentication].
/// For example:
/// ```no_run
/// # use sage_auth::validate::ValidateBuilder;
/// # use sage_auth::error::Result;
/// # use uuid::Uuid;
/// # async fn anonymous() -> Result<()> {
/// let resp = ValidateBuilder::new()
///     .access_token("ACCESS_TOKEN")
///     .client_token(Uuid::new_v4())
///     .request()
///     .await?;
/// # Ok(())
/// # }
/// ```
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

    /// Client token, the same as when you request `access_token`.
    pub fn client_token(&mut self, client_token: Uuid) -> &mut ValidateBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    /// `access_token` to invalidate.
    pub fn access_token(&mut self, access_token: &'a str) -> &mut ValidateBuilder<'a> {
        self.params.access_token = Some(access_token);
        self
    }

    /// Set base url, default is `https://authserver.mojang.com`.
    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut ValidateBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    /// set endpoint, default is `/authenticate`.
    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut ValidateBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    /// Make a request with the given parameters.
    /// If success, it will return `Ok(())`.
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
