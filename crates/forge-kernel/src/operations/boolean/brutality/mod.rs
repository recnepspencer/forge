//! 🔥 PLANAR BRUTALITY SUITE — Phase 0–2 Stress Tests
//!
//! DOMAIN: Comprehensive stress testing for predicates, coincidence handling,
//! boolean splitting, topology integrity, determinism, fuzzing, and stability.
//!
//! Refactored from monolith to modular suite.

mod predicates;
mod coincidence;
mod splitting;
mod integrity;
mod determinism;
mod fuzzing;
mod sliver;
mod features;
mod serialization;
mod tracing;
mod trace_dump;
mod tier1_manifold;
mod tier2_numerical;
mod tier3_adversarial;
mod deep_chains;
mod performance;
mod mb1_coplanar_apocalypse;
mod mb2_menger_graze;
mod mb3_singularity_star;
mod mb4_thin_labyrinth;
mod mb5_cancellation_chain;
mod mb6_halfspace_storm;
mod mb7_micro_feature_avalanche;
mod mb8_ultimate_degeneracy;

pub use predicates::*;
pub use coincidence::*;
pub use splitting::*;
pub use integrity::*;
pub use determinism::*;
pub use fuzzing::*;
pub use sliver::*;
pub use features::*;
pub use serialization::*;
pub use tracing::*;
