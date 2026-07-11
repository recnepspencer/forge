use forge_store_physical_certification::{
    S51SecurityScopeHarnessScenario, S51SecurityScopeHarnessSchedule,
};

pub const fn s5_1_security_scope_metadata_preserved_scenario() -> S51SecurityScopeHarnessScenario {
    S51SecurityScopeHarnessScenario::metadata_preserved(
        S51SecurityScopeHarnessSchedule::StableReadPlanAdmission,
    )
}

pub const fn s5_1_security_scope_metadata_preservation_scenarios(
) -> [S51SecurityScopeHarnessScenario; 4] {
    [
        S51SecurityScopeHarnessScenario::metadata_preserved(
            S51SecurityScopeHarnessSchedule::StableReadPlanAdmission,
        ),
        S51SecurityScopeHarnessScenario::metadata_preserved(
            S51SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
        ),
        S51SecurityScopeHarnessScenario::metadata_preserved(
            S51SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
        ),
        S51SecurityScopeHarnessScenario::metadata_preserved(
            S51SecurityScopeHarnessSchedule::RepairReadAdmission,
        ),
    ]
}

pub const fn s5_1_security_scope_drift_scenario() -> S51SecurityScopeHarnessScenario {
    S51SecurityScopeHarnessScenario::physical_scope_drift(
        S51SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
    )
}

pub const fn s5_1_security_scope_stale_key_scenario() -> S51SecurityScopeHarnessScenario {
    S51SecurityScopeHarnessScenario::stale_key_posture(
        S51SecurityScopeHarnessSchedule::CheckpointPublicationReplay,
    )
}

pub const fn s5_1_security_scope_wrong_tenant_scenario() -> S51SecurityScopeHarnessScenario {
    S51SecurityScopeHarnessScenario::wrong_tenant_scope(
        S51SecurityScopeHarnessSchedule::StableReadPlanAdmission,
    )
}

pub const fn s5_1_security_scope_missing_authenticity_scenario() -> S51SecurityScopeHarnessScenario
{
    S51SecurityScopeHarnessScenario::missing_authenticity_requirement(
        S51SecurityScopeHarnessSchedule::RootSwapBeforeLogicalDecode,
    )
}

pub const fn s5_1_security_scope_replayed_custody_scenario() -> S51SecurityScopeHarnessScenario {
    S51SecurityScopeHarnessScenario::replayed_custody_posture(
        S51SecurityScopeHarnessSchedule::RepairReadAdmission,
    )
}
