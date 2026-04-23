use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

fn declaration_for(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
) -> QuerySubscriptionDeclarationArtifact {
    declaration_for_basis(
        live_family,
        view_family,
        QuerySubscriptionBasisPosture::CurrentHead,
    )
}

fn declaration_for_basis(
    live_family: LiveQueryFamily,
    view_family: Option<LiveViewShapeFamily>,
    basis_posture: QuerySubscriptionBasisPosture,
) -> QuerySubscriptionDeclarationArtifact {
    let input = LiveQueryAdmissionArtifact::for_test_with_basis(
        live_family,
        view_family,
        QuerySubscriptionConstructionSource::FacadeLive,
        basis_posture,
    );
    let selection = select_query_subscription_family(input, roomy_budget()).unwrap();
    declare_query_subscription(selection, roomy_slice_budget()).unwrap()
}

#[test]
fn every_query_family_lowers_to_explicit_bridge_family_and_slices() {
    let cases = [
        (
            LiveQueryFamily::Detail,
            None,
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![BridgeSubscriptionSliceKind::ProjectedField; 2],
        ),
        (
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::Table),
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::Membership,
                BridgeSubscriptionSliceKind::Ordering,
            ],
        ),
        (
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::KanbanGrouped),
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::Membership,
                BridgeSubscriptionSliceKind::Ordering,
                BridgeSubscriptionSliceKind::Grouping,
                BridgeSubscriptionSliceKind::ViewMetadata,
            ],
        ),
        (
            LiveQueryFamily::Detail,
            Some(LiveViewShapeFamily::InspectorDetailFocused),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::ViewMetadata,
            ],
        ),
        (
            LiveQueryFamily::BoundedMaterialization,
            None,
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::Membership,
                BridgeSubscriptionSliceKind::Ordering,
                BridgeSubscriptionSliceKind::RelationScope,
            ],
        ),
    ];

    for (live_family, view_family, expected_family, expected_slices) in cases {
        let declaration = declaration_for(live_family, view_family);
        let plan =
            lower_query_subscription_to_bridge(declaration, roomy_lowering_budget()).unwrap();

        assert_eq!(plan.bridge_family(), &expected_family);
        assert_eq!(plan.bridge_slices(), expected_slices.as_slice());
        assert_eq!(plan.counters().bridge_lowering_count(), 1);
        assert_eq!(plan.counters().bridge_family_registry_lookup_count(), 1);
        assert_eq!(
            plan.counters().bridge_slice_count(),
            expected_slices.len() as u64
        );
        assert_eq!(
            plan.counters().bridge_slice_registry_lookup_count(),
            expected_slices.len() as u64
        );
        assert_eq!(plan.counters().basis_binding_request_count(), 1);
        assert_eq!(plan.counters().signal_strategy_request_count(), 1);
    }
}

#[test]
fn equivalent_declarations_lower_to_identical_bridge_digest() {
    let declaration = |source| {
        let input = LiveQueryAdmissionArtifact::for_test(LiveQueryFamily::Detail, None, source);
        let selection = select_query_subscription_family(input, roomy_budget()).unwrap();
        declare_query_subscription(selection, roomy_slice_budget()).unwrap()
    };

    let direct = lower_query_subscription_to_bridge(
        declaration(QuerySubscriptionConstructionSource::Direct),
        roomy_lowering_budget(),
    )
    .unwrap();
    let saved = lower_query_subscription_to_bridge(
        declaration(QuerySubscriptionConstructionSource::SavedExactReuse),
        roomy_lowering_budget(),
    )
    .unwrap();

    assert_eq!(
        direct.bridge_declaration_digest(),
        saved.bridge_declaration_digest()
    );
}

#[test]
fn basis_request_digest_changes_by_basis_posture() {
    let declaration = |basis, policy: &str, tenant: &str| {
        let input = LiveQueryAdmissionArtifact::for_test_with_context(
            LiveQueryFamily::Detail,
            None,
            QuerySubscriptionConstructionSource::FacadeLive,
            basis,
            Some(policy.to_string()),
            Some(tenant.to_string()),
            Some("relationship-proof".to_string()),
            QuerySubscriptionRelationshipProofPosture::Admitted,
        );
        let selection = select_query_subscription_family(input, roomy_budget()).unwrap();
        declare_query_subscription(selection, roomy_slice_budget()).unwrap()
    };

    let current = lower_query_subscription_to_bridge(
        declaration(
            QuerySubscriptionBasisPosture::CurrentHead,
            "policy",
            "tenant",
        ),
        roomy_lowering_budget(),
    )
    .unwrap();
    let branch = lower_query_subscription_to_bridge(
        declaration(
            QuerySubscriptionBasisPosture::BranchHead,
            "policy",
            "tenant",
        ),
        roomy_lowering_budget(),
    )
    .unwrap();
    let snapshot = lower_query_subscription_to_bridge(
        declaration(
            QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot,
            "policy",
            "tenant",
        ),
        roomy_lowering_budget(),
    )
    .unwrap();
    let changed_policy = lower_query_subscription_to_bridge(
        declaration(
            QuerySubscriptionBasisPosture::CurrentHead,
            "policy-v2",
            "tenant",
        ),
        roomy_lowering_budget(),
    )
    .unwrap();
    let changed_tenant = lower_query_subscription_to_bridge(
        declaration(
            QuerySubscriptionBasisPosture::CurrentHead,
            "policy",
            "tenant-beta",
        ),
        roomy_lowering_budget(),
    )
    .unwrap();

    assert_ne!(
        current.basis_request().digest(),
        branch.basis_request().digest()
    );
    assert_ne!(
        current.basis_request().digest(),
        snapshot.basis_request().digest()
    );
    assert_ne!(
        current.basis_request().digest(),
        changed_policy.basis_request().digest()
    );
    assert_ne!(
        current.basis_request().digest(),
        changed_tenant.basis_request().digest()
    );
    assert_eq!(
        current.basis_request().source_declaration_digest(),
        current.query_declaration_digest()
    );
}

