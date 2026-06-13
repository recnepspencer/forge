use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

fn roomy_admission_budget() -> QuerySubscriptionAdmissionBudget {
    QuerySubscriptionAdmissionBudget::admitted(1, 1, 1, 1, 1)
}

fn lowering_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> BridgeSubscriptionLoweringPlan {
    let input = LiveQueryAdmissionArtifact::for_test(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
    );
    let selection = select_query_subscription_family(input, roomy_budget()).unwrap();
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap()
}

#[test]
fn bridge_lowering_admits_to_runtime_backed_subscription_artifact() {
    let lowering = lowering_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let admission = admit_query_subscription(lowering.clone(), roomy_admission_budget()).unwrap();

    assert_eq!(
        admission.query_declaration_for_reporting(),
        lowering.query_declaration_for_reporting()
    );
    assert_eq!(
        admission.bridge_declaration_for_reporting(),
        lowering.bridge_declaration_for_reporting()
    );
    assert_eq!(
        admission.basis_binding_for_reporting(),
        lowering.basis_request().digest()
    );
    assert_eq!(
        admission.signal_strategy_for_reporting(),
        lowering.signal_strategy_request().digest()
    );
    assert_eq!(admission.counters().admission_count(), 1);
    assert_eq!(admission.counters().admission_denial_count(), 0);
    assert_eq!(admission.counters().activation_input_count(), 0);
    assert_eq!(admission.counters().bridge_lowering_count(), 1);
    assert_eq!(
        admission.counters().bridge_family_registry_lookup_count(),
        1
    );
    assert_eq!(
        admission.counters().bridge_slice_count(),
        lowering.bridge_slices().len() as u64
    );
    assert_eq!(
        admission.counters().bridge_slice_registry_lookup_count(),
        lowering.bridge_slices().len() as u64
    );
    assert_eq!(admission.counters().basis_binding_request_count(), 1);
    assert_eq!(admission.counters().signal_strategy_request_count(), 1);
    assert_eq!(
        admission.diagnostics().stage(),
        &QuerySubscriptionAdmissionDiagnosticStage::RuntimeBackedAdmission
    );
    assert_eq!(
        admission.diagnostics().outcome(),
        &QuerySubscriptionAdmissionDiagnosticOutcome::Admitted
    );
    assert_eq!(
        admission.support_profile().runtime_backed_support(),
        &QuerySubscriptionRuntimeBackedSupport::Admitted
    );
    assert_eq!(
        admission.support_profile().active_lifecycle_support(),
        &QuerySubscriptionActiveLifecycleSupport::Admitted
    );
    assert_eq!(
        admission.support_profile().lifecycle_closeout_support(),
        &QuerySubscriptionLifecycleCloseoutSupport::Admitted
    );
    assert_eq!(
        admission.support_profile().durable_support(),
        &QuerySubscriptionDurableSupport::ExplicitDebt
    );
}

#[test]
fn admission_digest_binds_exact_counter_evidence() {
    let lowering = lowering_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let counters = admission.counters();

    assert_eq!(
        admission.evidence_identity().as_str(),
        admission
            .recomputed_evidence_identity(&counters.evidence_identity())
            .as_str()
    );

    let mut altered_counters = counters.clone();
    altered_counters.forbidden_heap_allocation_denial_count =
        counters.forbidden_heap_allocation_denial_count() + 1;
    assert_ne!(
        admission.evidence_identity().as_str(),
        admission
            .recomputed_evidence_identity(&altered_counters.evidence_identity())
            .as_str()
    );
}

#[test]
fn activation_input_is_prepared_only_from_admitted_subscription_artifact() {
    let lowering = lowering_for(LiveQueryFamily::Detail, None);
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let admission_digest = admission.admission_for_reporting().to_string();
    let query_declaration_digest = admission.query_declaration_for_reporting().to_string();
    let bridge_declaration_digest = admission.bridge_declaration_for_reporting().to_string();
    let basis_binding_digest = admission.basis_binding_for_reporting().to_string();
    let signal_strategy_digest = admission.signal_strategy_for_reporting().to_string();

    let activation = prepare_subscription_activation(admission);

    assert_eq!(activation.admission_for_reporting(), admission_digest);
    assert_eq!(
        activation.query_declaration_for_reporting(),
        query_declaration_digest
    );
    assert_eq!(
        activation.bridge_declaration_for_reporting(),
        bridge_declaration_digest
    );
    assert_eq!(activation.basis_binding_for_reporting(), basis_binding_digest);
    assert_eq!(activation.signal_strategy_for_reporting(), signal_strategy_digest);
    assert_eq!(activation.counters().admission_count(), 1);
    assert_eq!(activation.counters().activation_input_count(), 1);
    assert_eq!(
        activation.counters().active_state_allocation_denial_count(),
        0
    );
}

#[test]
fn admission_budget_exhaustion_denies_before_admission_artifact_exists() {
    let lowering = lowering_for(LiveQueryFamily::Detail, None);
    let error = admit_query_subscription(
        lowering,
        QuerySubscriptionAdmissionBudget::admitted(0, 1, 1, 1, 1),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionAdmissionDenialKind::AdmissionBudgetExceeded
    );
    assert_eq!(error.counters().admission_count(), 0);
    assert_eq!(error.counters().admission_denial_count(), 1);
    assert_eq!(error.counters().work_budget_denial_count(), 1);
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
}

#[test]
fn durable_reload_overclaim_denies_before_activation_input_exists() {
    let lowering = lowering_for(LiveQueryFamily::Detail, None);
    let error = admit_query_subscription(
        lowering,
        roomy_admission_budget().with_durable_reload_request(),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionAdmissionDenialKind::DurableReloadOverclaim
    );
    assert_eq!(error.counters().admission_count(), 0);
    assert_eq!(error.counters().durable_overclaim_denial_count(), 1);
    assert_eq!(
        error.counters().declaration_time_checkpoint_denial_count(),
        1
    );
    assert_eq!(error.counters().activation_input_count(), 0);
    assert_eq!(
        error.diagnostics().stage(),
        &QuerySubscriptionAdmissionDiagnosticStage::DurableReloadOverclaim
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
}

#[test]
fn active_lifecycle_allocation_request_denies_during_admission() {
    let lowering = lowering_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let error = admit_query_subscription(
        lowering,
        roomy_admission_budget().with_active_lifecycle_allocation_request(),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionAdmissionDenialKind::ActiveLifecycleAllocationForbidden
    );
    assert_eq!(error.counters().admission_count(), 0);
    assert_eq!(error.counters().active_state_allocation_denial_count(), 1);
    assert_eq!(error.counters().activation_input_count(), 0);
    assert_eq!(
        error.diagnostics().stage(),
        &QuerySubscriptionAdmissionDiagnosticStage::ActiveLifecycleAllocation
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
}
