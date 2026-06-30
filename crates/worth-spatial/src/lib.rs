//! # worth-spatial
//!
//! `worth-spatial` owns authored spatial vocabulary and spatial semantics.
//! `forge-query` owns runtime-facing declaration, inspection, and workflow
//! lifecycle.
//!
//! Use the namespaced [`facade`] modules as the public entry surface:
//!
//! - [`facade::refs`] for authored reference vocabulary
//! - [`facade::refs`] for witness catalog vocabulary
//! - [`facade::anchor_selection`] for Query-native anchor and witness selection
//!   declaration entry and fact-delivery surfaces
//! - [`facade::anchor_binding`] for Query-native anchor binding declaration,
//!   evidence, identity, and projection surfaces
//! - [`facade::binding`] for Query-native primitive binding declaration,
//!   evidence, identity, and projection surfaces
//! - [`facade::rebinding`] for Query-native rebinding declaration, evidence,
//!   projection, and runtime-posture surfaces
//! - [`facade::placement`] for declarative placement vocabulary consumed by
//!   Query-native families
//! - [`facade::bindings`] for shared binding-site and anchor-carrier vocabulary
//! - [`facade::neighborhood`] for grouped local-neighborhood and topology
//!   replacement workflow surfaces
//! - [`facade::continuation`] for rebinding signal-compatibility and
//!   continuation workflow surfaces
//! - [`facade::inspection`] for retained historical, branch-local, and replay
//!   geometry surfaces
//! - [`facade::projection`] for receipt-backed geometry projection consumption
//! - [`facade::recovery`] for typed geometry recovery actions
//! - [`facade::planar_contracts`] for M6 planar admission vocabulary
//! - [`facade::planar_boolean_common_plane`] for pair-level common-plane
//!   eligibility receipts over certified planar support
//! - [`facade::planar_predicates`] for Query-native exact planar predicate
//!   authority backed by `worth-math` certified predicates
//! - [`facade::planar_predicate_consumption`] for Query-native validation that
//!   downstream planar classifications consumed `worth-math` predicate
//!   certificates rather than local substitutes
//! - [`facade::planar_structural_identity`] for Query-native planar structural
//!   identity fingerprints over boolean-readiness receipts and canonical
//!   transform basis
//! - [`facade::planar_motion_posture`] for Query-native retained planar
//!   movement, rotation, reorientation, cancellation, signal-compatibility, and
//!   continuation posture
//! - [`facade::planar_topology_contract`] for Query-native topology-to-spatial
//!   completeness receipts consumed before planar identity and boolean readiness
//! - [`facade::planar_retained_facts`] for Query-native retained planar fact
//!   replay over boolean-readiness, structural identity, motion posture, and
//!   topology completeness receipts
//! - [`facade::planar_precision`] for Query-native planar precision basis
//!   certification around retained predicate receipts
//! - [`facade::planar_local_frame`] for Query-native planar local-frame
//!   certificates consumed by later planar projection work
//! - [`facade::planar_projection`] for Query-native certified plane-to-2D
//!   point projection over retained local-frame certificates
//! - [`facade::planar_segment_segment`] for Query-native certified projected
//!   segment contact classification
//! - [`facade::workload_inventory`] for M6.5 seed and fixture classification
//!   before reusable workload construction
//! - [`facade::support`] for public family inventory and applicability posture
//! - [`facade::tolerance`] for tolerance and precision certification families
//!
//! Certification and test harnesses should use the same family-owned
//! declaration and fact surfaces as ordinary runtime code rather than a
//! parallel support namespace.

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(dead_code, unused_imports))]

#[cfg(test)]
extern crate self as worth_spatial;

mod anchor_selection;
mod authored_refs;
mod bindings;
pub mod certification;
mod placement;
mod planar_contracts;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_diagnostics/mod.rs"]
mod public_api_evidence_lookup_diagnostics;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_public_closeout/mod.rs"]
mod public_api_evidence_lookup_public_closeout;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_query_consumer_kit/mod.rs"]
mod public_api_evidence_lookup_query_consumer_kit;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_query_surface_matrix/mod.rs"]
mod public_api_evidence_lookup_query_surface_matrix;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_source_firewall/mod.rs"]
mod public_api_evidence_lookup_source_firewall;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_stage_cutover/mod.rs"]
mod public_api_evidence_lookup_stage_cutover;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/evidence_lookup_workload_cutover/mod.rs"]
mod public_api_evidence_lookup_workload_cutover;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/planar_contract_bundle/mod.rs"]
mod public_api_planar_contract_bundle;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/planar_diagnostics/mod.rs"]
mod public_api_planar_diagnostics;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/planar_motion_posture/mod.rs"]
mod public_api_planar_motion_posture;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/planar_projection_consumption/mod.rs"]
mod public_api_planar_projection_consumption;
#[cfg(test)]
#[path = "certification/public_facade_contracts/contracts/spatial_compiled_product_family/mod.rs"]
mod public_api_spatial_compiled_product_family;
mod query_adoption;
mod query_aspect_contract;
mod replay_family_catalog;
pub mod replay_undo_semantic_graph;
mod spatial_compiled_product_family;
#[cfg(test)]
mod structure_guard;
#[cfg(test)]
mod test_support;
pub mod touched_graph_conflict;
mod undo_family_catalog;
mod witness_resolution;
mod workload_platform;

pub mod facade;

mod trusted_boolean_evidence_authority {
    pub trait Seal {}
}
