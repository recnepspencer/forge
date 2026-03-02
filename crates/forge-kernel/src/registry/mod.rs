//! Feature registry — the dependency injection hub.
//!
//! DOMAIN: Wires concrete features into the engine's generic FeatureTree.
//! This is the ONLY module that knows about specific feature implementations
//! (BooleanFeature, MakePrimitiveFeature). Adding a new feature means:
//! 1. One line in `catalog.rs`
//! 2. One handler file in `handlers/`
//! Never the engine.
//!
//! DEPENDENCIES: engine (FeatureTree, Feature, FeatureOutput),
//!               operations/boolean (BooleanFeature),
//!               primitives (MakePrimitiveFeature)

mod catalog;
mod dispatch;
mod handlers;
mod native_feature;

pub mod facade;
