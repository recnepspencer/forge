//! Topology validators — pure invariant checking.
//!
//! DOMAIN: Structural and semantic invariant validation.
//!
//! STRUCTURE:
//!   facade.rs          — Public API surface (§7)
//!   structural.rs      — Dispatcher: wires individual validators into `validate_topology`
//!   validate.rs        — `ValidationLevel` + integration tests
//!
//! Category subdirectories (§1–§15 from validators.md):
//!   reference_integrity/ — Pointer/ownership/orphan checks
//!   loop_wiring/         — Half-edge/loop wiring invariants
//!   radial_edge/         — Radial cycle invariants (NMT core)
//!   vertex_disk/         — Vertex-disk/umbrella invariants
//!   shell_closure/       — Shell/body closure and orientation
//!   euler_genus/         — Euler formula and genus checks
//!   cache_index/         — Cache/index consistency
//!   region_cellular/     — Region/cellular topology invariants
//!   degeneracy/          — Degeneracy classification
//!   parametric_binding/  — NURBS binding invariants
//!   intersection_graph/  — Intersection/imprint graph invariants
//!   numerical_predicate/ — Numerical/predicate pipeline
//!   determinism/         — Determinism validators
//!   persistent_naming/   — Persistent naming/selector
//!   import_sanity/       — Import sanity/soup recovery

pub mod facade;
pub mod group_policy_runtime;
pub mod invariant_group;
pub mod invariant_id;

pub(crate) mod shared;
pub(crate) mod structural;
pub mod validate;

pub mod cache_index;
pub mod contract_registry;
pub mod degeneracy;
pub mod determinism;
pub mod euler_genus;
pub mod import_sanity;
pub mod intersection_graph;
pub mod loop_wiring;
pub mod numerical_predicate;
pub mod parametric_binding;
pub mod persistent_naming;
pub mod radial_edge;
pub mod reference_integrity;
pub mod region_cellular;
pub mod shell_closure;
pub mod vertex_disk;

pub use facade::*;
