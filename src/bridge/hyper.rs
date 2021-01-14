//! Bridged support for the `hyper` HTTP client.

use crate::constants::BASE_TOKEN_URI;
use crate::model::{AccessTokenExchangeRequest, AccessTokenResponse, RefreshTokenRequest};
use crate::Result;
use hyper::client::{Body, Client as HyperClient};
use hyper::header::ContentType;
use serde_json;
use serde_urlencoded;

/// A trait used that implements methods for interacting with Discord's OAuth2
/// API on Hyper's client.
///
/// # Examples
///
/// Bringing in the trait and creating a client. Since the trait is in scope,
/// the instance of hyper's Client will have those methods available:
///
/// ```rust,no_run
/// extern crate hyper;
/// extern crate serenity_oauth;
///
/// # fn main() {
/// use hyper::Client;
///
/// let client = Client::new();
///
/// // At this point, the methods defined by the trait are not in scope. By
/// // using the trait, they will be.
/// use serenity_oauth::DiscordOAuthHyperRequester;
///
/// // The methods defined by `DiscordOAuthHyperRequester` are now in scope and
/// // implemented on the instance of hyper's `Client`.
/// # }
/// ```
///
/// For examples of how to use the trait with the Client, refer to the trait's
/// methods.
pub trait DiscordOAuthHyperRequester {
    /// Exchanges a code for the user's access token.
    ///
    /// # Examples
    ///
    /// Exchange a code for an access token:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// use hyper::Client;
    /// use serenity_oauth::model::AccessTokenExchangeRequest;
    /// use serenity_oauth::DiscordOAuthHyperRequester;
    ///
    /// let request_data = AccessTokenExchangeRequest::new(
    ///     249608697955745802,
    ///     "dd99opUAgs7SQEtk2kdRrTMU5zagR2a4",
    ///     "user code here",
    ///     "https://myapplication.website",
    /// );
    ///
    /// let client = Client::new();
    /// let response = client.exchange_code(&request_data)?;
    ///
    /// println!("Access token: {}", response.access_token);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn exchange_code(&self, request: &AccessTokenExchangeRequest) -> Result<AccessTokenResponse>;

    /// Exchanges a refresh token, returning a new refresh token and fresh
    /// access token.
    ///
    /// # Examples
    ///
    /// Exchange a refresh token:
    ///
    /// ```rust,no_run
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// use hyper::Client;
    /// use serenity_oauth::model::RefreshTokenRequest;
    /// use serenity_oauth::DiscordOAuthHyperRequester;
    ///
    /// let request_data = RefreshTokenRequest::new(
    ///     249608697955745802,
    ///     "dd99opUAgs7SQEtk2kdRrTMU5zagR2a4",
    ///     "user code here",
    ///     "https://myapplication.website",
    /// );
    ///
    /// let client = Client::new();
    /// let response = client.exchange_refresh_token(&request_data)?;
    ///
    /// println!("Fresh access token: {}", response.access_token);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    fn exchange_refresh_token(&self, request: &RefreshTokenRequest) -> Result<AccessTokenResponse>;
}

impl DiscordOAuthHyperRequester for HyperClient {
    fn exchange_code(&self, request: &AccessTokenExchangeRequest) -> Result<AccessTokenResponse> {
        let body = serde_urlencoded::to_string(request)?;

        let response = self
            .post(BASE_TOKEN_URI)
            .header(ContentType::form_url_encoded())
            .body(Body::BufBody(body.as_bytes(), body.len()))
            .send()?;

        serde_json::from_reader(response).map_err(From::from)
    }

    fn exchange_refresh_token(&self, request: &RefreshTokenRequest) -> Result<AccessTokenResponse> {
        let body = serde_json::to_string(request)?;

        let response = self
            .post(BASE_TOKEN_URI)
            .body(Body::BufBody(body.as_bytes(), body.len()))
            .send()?;

        serde_json::from_reader(response).map_err(From::from)
    }
}
