//! Aggregation surface for rename-ratchet integration fixtures.
//!
//! Implementation lives in named child modules; this file only re-exports.
//! Each integration test crate embeds this module independently, so some
//! exports are unused in a given crate — that is intentional sharing.

#![allow(dead_code, unused_imports)]

mod repository;
mod retired_tokens;

pub use repository::LegacyReferenceTestRepository;
pub use retired_tokens::{
    retired_hyphen_fragment, retired_hyphen_query_token, retired_query_token,
    retired_underscore_fragment,
};
