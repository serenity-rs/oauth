//! A collection of functions for use with the OAuth2 API.
//!
//! This includes functions for easily generating URLs to redirect users to for
//! authorization.

pub use serenity_model::Permissions;

use constants::BASE_AUTHORIZE_URI;
use percent_encoding;
use super::Scope;
use std::fmt::Write;

/// Creates a URL for a simple bot authorization flow.
///
/// This is a special,
/// server-less and callback-less OAuth2 flow that makes it
/// easy for users to add bots to guilds.
///
/// # Examples
///
/// Create an authorization URL for a bot requiring the "Add Reactions" and
/// "Send Messages" permissions:
///
/// ```rust
/// extern crate serenity_model;
/// extern crate serenity_oauth;
///
/// # fn main() {
/// use serenity_model::Permissions;
///
/// let client_id = 249608697955745802;
/// let required = Permissions::ADD_REACTIONS | Permissions::SEND_MESSAGES;
/// let url = serenity_oauth::utils::bot_authorization_url(client_id, required);
///
/// // Assert that the expected URL is correct.
/// let expected = "https://discordapp.com/api/oauth2/authorize?client_id=249608697955745802&scope=bot&permissions=2112";
/// assert_eq!(url, expected);
/// # }
/// ```
pub fn bot_authorization_url(client_id: u64, permissions: Permissions)
    -> String {
    format!(
        "{}?client_id={}&scope=bot&permissions={}",
        BASE_AUTHORIZE_URI,
        client_id,
        permissions.bits(),
    )
}

/// Creates a URL for an authorization code grant.
///
/// This will create a URL to redirect the user to, requesting the given scopes
/// for your client ID.
///
/// The given `redirect_uri` will automatically be URL encoded.
///
/// A state _should_ be passed, as recommended by RFC 6749. It is a unique
/// identifier for the user's request. When Discord redirects the user to the
/// given redirect URI, it will append a `state` parameter to the query. It will
/// match the state that you have recorded for that user. If it does not, there
/// was likely a request interception.
///
/// As well as the callback URL having the same `state` appended in the query
/// parameters, this will also append a `code`.
///
/// # Examples
///
/// Produce an authorization code grant URL for your client, requiring the
/// [`Scope::Identify`] and [`Scope::GuildsJoin`] scopes, and an example of a
/// state:
///
/// **Note**: Please randomly generate this using a crate like `rand`.
///
/// ```rust
/// use serenity_oauth::Scope;
///
/// let client_id = 249608697955745802;
/// let scopes = [Scope::GuildsJoin, Scope::Identify];
/// let state = "15773059ghq9183habn";
/// let redirect_uri = "https://myapplication.website";
///
/// let url = serenity_oauth::utils::authorization_code_grant_url(
///     client_id,
///     &scopes,
///     Some(state),
///     redirect_uri,
/// );
///
/// // Assert that the URL is correct.
/// let expected = "https://discordapp.com/api/oauth2/authorize?response_type=code&client_id=249608697955745802&redirect_uri=https%3A%2F%2Fmyapplication.website&scope=guilds.join%20identify&state=15773059ghq9183habn";
/// assert_eq!(url, expected);
/// ```
///
/// [`Scope::GuildsJoin`]: enum.Scope.html#variant.GuildsJoin
/// [`Scope::Identify`]: enum.Scope.html#variant.Identify
pub fn authorization_code_grant_url(
    client_id: u64,
    scopes: &[Scope],
    state: Option<&str>,
    redirect_uri: &str,
) -> String {
    let mut base = String::from(BASE_AUTHORIZE_URI);
    let uri = percent_encoding::percent_encode(
        redirect_uri.as_bytes(),
        percent_encoding::USERINFO_ENCODE_SET,
    );

    let _ = write!(
        base,
        "?response_type=code&client_id={}&redirect_uri={}&scope=",
        client_id,
        uri,
    );

    let scope_count = scopes.len();

    for (i, scope) in scopes.iter().enumerate() {
        let _ = write!(base, "{}", scope);

        if i + 1 < scope_count {
            base.push_str("%20");
        }
    }

    if let Some(state) = state {
        let _ = write!(base, "&state={}", state);
    }

    base
}
