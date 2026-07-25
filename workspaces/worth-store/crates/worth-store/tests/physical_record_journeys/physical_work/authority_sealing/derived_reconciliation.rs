use super::assert_sources_exclude;

#[test]
fn derived_reconciliation_cannot_reacquire_physical_effect_authority() {
    assert_sources_exclude(
        "src/physical_runtime/instance/signal_owner",
        "physical-effect-no-retry",
        &[
            "PhysicalEffectRetryAfterDerivedRollback",
            "PhysicalExecutorCommand",
            "PhysicalWorkExecution",
            "PhysicalWorkExecutor",
        ],
    );
}
