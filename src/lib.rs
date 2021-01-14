#![warn(missing_docs)]
//! # serenity-oauth
//!
//! `serenity-oauth` is a collection of HTTP library support bridges for
//! interacting with the OAuth2 API that Discord uses.
//!
//! It includes support for sending code exchange requests and refresh token
//! requests.
//!
//! Included are models in the [`model`] directory that represent request bodies
//! and response bodies. The [`Scope`] enum represents possible OAuth2 scopes
//! that can be granted.
//!
//! In the [`utils`] module, functions to produce authorization URLs are
//! available. For example, [`utils::bot_authorization_url`] can be used to
//! produce a URL that can be used to redirect users to authorize an application
//! with the [`Scope::Bot`] scope.
//!
//! [`Scope`]: enum.Scope.html
//! [`Scope::Bot`]: enum.Scope.html#variant.Bot
//! [`model`]: model/
//! [`utils`]: utils/
//! [`utils::bot_authorization_url`]: utils/fn.bot_authorization_url.html

#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

pub mod bridge;
pub mod constants;
pub mod model;
pub mod utils;

mod error;
mod scope;

pub use bridge::hyper::DiscordOAuthHyperRequester;
pub use bridge::reqwest::DiscordOAuthReqwestRequester;
pub use error::{Error, Result};
pub use scope::Scope;
