use super::world::table_lowering;
use super::*;

#[test]
fn durable_reload_overclaim_carries_support_and_pipeline_diagnostics() {
    let lowering = table_lowering();

    let error = admit_query_subscription(
        lowering,
        QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1).with_durable_reload_request(),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionAdmissionDenialKind::DurableReloadOverclaim
    );
    assert_eq!(
        error.pipeline_diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::DurableReloadOverclaim
    );
    assert_eq!(
        error
            .pipeline_diagnostic()
            .counter_projection()
            .label()
            .as_str(),
        error.counters().counter_projection().label()
    );
    assert_eq!(
        error.diagnostics().stage(),
        &QuerySubscriptionAdmissionDiagnosticStage::DurableReloadOverclaim
    );
    assert_eq!(error.counters().admission_count(), 0);
    assert_eq!(error.counters().durable_overclaim_denial_count(), 1);
    assert_eq!(
        error.counters().declaration_time_checkpoint_denial_count(),
        1
    );
    assert_eq!(
        error.support_profile().runtime_backed_support(),
        &QuerySubscriptionRuntimeBackedSupport::Denied
    );
    assert_eq!(
        error.support_profile().active_lifecycle_support(),
        &QuerySubscriptionActiveLifecycleSupport::Denied
    );
    assert_eq!(
        error.support_profile().lifecycle_closeout_support(),
        &QuerySubscriptionLifecycleCloseoutSupport::Denied
    );
    assert_eq!(
        error.support_profile().durable_support(),
        &QuerySubscriptionDurableSupport::ExplicitDebt
    );
    assert_eq!(
        error.support_profile().source_projection().label(),
        error.pipeline_diagnostic().source_projection().label()
    );
    assert!(!error
        .support_profile()
        .profile_projection()
        .label()
        .is_empty());
}
