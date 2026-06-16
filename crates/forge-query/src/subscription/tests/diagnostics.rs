use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

use super::*;

#[test]
fn view_family_mismatch_carries_family_selection_diagnostic_evidence() {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::Detail,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );

    let error = select_query_subscription_family(live, roomy_budget()).unwrap_err();

    assert_eq!(
        error.failure_class(),
        &QuerySubscriptionFamilySelectionFailureClass::ViewFamilyLiveFamilyMismatch
    );
    assert_eq!(
        error.diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::ViewMismatch
    );
    assert_eq!(
        error.diagnostic().outcome(),
        &QuerySubscriptionDiagnosticOutcome::Denied
    );
    assert_eq!(
        error.diagnostic().counter_projection().label().as_str(),
        error.counters().counter_projection().label()
    );
    assert_eq!(error.counters().family_registry_lookup_count(), 1);
    assert_eq!(error.counters().view_family_registry_lookup_count(), 1);
    assert_eq!(error.counters().family_selection_count(), 0);
    assert_eq!(error.counters().family_denial_count(), 1);
    assert_eq!(error.counters().declaration_count(), 0);
    assert_eq!(error.counters().bridge_lowering_count(), 0);
    assert!(!error.diagnostic().source_projection().label().is_empty());
    assert!(!error.diagnostic().evidence_projection().label().is_empty());
}

#[test]
fn masked_detail_table_and_grouped_requests_deny_before_bridge_lowering() {
    for (
        live_family,
        view_family,
        expected_family,
        expected_ordering_width,
        expected_grouping_width,
    ) in [
        (
            LiveQueryFamily::Detail,
            None,
            QuerySubscriptionFamily::DetailExact,
            0,
            0,
        ),
        (
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::Table),
            QuerySubscriptionFamily::CollectionMembership,
            1,
            0,
        ),
        (
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::KanbanGrouped),
            QuerySubscriptionFamily::GroupedCollectionMembership,
            1,
            1,
        ),
    ] {
        let live = LiveQueryAdmissionArtifact::for_test(
            live_family,
            view_family,
            QuerySubscriptionConstructionSource::Direct,
        );
        let selection = select_query_subscription_family(live, roomy_budget()).unwrap();

        assert_eq!(selection.family(), &expected_family);
        assert_eq!(selection.ordering_width(), expected_ordering_width);
        assert_eq!(selection.grouping_width(), expected_grouping_width);

        let error = declare_query_subscription(
            selection,
            roomy_slice_budget().with_masked_slice_request_detected(),
        )
        .unwrap_err();

        assert_eq!(
            error.denial_kind(),
            &QuerySubscriptionDeclarationDenialKind::UnsupportedMaskedSlice
        );
        assert_eq!(
            error.diagnostic().stage(),
            &QuerySubscriptionDiagnosticStage::Declaration
        );
        assert_eq!(error.counters().declaration_count(), 0);
        assert_eq!(error.counters().declaration_denial_count(), 1);
        assert_eq!(error.counters().bridge_lowering_count(), 0);
        assert_eq!(error.counters().masked_slice_denial_count(), 1);
        assert_eq!(
            error.diagnostic().counter_projection().label().as_str(),
            error.counters().counter_projection().label()
        );
    }
}

#[test]
fn delivery_intent_denial_has_specific_diagnostic_stage() {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, roomy_budget()).unwrap();

    let error = declare_query_subscription(
        selection,
        roomy_slice_budget().without_delivery_intent_support(),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionDeclarationDenialKind::DeliveryIntentUnsupported
    );
    assert_eq!(
        error.diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::DeliveryIntent
    );
    assert_eq!(error.counters().delivery_intent_denial_count(), 1);
    assert_eq!(error.counters().declaration_denial_count(), 1);
    assert_eq!(error.counters().declaration_count(), 0);
    assert_eq!(error.counters().bridge_lowering_count(), 0);
    assert_eq!(
        error.diagnostic().counter_projection().label().as_str(),
        error.counters().counter_projection().label()
    );
}

#[test]
fn bridge_family_slice_and_basis_denials_have_distinct_diagnostic_stages() {
    let table_declaration = table_declaration();
    let family_error = lower_query_subscription_to_bridge(
        table_declaration.clone(),
        roomy_lowering_budget().without_bridge_family_support(),
    )
    .unwrap_err();
    assert_eq!(
        family_error.diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::BridgeFamilyLowering
    );
    assert_eq!(
        family_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BridgeFamilyUnsupported
    );
    assert_eq!(family_error.counters().bridge_lowering_count(), 0);
    assert_eq!(family_error.counters().bridge_family_denial_count(), 1);
    assert_eq!(
        family_error.diagnostic().counter_projection().label().as_str(),
        family_error.counters().counter_projection().label()
    );

    let slice_error = lower_query_subscription_to_bridge(
        table_declaration.clone(),
        roomy_lowering_budget().without_bridge_slice_support(BridgeSubscriptionSliceKind::Ordering),
    )
    .unwrap_err();
    assert_eq!(
        slice_error.diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::BridgeSliceLowering
    );
    assert_eq!(
        slice_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BridgeSliceUnsupported
    );
    assert_eq!(slice_error.counters().bridge_lowering_count(), 0);
    assert_eq!(slice_error.counters().bridge_slice_denial_count(), 1);
    assert_eq!(
        slice_error.diagnostic().counter_projection().label().as_str(),
        slice_error.counters().counter_projection().label()
    );

    let live = LiveQueryAdmissionArtifact::for_test_with_basis(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::Direct,
        QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot,
    );
    let selection = select_query_subscription_family(live, roomy_budget()).unwrap();
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    let basis_error = lower_query_subscription_to_bridge(
        declaration,
        roomy_lowering_budget().without_historical_basis_support(),
    )
    .unwrap_err();
    assert_eq!(
        basis_error.diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::BasisBinding
    );
    assert_eq!(
        basis_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BasisBindingUnsupported
    );
    assert_eq!(basis_error.counters().bridge_lowering_count(), 0);
    assert_eq!(basis_error.counters().basis_binding_denial_count(), 1);
    assert_eq!(
        basis_error.diagnostic().counter_projection().label().as_str(),
        basis_error.counters().counter_projection().label()
    );
}

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
        error.pipeline_diagnostic().counter_projection().label().as_str(),
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
    assert!(!error.support_profile().profile_projection().label().is_empty());
}

