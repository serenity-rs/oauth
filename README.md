# serenity-oauth

`serenity-oauth` is a collection of HTTP library support bridges for
interacting with the OAuth2 API that Discord uses.

It includes support for sending code exchange requests and refresh token
requests.

Included are models in the `model` directory that represent request bodies
and response bodies. The `Scope` enum represents possible OAuth2 scopes
that can be granted.

In the `utils` module, functions to produce authorization URLs are
available. For example, `utils::bot_authorization_url` can be used to
produce a URL that can be used to redirect users to authorize an application
with the `Scope::Bot` scope.

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
serenity-oauth = { git = "https://github.com/serenity-rs/oauth" }
```

### Examples

For an example of how to use this in a real-world program, see the [`examples`]
directory.

### License

This project is licensed under [ISC][license].

[license]: https://github.com/serenity-rs/oauth/blob/master/LICENSE.md
[`examples`]: https://github.com/serenity-rs/oauth/tree/master/examples