#[test]
fn unsupported_bridge_family_slice_and_basis_deny_before_lowering_plan_exists() {
    let declaration = declaration_for(LiveQueryFamily::BoundedMaterialization, None);
    let family_error = lower_query_subscription_to_bridge(
        declaration.clone(),
        roomy_lowering_budget().without_bridge_family_support(),
    )
    .unwrap_err();
    assert_eq!(
        family_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BridgeFamilyUnsupported
    );
    assert_eq!(family_error.counters().bridge_lowering_count(), 0);
    assert_eq!(family_error.counters().bridge_family_denial_count(), 1);

    let slice_error = lower_query_subscription_to_bridge(
        declaration.clone(),
        roomy_lowering_budget()
            .without_bridge_slice_support(BridgeSubscriptionSliceKind::RelationScope),
    )
    .unwrap_err();
    assert_eq!(
        slice_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BridgeSliceUnsupported
    );
    assert_eq!(slice_error.counters().bridge_lowering_count(), 0);
    assert_eq!(slice_error.counters().bridge_slice_denial_count(), 1);

    let historical_input = LiveQueryAdmissionArtifact::for_test_with_basis(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionConstructionSource::FacadeLive,
        QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot,
    );
    let historical_selection =
        select_query_subscription_family(historical_input, roomy_budget()).unwrap();
    let historical_declaration =
        declare_query_subscription(historical_selection, roomy_slice_budget()).unwrap();
    let basis_error = lower_query_subscription_to_bridge(
        historical_declaration,
        roomy_lowering_budget().without_historical_basis_support(),
    )
    .unwrap_err();
    assert_eq!(
        basis_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BasisBindingUnsupported
    );
    assert_eq!(basis_error.counters().basis_binding_denial_count(), 1);

    let preview_declaration = declaration_for_basis(
        LiveQueryFamily::Detail,
        None,
        QuerySubscriptionBasisPosture::PreviewScoped,
    );
    let preview_error =
        lower_query_subscription_to_bridge(preview_declaration, roomy_lowering_budget())
            .unwrap_err();
    assert_eq!(
        preview_error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BasisBindingUnsupported
    );
    assert_eq!(preview_error.counters().basis_binding_denial_count(), 1);
}

#[test]
fn bridge_lowering_budget_exhaustion_denies_before_plan_exists() {
    let declaration = declaration_for(
        LiveQueryFamily::OrderedCollection,
        Some(LiveViewShapeFamily::Table),
    );
    let error = lower_query_subscription_to_bridge(
        declaration,
        QuerySubscriptionBridgeLoweringBudget::admitted(1, 1, 8, 1, 1),
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::LoweringBudgetExceeded
    );
    assert_eq!(error.counters().bridge_lowering_count(), 0);
    assert_eq!(error.counters().work_budget_denial_count(), 1);
}

#[test]
fn bridge_fallback_lowering_is_explicitly_budget_gated() {
    let declaration = declaration_for(LiveQueryFamily::Detail, None)
        .with_bridge_posture(QuerySubscriptionBridgePosture::BridgeLoweringDeferred);

    let error = lower_query_subscription_to_bridge(declaration.clone(), roomy_lowering_budget())
        .unwrap_err();
    assert_eq!(
        error.denial_kind(),
        &QuerySubscriptionBridgeLoweringDenialKind::BridgeFallbackUnsupported
    );
    assert_eq!(error.counters().bridge_lowering_count(), 0);
    assert_eq!(error.counters().bridge_fallback_denial_count(), 1);

    let admitted = lower_query_subscription_to_bridge(
        declaration,
        roomy_lowering_budget().with_bridge_fallback_support(),
    )
    .unwrap();
    assert_eq!(admitted.counters().bridge_lowering_count(), 1);
}
