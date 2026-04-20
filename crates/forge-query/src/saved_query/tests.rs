use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
};
use crate::composition::{
    GuidedCompositionPath, QueryScopeDescriptor,
};
use crate::saved_query::{
    evaluate_saved_query_reuse, freeze_composed_saved_query, freeze_direct_saved_query,
    SavedQueryFailureClass, SavedQueryFreezeContext, SavedQueryPersistenceClaim,
    SavedQueryReuseDescriptor, SavedQueryReuseOutcome,
};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor,
};

fn detail_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "saved-query-detail",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn collection_schema_view() -> crate::schema_view::QuerySchemaView {
    crate::schema_view::QuerySchemaView::new(
        "saved-query-collection",
        [
            crate::schema_view::SchemaFieldView::new(
                "identity",
                "id",
                crate::schema_view::SchemaFieldKind::String,
            ),
            crate::schema_view::SchemaFieldView::new(
                "profile",
                "display_name",
                crate::schema_view::SchemaFieldKind::String,
            )
            .text_predicate_queryable(),
        ],
        [],
    )
}

fn direct_detail() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_detail(
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
    )
    .unwrap()
}

fn direct_collection() -> crate::canonicalization::CanonicalQueryBundle {
    GuidedAuthoringPath::canonicalize_collection(
        crate::authoring::RawAuthoredQuery::collection_builder(RootEntityKey::new("user").unwrap())
            .project(AspectFieldSelector::new("identity", "id").unwrap())
            .project(AspectFieldSelector::new("profile", "display_name").unwrap())
            .order_by(crate::authoring::OrderingSelector::ascending("profile", "display_name").unwrap())
            .build()
            .unwrap(),
        crate::authoring::RawAuthoredResultShape::collection_builder()
            .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap(),
            )
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn basis_intent() -> crate::basis::ExecutionBasisIntent {
    crate::basis::ExecutionBasisIntent::new(
        crate::basis::BasisAuthorityFamily::Runtime,
        crate::basis::SnapshotLineageClass::CurrentHead,
        false,
    )
}

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
    assert_eq!(denial.failure_class(), &SavedQueryFailureClass::IllegalSemanticDrift);
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
    assert_eq!(denial.failure_class(), &SavedQueryFailureClass::IllegalSemanticDrift);
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
    assert_eq!(denial.failure_class(), &SavedQueryFailureClass::IllegalSemanticDrift);
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
            admit_view_shape(&direct, ViewShapeDescriptor::inspector_detail_focused("profile"))
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

#[test]
fn durable_claims_are_explicitly_denied() {
    let direct = direct_collection();
    let view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &direct,
            collection_schema_view(),
            admit_view_shape(&direct, ViewShapeDescriptor::table()).unwrap(),
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

    let durable_reload = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::DurableReload)
        .unwrap_err();
    assert_eq!(durable_reload.failure_class(), &SavedQueryFailureClass::DurableClaimDenied);
    let import_export = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::ImportExport)
        .unwrap_err();
    assert_eq!(import_export.failure_class(), &SavedQueryFailureClass::DurableClaimDenied);
    let restart = saved
        .admit_persistence_claim(SavedQueryPersistenceClaim::RestartStableContinuation)
        .unwrap_err();
    assert_eq!(restart.failure_class(), &SavedQueryFailureClass::DurableClaimDenied);
}

#[test]
fn saved_query_freeze_denies_mismatched_canonical_and_view_plan() {
    let detail = direct_detail();
    let collection = direct_collection();
    let collection_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &collection,
            collection_schema_view(),
            admit_view_shape(&collection, ViewShapeDescriptor::table()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let error = freeze_direct_saved_query(
        &detail,
        &collection_view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap_err();
    assert_eq!(
        error.failure_class(),
        &SavedQueryFailureClass::FreezeInvariantRejected
    );
}