#[test]
fn policy_tenant_and_relationship_proof_context_change_subscription_meaning() {
    let tenant_alpha =
        certified_subscription_identity("policy-alpha", "tenant-alpha", "proof-alpha");
    let tenant_beta = certified_subscription_identity("policy-alpha", "tenant-beta", "proof-alpha");
    let proof_beta = certified_subscription_identity("policy-alpha", "tenant-alpha", "proof-beta");

    assert_ne!(
        tenant_alpha.declaration_digest,
        tenant_beta.declaration_digest
    );
    assert_ne!(
        tenant_alpha.basis_request_digest,
        tenant_beta.basis_request_digest
    );
    assert_ne!(
        tenant_alpha.declaration_digest,
        proof_beta.declaration_digest
    );
    assert_ne!(
        tenant_alpha.basis_request_digest,
        proof_beta.basis_request_digest
    );
    assert_ne!(
        tenant_beta.declaration_digest,
        proof_beta.declaration_digest
    );
    assert_ne!(
        tenant_beta.basis_request_digest,
        proof_beta.basis_request_digest
    );
}

#[test]
fn relationship_proof_drift_denies_before_declaration_or_bridge_lowering() {
    let live = LiveQueryAdmissionArtifact::for_test_with_relationship_proof_posture(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::SavedExactReuse,
        QuerySubscriptionRelationshipProofPosture::Drifted,
    );

    let error = select_query_subscription_family(live, roomy_budget()).unwrap_err();

    assert_eq!(
        error.failure_class(),
        &QuerySubscriptionFamilySelectionFailureClass::RelationshipProofAdmissionDrift
    );
    assert_eq!(
        error.diagnostic().stage(),
        &QuerySubscriptionDiagnosticStage::RelationshipProofDrift
    );
    assert_eq!(error.counters().family_selection_count(), 0);
    assert_eq!(error.counters().family_registry_lookup_count(), 0);
    assert_eq!(error.counters().view_family_registry_lookup_count(), 0);
    assert_eq!(error.counters().family_denial_count(), 1);
    assert_eq!(error.counters().declaration_count(), 0);
    assert_eq!(error.counters().bridge_lowering_count(), 0);
    assert_eq!(error.counters().relationship_proof_drift_denial_count(), 1);
    assert_eq!(
        error.diagnostic().counter_projection().label().as_str(),
        error.counters().counter_projection().label()
    );
    assert!(error.message().contains("relationship proof posture"));
    assert!(!error.diagnostic().source_projection().label().is_empty());
}

fn table_declaration() -> QuerySubscriptionDeclarationArtifact {
    let live = LiveQueryAdmissionArtifact::for_test(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
        QuerySubscriptionConstructionSource::Direct,
    );
    let selection = select_query_subscription_family(live, roomy_budget()).unwrap();
    declare_query_subscription(selection, roomy_slice_budget()).unwrap()
}

fn table_lowering() -> BridgeSubscriptionLoweringPlan {
    lower_query_subscription_to_bridge(table_declaration(), roomy_lowering_budget()).unwrap()
}

#[derive(Debug)]
struct CertifiedSubscriptionIdentity {
    declaration_digest: String,
    basis_request_digest: String,
}

fn certified_subscription_identity(
    policy: &str,
    tenant: &str,
    proof: &str,
) -> CertifiedSubscriptionIdentity {
    let live = LiveQueryAdmissionArtifact::for_test_with_context(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::SavedExactReuse,
        QuerySubscriptionBasisPosture::CurrentHead,
        QuerySubscriptionFutureSelection::ordinary(),
        Some(policy.to_string()),
        Some(tenant.to_string()),
        Some(proof.to_string()),
        QuerySubscriptionRelationshipProofPosture::Admitted,
    );
    let selection = select_query_subscription_family(live, roomy_budget()).unwrap();
    let declaration = declare_query_subscription(selection, roomy_slice_budget()).unwrap();
    let declaration_digest = declaration.declaration_projection().label().to_string();
    let lowering =
        lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap();
    CertifiedSubscriptionIdentity {
        declaration_digest,
        basis_request_digest: lowering.basis_request().basis_binding_projection().label().to_string(),
    }
}
