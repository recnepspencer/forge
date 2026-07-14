mod execution;
mod inputs;

pub use execution::{
    admit_security_scope_fixture, execute_security_scope_harness_replay_with_physical_replay,
    execute_security_scope_harness_scenario, SecurityScopeFixtureAuthority,
    SecurityScopeHarnessExecution, SecurityScopeHarnessReplayExecution,
};
pub use inputs::{
    security_scope_drift_scenario, security_scope_metadata_preservation_scenarios,
    security_scope_metadata_preserved_scenario, security_scope_missing_authenticity_scenario,
    security_scope_replayed_custody_scenario, security_scope_stale_key_scenario,
    security_scope_wrong_tenant_scenario,
};
