use reqwest::{IntoUrl, StatusCode, Url};
use serde_derive::Serialize;
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InvalidateParams<'a> {
    access_token: Option<&'a str>,
    client_token: Option<Uuid>,
}

pub struct InvalidateBuilder<'a> {
    params: InvalidateParams<'a>,
    server: Url,
    endpoint: &'a str,
}

impl<'a> InvalidateBuilder<'a> {
    pub fn new() -> InvalidateBuilder<'a> {
        InvalidateBuilder {
            params: InvalidateParams {
                access_token: None,
                client_token: None,
            },
            server: Url::parse("https://authserver.mojang.com").unwrap(),
            endpoint: "/invalidate",
        }
    }

    pub fn client_token(&mut self, client_token: Uuid) -> &mut InvalidateBuilder<'a> {
        self.params.client_token = Some(client_token);
        self
    }

    pub fn access_token(&mut self, access_token: &'a str) -> &mut InvalidateBuilder<'a> {
        self.params.access_token = Some(access_token);
        self
    }

    pub fn server<T: IntoUrl>(&mut self, server: T) -> Result<&mut InvalidateBuilder<'a>> {
        self.server = server.into_url()?;
        Ok(self)
    }

    pub fn endpoint(&mut self, endpoint: &'a str) -> &mut InvalidateBuilder<'a> {
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

        println!("{:?}", response);

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(Error::from_response(response).await),
        }
    }
}
