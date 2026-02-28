//! Topology operations — all mutation primitives and composite algorithms.
//!
//! DOMAIN: Topology mutation through Euler operators, compound algorithms,
//! and category-specific operator families.
//!
//! Core infrastructure (always at top level):
//! - `operator`: `EulerOperator` trait and `apply_op` runner
//! - `euler`: Classic Euler operator primitives (MVF, SplitEdge, etc.)
//! - `algorithms`: Compound algorithms built from Euler primitives
//!
//! Category subdirectories (from operators-list.md §B–§N):
//! - `lifecycle`: Body/component/lump/shell lifecycle (§B)
//! - `entity_lifecycle`: Face/loop/edge/vertex lifecycle (§C)
//! - `boundary_editing`: Loop wiring primitives (§D)
//! - `non_manifold`: Radial-edge/uses, NMT sewing/gluing (§E)
//! - `regions`: Region/cellular topology (§F)
//! - `sheets_wires`: Sheet/wire/laminar topology (§G)
//! - `brep_coupling`: Parametric B-Rep coupling (§H)
//! - `degeneracy`: Degeneracy/collapse/singularity (§I)
//! - `boolean`: Boolean/imprint/intersection surgery (§J)
//! - `healing`: Sewing/healing/repair (§K)
//! - `construction`: Feature-level modeling (§L)
//! - `global_editing`: Global topology operations (§M)
//! - `transform`: Transform/copy/pattern (§N)

pub mod algorithms;
pub mod euler;
pub mod operator;

pub mod lifecycle;
pub mod entity_lifecycle;
pub mod boundary_editing;
pub mod non_manifold;
pub mod regions;
pub mod sheets_wires;
pub mod brep_coupling;
pub mod degeneracy;
pub mod boolean;
pub mod healing;
pub mod construction;
pub mod global_editing;
pub mod transform;
