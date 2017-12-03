use std::fmt::{Display, Formatter, Result as FmtResult};

/// A Discord OAuth2 scope that can be granted.
///
/// If you require a scope that is not registered here, use [`Scope::Other`] and
/// notify the library developers about the missing scope.
///
/// **Note**: The [`Scope::Bot`] and [`Scope::GuildsJoin`] scopes require you to
/// have a bot account linked to your application. Also, in order to add a user
/// to a guild, your bot has to already belong in that guild.
///
/// [`Scope::Bot`]: #variant.Bot
/// [`Scope::GuildsJoin`]: #variant.GuildsJoin
/// [`Scope::Other`]: #variant.Other
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Scope {
    /// For OAuth2 bots, this puts the bot in the user's selected guild by
    /// default.
    Bot,
    /// Allows the `/users/@me/connections` API endpoint to return linked
    /// third-party accounts.
    Connections,
    /// Enables the `/users/@me` API endpoint to return an `email` field.
    Email,
    /// Allows the `/users/@me` API endpoint, without the `email` field.
    Identify,
    /// Allows the `/users/@me/guilds` API endpoint to return basic information
    /// about all of a user's guilds.
    Guilds,
    /// Allows the `/invites/{code}` API endpoint to be used for joining users
    /// to a guild.
    GuildsJoin,
    /// Allows your application to join users to a group DM.
    GdmJoin,
    /// For local RPC server API access, this allows you to read messages from
    /// all cliuent channels.
    ///
    /// This is otherwise restricted to channels/guilds your application
    /// creates.
    MessagesRead,
    /// For local RPC server access, this allows you to control a user's local
    /// Discord client.
    Rpc,
    /// For local RPC server API access, this allows you to access the API as
    /// the local user.
    RpcApi,
    /// For local RPC server API access, this allows you to receive
    /// notifications pushed out to the user.
    RpcNotificationsRead,
    /// This generates a webhook that is returned in the OAuth token response
    /// for authorization code grants.
    WebhookIncoming,
    /// A scope that does not have a matching enum variant.
    Other(String),
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Scope::*;

        f.write_str(match *self {
            Bot => "bot",
            Connections => "connections",
            Email => "email",
            Identify => "identify",
            Guilds => "guilds",
            GuildsJoin => "guilds.join",
            GdmJoin => "gdm.join",
            MessagesRead => "messages.read",
            Rpc => "rpc",
            RpcApi => "rpc.api",
            RpcNotificationsRead => "rpc.notifications.read",
            WebhookIncoming => "webhook.incoming",
            Other(ref inner) => inner,
        })
    }
}
