//! Named harness topology for Store certification and replay scenarios.
//!
//! `production_facade` contains helpers that exercise production-owned
//! capabilities through their public lifecycle. `test_authority` contains the
//! synthetic courtroom-only witnesses and shortcut attempts that exist only to
//! falsify production topology.

#[cfg(feature = "certification-world")]
mod blob;
#[cfg(feature = "boundary-fixtures")]
pub mod fixtures;
#[cfg(feature = "layout-fixtures")]
pub mod layout;
#[cfg(feature = "layout-fixtures")]
pub mod layout_evolution;
#[cfg(feature = "certification-world")]
mod lsm_execution_fixture;
#[cfg(feature = "physical-isolation-fixtures")]
pub mod physical_isolation;
#[cfg(feature = "physical-isolation-fixtures")]
pub mod physical_reference;
#[cfg(feature = "physical-residency-fixtures")]
pub mod physical_residency;
#[cfg(feature = "certification-world")]
pub mod physical_simulation;
#[cfg(feature = "certification-world")]
mod pressure;
#[cfg(feature = "boundary-fixtures")]
pub mod production_facade;
#[cfg(feature = "physical-isolation-fixtures")]
pub mod recovery;
#[cfg(feature = "scheduling-fixtures")]
pub mod scheduling;
#[cfg(feature = "certification-world")]
mod security_scope;
#[cfg(feature = "physical-isolation-fixtures")]
pub mod test_authority;

#[cfg(feature = "certification-world")]
pub use lsm_execution_fixture::{
    execute_baseline_lsm_membership_replacement_fixture, execute_baseline_lsm_persisted_fixture,
    execute_baseline_lsm_replay_source_fixture, execute_frontierless_lsm_replay_source_fixture,
    execute_lsm_compaction_reader_cutover_fixture, execute_lsm_replay_hostile_matrix,
    execute_repeated_lsm_membership_fixture, lsm_membership_replacement_crash_fixture,
    observe_lsm_owner_cases, substituted_lsm_base_is_rejected_before_compaction,
    ExecutedLsmCompactionFixture, LsmMembershipReplacementCrashFixture, LsmOwnerCaseObservations,
    LsmReplayHostileMatrix, RepeatedLsmMembershipFixture,
};
#[cfg(feature = "boundary-fixtures")]
pub use production_facade::*;
