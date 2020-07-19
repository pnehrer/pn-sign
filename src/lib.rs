//! # Algorithms for signing PubNub API calls using the secret key
//!
//! See [PubNub Acccess Manager](https://www.pubnub.com/docs/pubnub-rest-api-documentation#pubnub-access-manager-pam)
//! documentation for additional details.
#![feature(backtrace)]

#[doc(hidden)]
pub mod built_info;
pub mod common;
pub mod v1;
pub mod v2;

pub use v1::sign_request as sign_request_v1;
pub use v2::sign_request as sign_request_v2;
