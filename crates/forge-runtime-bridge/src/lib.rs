//! `forge-runtime-bridge` owns the runtime bridge facade and its internal
//! subdomain boundaries.
//!
//! Milestone 1 currently includes canonical truth-envelope and snapshot input
//! contracts, build-time-frozen bridge mapping registration, and deterministic
//! routing plan / invalidation artifact lowering.
//!
//! Milestone 5 bulk planning begins by composing canonical single-route plans
//! into a replay-safe batch workload identity and summary without reintroducing
//! scalar orchestration at the public boundary.
//!
//! For ordinary usage, start with [`facade`].
//!
//! ```no_run
//! use forge_runtime_bridge::facade::{
//!     BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
//!     RuntimeBridge, SignalInvalidationScope, SnapshotReadContract, TruthPatchScope,
//!     TruthCommitIdentity,
//! };
//! use forge_foundational::facade::{AspectKey, FieldKey, ScalarAspectType};
//!
//! fn standard_path_example<
//!     TruthSource,
//!     BranchHeads,
//!     ComputeSink,
//! >(
//!     truth_source: TruthSource,
//!     branch_heads: BranchHeads,
//!     compute_sink: ComputeSink,
//! ) -> Result<(), Box<dyn std::error::Error>>
//! where
//!     TruthSource: forge_runtime_bridge::facade::RelationalBridgeSource + Clone + 'static,
//!     BranchHeads: forge_runtime_bridge::facade::TruthBranchHeadSource + Clone + 'static,
//!     ComputeSink: forge_runtime_bridge::facade::SignalBridgeSink + Clone + 'static,
//! {
//!     let bridge = RuntimeBridge::builder()
//!         .with_truth_source(truth_source)
//!         .with_truth_branch_head_source(branch_heads)
//!         .with_compute_sink(compute_sink)
//!         .register_mapping(BridgeMappingRegistration::new(
//!             BridgeMappingId::new("pricing:steel"),
//!             TruthPatchScope::for_entity_field(
//!                 MappingSelector::exact("component:steel"),
//!                 AspectKey::new("cost").expect("valid aspect key"),
//!                 FieldKey::new("usd".to_owned()).expect("valid field key"),
//!             ),
//!             SnapshotReadContract::scalar(
//!                 AspectKey::new("cost").expect("valid aspect key"),
//!                 ScalarAspectType::String,
//!             ),
//!             SignalInvalidationScope::new("price:bicycle"),
//!             CoarseRoutingMode::Direct,
//!         ))
//!         .build()?;
//!
//!     let route = bridge.route(TruthCommitIdentity::new("commit:steel-main"))?;
//!     let evaluation = bridge.evaluate_current(route.target())?;
//!     let diagnostics = bridge.diagnostics().explain_last();
//!
//!     let _ = evaluation;
//!     let _ = diagnostics;
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]

mod adapter;
mod builder;
mod canonical_basis;
mod clone_budget;
mod continuity;
mod delivery;
mod diagnostics;
mod error;
pub mod facade;
mod historical;
mod identity;
mod input;
mod mapping;
mod merge;
mod policy;
mod routing;
mod snapshot;
mod source;
mod speculation;
mod stream;
mod structural;
mod subscription;
mod writeback;

#[cfg(test)]
mod harness;

#[cfg(test)]
mod tests {}
