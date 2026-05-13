//! Shared Forge boundary vocabulary.
//!
//! `forge-foundational` standardizes the meaning exchanged between Forge
//! crates. It is not a hot-path storage runtime, mutation engine, planner, or
//! proof kernel. Domain crates may keep local optimized representations, then
//! materialize foundational boundary forms when they cross crate or artifact
//! boundaries.
//!
//! Milestone 1 begins with the crate boundary itself: all public vocabulary is
//! curated through the facade, while responsibility-shaped internal homes are
//! named before semantic value, aspect, identity, locator, compatibility, and
//! canonicalization types land.

#![forbid(unsafe_code)]

mod aspects;
mod boundary;
mod canonicalization;
mod compatibility;
pub mod facade;
mod identities;
mod locators;
mod profiles;
mod values;

pub use facade::*;
