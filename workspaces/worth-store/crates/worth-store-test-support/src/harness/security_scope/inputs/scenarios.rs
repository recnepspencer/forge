use worth_store_physical_certification::{
    SecurityScopeHarnessScenario, SecurityScopeHarnessSchedule,
};

pub const fn security_scope_metadata_preserved_scenario() -> SecurityScopeHarnessScenario {
    SecurityScopeHarnessScenario::metadata_preserved(
        SecurityScopeHarnessSchedule::StableReadPlanAdmission,
    )
}

pub const fn security_scope_metadata_preservation_scenarios() -> [SecurityScopeHarnessScenario; 4] {
    [
        SecurityScopeHarnessScenario::metadata_preserved(
            SecurityScopeHarnessSchedule::StableReadPlanAdmission,
        ),
        SecurityScopeHarnessScenario::metadata_preserved(
            SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
        ),
        SecurityScopeHarnessScenario::metadata_preserved(
            SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
        ),
        SecurityScopeHarnessScenario::metadata_preserved(
            SecurityScopeHarnessSchedule::RepairReadAdmission,
        ),
    ]
}

pub const fn security_scope_drift_scenario() -> SecurityScopeHarnessScenario {
    SecurityScopeHarnessScenario::physical_scope_drift(
        SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
    )
}

pub const fn security_scope_stale_key_scenario() -> SecurityScopeHarnessScenario {
    SecurityScopeHarnessScenario::stale_key_posture(
        SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
    )
}

pub const fn security_scope_wrong_tenant_scenario() -> SecurityScopeHarnessScenario {
    SecurityScopeHarnessScenario::wrong_tenant_scope(
        SecurityScopeHarnessSchedule::StableReadPlanAdmission,
    )
}

pub const fn security_scope_missing_authenticity_scenario() -> SecurityScopeHarnessScenario {
    SecurityScopeHarnessScenario::missing_authenticity_requirement(
        SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
    )
}

pub const fn security_scope_replayed_custody_scenario() -> SecurityScopeHarnessScenario {
    SecurityScopeHarnessScenario::replayed_custody_posture(
        SecurityScopeHarnessSchedule::RepairReadAdmission,
    )
}
