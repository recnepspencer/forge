mod fixtures;
mod scenarios;

pub(crate) use fixtures::SecurityScopeNativeHarnessFixture;
pub use scenarios::{
    security_scope_drift_scenario, security_scope_metadata_preservation_scenarios,
    security_scope_metadata_preserved_scenario, security_scope_missing_authenticity_scenario,
    security_scope_replayed_custody_scenario, security_scope_stale_key_scenario,
    security_scope_wrong_tenant_scenario,
};
