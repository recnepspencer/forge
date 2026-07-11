//! Named harness topology for Store certification and replay scenarios.
//!
//! `production_facade` contains helpers that exercise production-owned
//! capabilities through their public lifecycle. `test_authority` contains the
//! synthetic courtroom-only witnesses and shortcut attempts that exist only to
//! falsify production topology.

pub mod fixtures;
mod lsm_execution_fixture;
mod milestone;
pub mod physical_reference;
pub mod physical_simulation;
pub mod production_facade;
pub mod test_authority;

pub use milestone::s8_layout_access::{
    baseline_btree_probe_slot, deterministic_baseline_btree_witness,
    execute_s8_layout_runtime_receipt,
};
pub use production_facade::*;
pub use lsm_execution_fixture::execute_baseline_lsm_persisted_fixture;
