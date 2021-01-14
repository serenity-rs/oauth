//! A module containing bridges to HTTP clients.
//!
//! This contains traits implemented on HTTP clients, as well as oneshot
//! functions that create one-off clients for ease of use.

pub mod hyper;
pub mod reqwest;