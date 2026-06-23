use super::*;
use crate::identity_evolution::InspectorIdentityClassification;
use crate::saved_query::{
    evaluate_saved_query_reuse, freeze_direct_saved_query, SavedQueryFreezeContext,
    SavedQueryReuseDescriptor, SavedQueryReuseOutcome,
};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};

#[test]
fn identity_aware_inspector_freeze_captures_identity_contract() {
    let direct = direct_detail();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(
                &direct,
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    forge_foundational::facade::AspectKey::new("profile").unwrap(),
                    InspectorIdentityClassification::AdvisoryCandidates,
                ),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let saved = freeze_direct_saved_query(
        &direct,
        &view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();

    assert_eq!(
        saved.metadata().identity_consumption().classification(),
        Some(InspectorIdentityClassification::AdvisoryCandidates)
    );
    assert_eq!(
        saved.metadata().inspector_identity_classification(),
        Some(InspectorIdentityClassification::AdvisoryCandidates)
    );
    assert!(!saved
        .metadata()
        .identity_consumption_digest()
        .as_str()
        .is_empty());
}

#[test]
fn identity_aware_observed_inspector_freeze_captures_summary_contract() {
    let direct = direct_detail();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(
                &direct,
                ViewShapeDescriptor::identity_aware_inspector_detail_observed(),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let saved = freeze_direct_saved_query(
        &direct,
        &view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();

    assert_eq!(
        saved.metadata().view_shape_family().as_str(),
        "inspector_detail_observed"
    );
    assert_eq!(
        saved.metadata().identity_consumption(),
        &crate::view_shape::ViewShapeIdentityConsumption::inspector_identity_summary()
    );
    assert_eq!(saved.metadata().inspector_identity_classification(), None);
    assert!(!saved
        .metadata()
        .identity_consumption_digest()
        .as_str()
        .is_empty());
}

#[test]
fn identity_aware_inspector_reuse_requires_fresh_freeze_on_contract_change() {
    let direct = direct_detail();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(
                &direct,
                ViewShapeDescriptor::identity_aware_inspector_detail_focused(
                    forge_foundational::facade::AspectKey::new("profile").unwrap(),
                    InspectorIdentityClassification::AdvisoryCandidates,
                ),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let saved = freeze_direct_saved_query(
        &direct,
        &view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();
    let descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        saved.metadata().basis_family().cloned(),
        saved.metadata().template_binding_digest().cloned(),
        saved.metadata().template_slot_count(),
        saved.metadata().view_shape_digest().clone(),
        saved.metadata().view_shape_family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        saved.metadata().support_profile_digest().to_string(),
        saved.metadata().capability_family_identity().to_string(),
    )
    .with_identity_consumption(
        crate::view_shape::ViewShapeIdentityConsumption::focused_inspector_identity_classification(
            InspectorIdentityClassification::AuthoritativeContinuity,
        ),
    );

    let outcome = evaluate_saved_query_reuse(&saved, &descriptor);
    let SavedQueryReuseOutcome::Admitted(decision) = outcome else {
        panic!("identity-aware inspector contract drift should require a fresh freeze");
    };
    assert_eq!(
        decision.overall(),
        crate::saved_query::SavedQueryRebindingLegality::LegalRequiresFreshFreeze
    );
}

#[test]
fn observed_and_focused_inspector_reuse_require_fresh_freeze_on_lane_change() {
    let direct = direct_detail();
    let observed_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::inspector_detail_observed()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let focused_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(
                &direct,
                ViewShapeDescriptor::inspector_detail_focused(
                    forge_foundational::facade::AspectKey::new("profile").unwrap(),
                ),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let saved = freeze_direct_saved_query(
        &direct,
        &observed_view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();
    let descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        saved.metadata().basis_family().cloned(),
        saved.metadata().template_binding_digest().cloned(),
        saved.metadata().template_slot_count(),
        focused_view.view_shape_digest().clone(),
        focused_view.family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        saved.metadata().support_profile_digest().to_string(),
        saved.metadata().capability_family_identity().to_string(),
    );

    let outcome = evaluate_saved_query_reuse(&saved, &descriptor);
    let SavedQueryReuseOutcome::Admitted(decision) = outcome else {
        panic!("observed-to-focused inspector drift should require a fresh freeze");
    };
    assert_eq!(
        decision.overall(),
        crate::saved_query::SavedQueryRebindingLegality::LegalRequiresFreshFreeze
    );
}
