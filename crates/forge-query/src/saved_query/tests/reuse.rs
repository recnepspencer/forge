use super::*;
use crate::composition::{GuidedCompositionPath, QueryScopeDescriptor};
use crate::saved_query::{
    evaluate_saved_query_reuse, freeze_composed_saved_query, freeze_direct_saved_query,
    SavedQueryFailureClass, SavedQueryFreezeContext, SavedQueryReuseDescriptor,
    SavedQueryReuseOutcome,
};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};

#[test]
fn direct_and_composed_saved_queries_preserve_canonical_meaning() {
    let direct = direct_detail();
    let direct_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::detail()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let direct_saved = freeze_direct_saved_query(
        &direct,
        &direct_view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();

    let (.., expanded) = GuidedCompositionPath::expand_detail_scopes(
        crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::detail_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
        [QueryScopeDescriptor::predicate("noop", Vec::new())],
    )
    .unwrap();
    let composed = GuidedCompositionPath::canonicalize_expanded(expanded).unwrap();
    let composed_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            composed.canonical(),
            detail_schema_view(),
            admit_view_shape(composed.canonical(), ViewShapeDescriptor::detail()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();
    let composed_saved = freeze_composed_saved_query(
        &composed,
        &composed_view,
        SavedQueryFreezeContext::new("test-support", "query_composition"),
    )
    .unwrap();

    assert_eq!(
        direct_saved.metadata().canonical_query_digest(),
        composed_saved.metadata().canonical_query_digest()
    );
    assert_ne!(
        direct_saved.metadata().composition_digest(),
        composed_saved.metadata().composition_digest()
    );
}

#[test]
fn saved_query_reuse_denies_support_profile_drift() {
    let direct = direct_detail();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::detail()).unwrap(),
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
        "different-support",
        saved.metadata().capability_family_identity().to_string(),
    );

    let outcome = evaluate_saved_query_reuse(&saved, &descriptor);
    let SavedQueryReuseOutcome::Denied(denial) = outcome else {
        panic!("support profile drift should deny reuse");
    };
    assert_eq!(
        denial.failure_class(),
        &SavedQueryFailureClass::IllegalSemanticDrift
    );
}

#[test]
fn saved_query_reuse_denies_basis_family_change() {
    let direct = direct_detail();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::detail()).unwrap(),
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
        Some(crate::query_context::QueryContextFamily::CurrentBranchHead),
        saved.metadata().template_binding_digest().cloned(),
        saved.metadata().template_slot_count(),
        saved.metadata().view_shape_digest().clone(),
        saved.metadata().view_shape_family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        saved.metadata().support_profile_digest().to_string(),
        saved.metadata().capability_family_identity().to_string(),
    );

    let outcome = evaluate_saved_query_reuse(&saved, &descriptor);
    let SavedQueryReuseOutcome::Denied(denial) = outcome else {
        panic!("basis family change should deny reuse");
    };
    assert_eq!(
        denial.failure_class(),
        &SavedQueryFailureClass::IllegalSemanticDrift
    );
}

#[test]
fn saved_query_reuse_denies_template_slot_set_change() {
    let direct = direct_detail();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::detail()).unwrap(),
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
        saved.metadata().template_slot_count() + 1,
        saved.metadata().view_shape_digest().clone(),
        saved.metadata().view_shape_family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        saved.metadata().support_profile_digest().to_string(),
        saved.metadata().capability_family_identity().to_string(),
    );

    let outcome = evaluate_saved_query_reuse(&saved, &descriptor);
    let SavedQueryReuseOutcome::Denied(denial) = outcome else {
        panic!("template slot set change should deny reuse");
    };
    assert_eq!(
        denial.failure_class(),
        &SavedQueryFailureClass::IllegalSemanticDrift
    );
}

#[test]
fn saved_query_reuse_requires_fresh_freeze_for_view_change() {
    let direct = direct_detail();
    let detail_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            detail_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::detail()).unwrap(),
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
        &detail_view,
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
        panic!("view-family change should require a fresh freeze, not deny reuse");
    };
    assert_eq!(
        decision.overall(),
        crate::saved_query::SavedQueryRebindingLegality::LegalRequiresFreshFreeze
    );
}
