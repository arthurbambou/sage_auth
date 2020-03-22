//! Refresh request

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

/// `RefreshBuilder` is used to generate a refresh request
///
/// Refresh a valid `access_token`.
/// It can be used to keep a user logged in between gaming sessions and is
/// preferred over storing the user's password in a file.
///
/// For example:
/// ```no_run
/// # use sage_auth::refresh::RefreshBuilder;
/// # use sage_auth::error::Result;
/// # use uuid::Uuid;
/// # async fn anonymous() -> Result<()> {
/// let resp = RefreshBuilder::new()
///     .access_token("ACCESS_TOKEN")
///     .client_token(Uuid::new_v4())
///     .request()
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct RefreshBuilder<'a> {
    params: RefreshParams<'a>,
    server: Url,
    endpoint: &'a str,
}

/// Response body from Mojang server
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    /// A new `access_token`. For more details, see
    /// [AuthenticateResponse](crate::auth::AuthenticateResponse::access_token).
    pub access_token: String,

    /// The same as sent.
    pub client_token: Uuid,

    pub selected_profile: Option<Profile>,

    /// Only present if `request_user` is set.
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

    /// Client token, the same as when you request `access_token`.
    pub fn client_token(&mut self, client_token: Uuid) -> &mut RefreshBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    /// Set to request user profile.
    /// If set, `AuthenticateResponse` will contain the user profile.
    pub fn request_user(&mut self) -> &mut RefreshBuilder<'a> {
        self.params.request_user = true;
        self
    }

    /// `access_token` to invalidate.
    pub fn access_token(&mut self, access_token: &'a str) -> &mut RefreshBuilder<'a> {
        self.params.access_token = Some(access_token);
        self
    }

    /// Set base url, default is `https://authserver.mojang.com`.
    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut RefreshBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    /// set endpoint, default is `/authenticate`.
    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut RefreshBuilder<'a> {
        self.endpoint = endpoint;
        self
    }

    /// Make a request with the given parameters.
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
