//! A set of constants around the OAuth2 API.

/// The base authorization URI, used for authorizing an application.
pub const BASE_AUTHORIZE_URI: &str = "https://discordapp.com/api/oauth2/authorize";
/// The revocation URL, used to revoke an access token.
pub const BASE_REVOKE_URI: &str = "https://discordapp.com/api/oauth2/revoke";
/// The token URI, used for exchanging a refresh token for a fresh access token
/// and new refresh token.
pub const BASE_TOKEN_URI: &str = "https://discordapp.com/api/oauth2/token";
