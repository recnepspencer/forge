//! Topology validators — pure invariant checking.
//!
//! DOMAIN: Structural and semantic invariant validation.
//!
//! Categories are organized by validator domain (matching `docs/topo/validators.md`).
//! Existing validators live in `structural` and `validate` until individually
//! migrated into their category subdirectory.
//!
//! - `structural`: Pure connectivity checks (twins, loops, Euler formula)
//! - `validate`: Public API — re-exports + `ValidationLevel`
//!
//! Category subdirectories (§1–§15 from validators.md):
//! - `reference_integrity`: Pointer/ownership/orphan checks
//! - `loop_wiring`: Half-edge/loop wiring invariants
//! - `radial_edge`: Radial cycle invariants (NMT core)
//! - `vertex_disk`: Vertex-disk/umbrella invariants
//! - `shell_closure`: Shell/body closure and orientation
//! - `region_cellular`: Region/cellular topology invariants
//! - `euler_genus`: Euler formula and genus checks
//! - `degeneracy`: Degeneracy classification
//! - `parametric_binding`: NURBS binding invariants
//! - `intersection_graph`: Intersection/imprint graph invariants
//! - `numerical_predicate`: Numerical/predicate pipeline
//! - `determinism`: Determinism validators
//! - `cache_index`: Cache/index consistency
//! - `persistent_naming`: Persistent naming/selector
//! - `import_sanity`: Import sanity/soup recovery

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
