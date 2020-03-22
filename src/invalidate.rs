//! Invalidate request

use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::Serialize;
use uuid::Uuid;

use crate::consts::DEFAULT_SERVER;
use crate::{Error, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InvalidateParams<'a> {
    access_token: Option<&'a str>,
    client_token: Option<Uuid>,
}

/// `InvalidateBuilder` is used to generate a invalidate request
///
/// An invalidate request can make the given `access_token` invalid.
/// For example:
/// ```no_run
/// # use sage_auth::invalidate::InvalidateBuilder;
/// # use sage_auth::error::Result;
/// # use uuid::Uuid;
/// # async fn anonymous() -> Result<()> {
/// let resp = InvalidateBuilder::new()
///     .access_token("ACCESS_TOKEN")
///     .client_token(Uuid::new_v4())
///     .request()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct InvalidateBuilder<'a> {
    params: InvalidateParams<'a>,
    server: Url,
    endpoint: &'a str,
}

impl Default for InvalidateParams<'_> {
    fn default() -> InvalidateParams<'static> {
        InvalidateParams {
            access_token: None,
            client_token: None,
        }
    }
}

impl Default for InvalidateBuilder<'_> {
    fn default() -> InvalidateBuilder<'static> {
        InvalidateBuilder {
            params: InvalidateParams::default(),
            server: (*DEFAULT_SERVER).clone(),
            endpoint: "/invalidate",
        }
    }
}

impl<'a> InvalidateBuilder<'a> {
    pub fn new() -> InvalidateBuilder<'a> {
        InvalidateBuilder::default()
    }

    /// Client token, the same as when you request `access_token`.
    pub fn client_token(&mut self, client_token: Uuid) -> &mut InvalidateBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    /// `access_token` to invalidate.
    pub fn access_token(&mut self, access_token: &'a str) -> &mut InvalidateBuilder<'a> {
        self.params.access_token = Some(access_token);
        self
    }

    /// Set base url, default is `https://authserver.mojang.com`.
    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut InvalidateBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    /// set endpoint, default is `/authenticate`.
    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut InvalidateBuilder<'a> {
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

        println!("{:?}", response);

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(Error::from_response(response).await),
        }
    }
}
