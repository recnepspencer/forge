use super::*;
use crate::identity::hash_parts;
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
        admission.query_declaration_digest(),
        lowering.query_declaration_digest()
    );
    assert_eq!(
        admission.bridge_declaration_digest(),
        lowering.bridge_declaration_digest()
    );
    assert_eq!(
        admission.basis_binding_digest(),
        lowering.basis_request().digest()
    );
    assert_eq!(
        admission.signal_strategy_digest(),
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
    let mut expected_parts = vec![
        "query_subscription_admission_artifact_v1".to_string(),
        format!("query_declaration:{}", admission.query_declaration_digest()),
        format!(
            "bridge_declaration:{}",
            admission.bridge_declaration_digest()
        ),
        format!("basis:{}", admission.basis_binding_digest()),
        format!("signal_strategy:{}", admission.signal_strategy_digest()),
        format!("diagnostics:{}", admission.diagnostics().digest()),
        format!("support:{}", admission.support_profile().digest()),
        format!(
            "budget:declaration:{}",
            admission.admission_budget().declaration_input_width_limit()
        ),
        format!(
            "budget:bridge:{}",
            admission.admission_budget().bridge_plan_width_limit()
        ),
        format!(
            "budget:basis:{}",
            admission.admission_budget().basis_binding_width_limit()
        ),
        format!(
            "budget:signal:{}",
            admission.admission_budget().signal_strategy_width_limit()
        ),
        format!(
            "budget:activation:{}",
            admission.admission_budget().activation_input_width_limit()
        ),
        format!(
            "counter:family_selection:{}",
            counters.family_selection_count()
        ),
        format!("counter:family_denial:{}", counters.family_denial_count()),
        format!(
            "counter:family_registry_lookup:{}",
            counters.family_registry_lookup_count()
        ),
        format!(
            "counter:view_family_registry_lookup:{}",
            counters.view_family_registry_lookup_count()
        ),
        format!(
            "counter:equivalence_digest_part:{}",
            counters.equivalence_digest_part_count()
        ),
        format!(
            "counter:admission_dimension_denial:{}",
            counters.admission_dimension_denial_count()
        ),
        format!(
            "counter:work_budget_denial:{}",
            counters.work_budget_denial_count()
        ),
        format!(
            "counter:unknown_cost_denial:{}",
            counters.unknown_cost_denial_count()
        ),
        format!(
            "counter:raw_cdc_fallback_denial:{}",
            counters.raw_cdc_fallback_denial_count()
        ),
        format!(
            "counter:host_observer_inference_denial:{}",
            counters.host_observer_inference_denial_count()
        ),
        format!(
            "counter:relationship_proof_drift_denial:{}",
            counters.relationship_proof_drift_denial_count()
        ),
        format!("counter:declaration:{}", counters.declaration_count()),
        format!(
            "counter:declaration_denial:{}",
            counters.declaration_denial_count()
        ),
        format!("counter:declared_slice:{}", counters.declared_slice_count()),
        format!(
            "counter:deduplicated_slice:{}",
            counters.deduplicated_slice_count()
        ),
        format!(
            "counter:slice_deduplication_input:{}",
            counters.slice_deduplication_input_count()
        ),
        format!(
            "counter:slice_sort_comparison:{}",
            counters.slice_sort_comparison_count()
        ),
        format!(
            "counter:masked_slice_denial:{}",
            counters.masked_slice_denial_count()
        ),
        format!(
            "counter:delivery_intent_denial:{}",
            counters.delivery_intent_denial_count()
        ),
        format!(
            "counter:declaration_digest_part:{}",
            counters.declaration_digest_part_count()
        ),
        format!(
            "counter:bridge_lowering:{}",
            counters.bridge_lowering_count()
        ),
        format!(
            "counter:bridge_family_denial:{}",
            counters.bridge_family_denial_count()
        ),
        format!(
            "counter:bridge_fallback_denial:{}",
            counters.bridge_fallback_denial_count()
        ),
        format!(
            "counter:bridge_family_registry_lookup:{}",
            counters.bridge_family_registry_lookup_count()
        ),
        format!("counter:bridge_slice:{}", counters.bridge_slice_count()),
        format!(
            "counter:bridge_slice_denial:{}",
            counters.bridge_slice_denial_count()
        ),
        format!(
            "counter:bridge_slice_registry_lookup:{}",
            counters.bridge_slice_registry_lookup_count()
        ),
        format!(
            "counter:basis_binding_request:{}",
            counters.basis_binding_request_count()
        ),
        format!(
            "counter:basis_binding_denial:{}",
            counters.basis_binding_denial_count()
        ),
        format!(
            "counter:signal_strategy_request:{}",
            counters.signal_strategy_request_count()
        ),
        format!("counter:admission:{}", counters.admission_count()),
        format!(
            "counter:admission_denial:{}",
            counters.admission_denial_count()
        ),
        format!(
            "counter:durable_overclaim_denial:{}",
            counters.durable_overclaim_denial_count()
        ),
        format!(
            "counter:activation_input:{}",
            counters.activation_input_count()
        ),
        format!(
            "counter:active_state_allocation_denial:{}",
            counters.active_state_allocation_denial_count()
        ),
        format!(
            "counter:declaration_time_checkpoint_denial:{}",
            counters.declaration_time_checkpoint_denial_count()
        ),
        format!(
            "counter:scratch_allocation:{}",
            counters.scratch_allocation_count()
        ),
        format!(
            "counter:forbidden_heap_allocation_denial:{}",
            counters.forbidden_heap_allocation_denial_count()
        ),
    ];

    assert_eq!(admission.admission_digest(), hash_parts(&expected_parts));
    expected_parts.pop();
    assert_ne!(admission.admission_digest(), hash_parts(&expected_parts));
}

#[test]
fn activation_input_is_prepared_only_from_admitted_subscription_artifact() {
    let lowering = lowering_for(LiveQueryFamily::Detail, None);
    let admission = admit_query_subscription(lowering, roomy_admission_budget()).unwrap();
    let admission_digest = admission.admission_digest().to_string();
    let query_declaration_digest = admission.query_declaration_digest().to_string();
    let bridge_declaration_digest = admission.bridge_declaration_digest().to_string();
    let basis_binding_digest = admission.basis_binding_digest().to_string();
    let signal_strategy_digest = admission.signal_strategy_digest().to_string();

    let activation = prepare_subscription_activation(admission);

    assert_eq!(activation.admission_digest(), admission_digest);
    assert_eq!(
        activation.query_declaration_digest(),
        query_declaration_digest
    );
    assert_eq!(
        activation.bridge_declaration_digest(),
        bridge_declaration_digest
    );
    assert_eq!(activation.basis_binding_digest(), basis_binding_digest);
    assert_eq!(activation.signal_strategy_digest(), signal_strategy_digest);
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
