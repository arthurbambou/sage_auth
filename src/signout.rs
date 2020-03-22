//! Signout request

use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::Serialize;

use crate::consts::DEFAULT_SERVER;
use crate::{Error, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignoutParams<'a> {
    username: Option<&'a str>,
    password: Option<&'a str>,
}

/// `SignoutBuilder` is used to generate a signout request
///
/// Invalidates `access_token`s using an account's username and password.
/// For example:
/// ```no_run
/// # use sage_auth::signout::SignoutBuilder;
/// # use sage_auth::error::Result;
/// # async fn anonymous() -> Result<()> {
/// let resp = SignoutBuilder::new()
///     .username("USERNAME")
///     .password("PASSWORD")
///     .request()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SignoutBuilder<'a> {
    params: SignoutParams<'a>,
    server: Url,
    endpoint: &'a str,
}

impl Default for SignoutParams<'_> {
    fn default() -> SignoutParams<'static> {
        SignoutParams {
            username: None,
            password: None,
        }
    }
}

impl Default for SignoutBuilder<'_> {
    fn default() -> SignoutBuilder<'static> {
        SignoutBuilder {
            params: SignoutParams::default(),
            server: (*DEFAULT_SERVER).clone(),
            endpoint: "/signout",
        }
    }
}

impl<'a> SignoutBuilder<'a> {
    pub fn new() -> SignoutBuilder<'a> {
        SignoutBuilder::default()
    }

    /// Set username
    pub fn username(&mut self, username: &'a str) -> &mut SignoutBuilder<'a> {
        self.params.username = Some(username);
        self
    }

    /// Set password
    pub fn password(&mut self, password: &'a str) -> &mut SignoutBuilder<'a> {
        self.params.password = Some(password);
        self
    }

    /// Set base url, default is `https://authserver.mojang.com`.
    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut SignoutBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    /// set endpoint, default is `/authenticate`.
    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut SignoutBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    /// Make a request with the given parameters.
    /// If success, it will return `Ok(())`.
    pub async fn request(&mut self) -> Result<()> {
        if self.params.username.is_none() {
            return Err(Error::MissingField("username"));
        }
        if self.params.password.is_none() {
            return Err(Error::MissingField("password"));
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
