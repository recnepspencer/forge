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
pub mod invariant_id;
pub mod invariant_group;

pub(crate) mod shared;
pub(crate) mod structural;
pub mod validate;

pub mod reference_integrity;
pub mod loop_wiring;
pub mod radial_edge;
pub mod vertex_disk;
pub mod shell_closure;
pub mod region_cellular;
pub mod euler_genus;
pub mod degeneracy;
pub mod parametric_binding;
pub mod intersection_graph;
pub mod numerical_predicate;
pub mod determinism;
pub mod cache_index;
pub mod persistent_naming;
pub mod import_sanity;

pub use facade::*;
