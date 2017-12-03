//! A collection of models that can be deserialized from response bodies and
//! serialized into request bodies.

use serenity_model::{PartialGuild, Webhook};

/// Structure of data used as the body of a request to exchange the [`code`] for
/// an access token.
///
/// [`code`]: #structfield.code
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessTokenExchangeRequest {
    /// Your application's client ID.
    pub client_id: u64,
    /// Your application's client secret.
    pub client_secret: String,
    /// The code in the query parameters to your redirect URI.
    pub code: String,
    /// The type of grant.
    ///
    /// Must be set to `authorization_code`.
    ///
    /// If using [`AccessTokenExchangeRequest::new`], this will automatically be
    /// set for you.
    pub grant_type: String,
    /// Your redirect URI.
    pub redirect_uri: String,
}

impl AccessTokenExchangeRequest {
    /// Creates a new request body for exchanging a code for an access token.
    ///
    /// # Examples
    ///
    /// Create a new request and assert that the grant type is correct:
    ///
    /// ```rust
    /// use serenity_oauth::model::AccessTokenExchangeRequest;
    ///
    /// let request = AccessTokenExchangeRequest::new(
    ///     249608697955745802,
    ///     "dd99opUAgs7SQEtk2kdRrTMU5zagR2a4",
    ///     "user code here",
    ///     "https://myapplication.website",
    /// );
    ///
    /// assert_eq!(request.grant_type, "authorization_code");
    /// ```
    pub fn new<S, T, U>(
        client_id: u64,
        client_secret: S,
        code: T,
        redirect_uri: U,
    ) -> Self where S: Into<String>, T: Into<String>, U: Into<String> {
        Self {
            client_secret: client_secret.into(),
            code: code.into(),
            grant_type: "authorization_code".to_owned(),
            redirect_uri: redirect_uri.into(),
            client_id,
        }
    }
}

/// Response data containing a new access token and refresh token.
///
/// This can be received when either:
///
/// 1. exchanging a code for an access token;
/// 2. exchanging a refresh token for a fresh access token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    /// The user's access token.
    pub access_token: String,
    /// The number of seconds until the access token expires.
    pub expires_in: u64,
    /// The refresh token to use when the access token expires.
    pub refresh_token: String,
    /// The scope that is granted.
    pub scope: String,
    /// The type of token received.
    pub token_type: String,
}

/// Response data containing an access token, but without a refresh token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClientCredentialsAccessTokenResponse {
    /// The user's access token.
    pub access_token: String,
    /// The number of seconds until the access token expires.
    pub expires_in: u64,
    /// The scope that is granted.
    pub scope: String,
    /// The type of token received.
    pub token_type: String,
}

/// An extended [`Scope::Bot`] authorization flow.
///
/// This will authorize the application as a bot into a user's selected guild,
/// as well as granting additional scopes.
///
/// [`Scope::Bot`]: ../enum.Scope.html#variant.Bot
#[derive(Clone, Debug, Deserialize)]
pub struct ExtendedBotAuthorizationResponse {
    /// The user's access token.
    pub access_token: String,
    /// The number of seconds until the access token expires.
    pub expires_in: u64,
    /// Partial guild data that the application was authorized into.
    pub guild: PartialGuild,
    /// The refresh token to use when the access token expires.
    pub refresh_token: String,
    /// The scope that is granted.
    pub scope: String,
    /// The type of token received.
    pub token_type: String,
}

/// Request for exchanging a refresh token for a new access token.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    /// Your application's client ID.
    pub client_id: u64,
    /// Your application's client secret.
    pub client_secret: String,
    /// The type of grant.
    ///
    /// Must be set to `refresh_token`.
    ///
    /// If using [`RefreshTokenRequest::new`], this will automatically be
    /// set for you.
    pub grant_type: String,
    /// Your redirect URI.
    pub redirect_uri: String,
    /// The user's refresh token.
    pub refresh_token: String,
}

impl RefreshTokenRequest {
    /// Creates a new request body for refreshing an access token using a
    /// refresh token.
    ///
    /// # Examples
    ///
    /// Create a new request and assert that the grant type is correct:
    ///
    /// ```rust
    /// use serenity_oauth::model::RefreshTokenRequest;
    ///
    /// let request = RefreshTokenRequest::new(
    ///     249608697955745802,
    ///     "dd99opUAgs7SQEtk2kdRrTMU5zagR2a4",
    ///     "user code here",
    ///     "https://myapplication.website",
    /// );
    ///
    /// assert_eq!(request.grant_type, "refresh_token");
    /// ```
    pub fn new<S, T, U>(
        client_id: u64,
        client_secret: S,
        redirect_uri: T,
        refresh_token: U,
    ) -> Self where S: Into<String>, T: Into<String>, U: Into<String> {
        Self {
            client_secret: client_secret.into(),
            grant_type: "refresh_token".to_owned(),
            redirect_uri: redirect_uri.into(),
            refresh_token: refresh_token.into(),
            client_id,
        }
    }
}

/// The response data from a successful trading of a code for an access token
/// after authorization of [`Scope::WebhookIncoming`].
///
/// You should store [`webhook`]'s `id` and `token` structfields.
///
/// [`Scope::WebhookIncoming`]: ../enum.Scope.html#variant.WebhookIncoming
/// [`webhook`]: #structfield.webhook
#[derive(Clone, Debug, Deserialize)]
pub struct WebhookTokenResponse {
    /// The user's access token.
    pub access_token: String,
    /// The number of seconds until the access token expires.
    pub expires_in: u64,
    /// The refresh token to use when the access token expires.
    pub refresh_token: String,
    /// The scope that is granted.
    pub scope: String,
    /// The type of token received.
    pub token_type: String,
    /// Information about the webhook created.
    pub webhook: Webhook,
}
