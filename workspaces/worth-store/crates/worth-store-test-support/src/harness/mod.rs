//! Named harness topology for Store certification and replay scenarios.
//!
//! `production_facade` contains helpers that exercise production-owned
//! capabilities through their public lifecycle. `test_authority` contains the
//! synthetic courtroom-only witnesses and shortcut attempts that exist only to
//! falsify production topology.

mod blob;
pub mod fixtures;
pub mod layout;
pub mod layout_evolution;
mod lsm_execution_fixture;
pub mod physical_isolation;
pub mod physical_reference;
pub mod physical_simulation;
mod pressure;
pub mod production_facade;
pub mod recovery;
mod security_scope;
pub mod test_authority;

pub use lsm_execution_fixture::{
    execute_baseline_lsm_membership_replacement_fixture, execute_baseline_lsm_persisted_fixture,
    execute_baseline_lsm_replay_source_fixture, execute_frontierless_lsm_replay_source_fixture,
    execute_lsm_compaction_reader_cutover_fixture, execute_lsm_replay_hostile_matrix,
    execute_repeated_lsm_membership_fixture, lsm_membership_replacement_crash_fixture,
    observe_lsm_owner_cases, substituted_lsm_base_is_rejected_before_compaction,
    ExecutedLsmCompactionFixture, LsmMembershipReplacementCrashFixture, LsmOwnerCaseObservations,
    LsmReplayHostileMatrix, RepeatedLsmMembershipFixture,
};
pub use production_facade::*;
