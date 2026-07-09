use super::{
    basis_intent, collection_schema_view, detail_schema_view, direct_collection, direct_detail,
};
use crate::composition::{GuidedCompositionPath, QueryScopeDescriptor};
use crate::harness::fixtures::execution_preflights;
use crate::policy_basis::{PolicyExecutionModeRequest, SavedQueryPolicyReuseDescriptor};
use crate::query_context::{
    admit_query_basis_context, bind_query_basis_context, QueryBasisContextRequest,
    QueryContextBindingSource, QueryContextFamily,
};
use crate::saved_query::{
    freeze_composed_saved_query, freeze_direct_saved_query, SavedQueryArtifact,
    SavedQueryFreezeContext, SavedQueryReuseDescriptor, SavedQueryTemporalAsyncSurfacePosture,
};
use crate::view_shape::{
    admit_view_shape, plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape,
    ViewShapeDescriptor, ViewShapePlanArtifact,
};
use worth_foundational::facade::AspectKey;

pub(super) fn freeze_ordinary_grouped_saved_query() -> SavedQueryArtifact {
    let canonical = direct_collection();
    let grouped_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            collection_schema_view(),
            admit_view_shape(
                &canonical,
                ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let saved = freeze_direct_saved_query(
        &canonical,
        &grouped_view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();
    assert_eq!(
        saved.metadata().temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly
    );
    saved
}

pub(super) fn freeze_ordinary_detail_saved_query() -> SavedQueryArtifact {
    let canonical = direct_detail();
    let detail_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            &canonical,
            detail_schema_view(),
            admit_view_shape(&canonical, ViewShapeDescriptor::detail()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let saved = freeze_direct_saved_query(
        &canonical,
        &detail_view,
        SavedQueryFreezeContext::new("test-support", "query_direct"),
    )
    .unwrap();
    assert_eq!(
        saved.metadata().temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly
    );
    saved
}

pub(super) fn freeze_future_preserving_grouped_saved_query() -> SavedQueryArtifact {
    let composed = basis_aware_composed_collection();
    let grouped_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            composed.canonical(),
            collection_schema_view(),
            admit_view_shape(
                composed.canonical(),
                ViewShapeDescriptor::kanban_grouped(aspect_key("status")),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let saved = freeze_composed_saved_query(
        &composed,
        &grouped_view,
        SavedQueryFreezeContext::new("test-support", "query_composition"),
    )
    .unwrap();
    assert_eq!(
        saved.metadata().temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
    saved
}

pub(super) fn freeze_future_preserving_detail_saved_query(
    composed: &crate::composition::ComposedCanonicalQueryBundle,
) -> SavedQueryArtifact {
    let detail_view = plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            composed.canonical(),
            detail_schema_view(),
            admit_view_shape(composed.canonical(), ViewShapeDescriptor::detail()).unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap();

    let saved = freeze_composed_saved_query(
        composed,
        &detail_view,
        SavedQueryFreezeContext::new("test-support", "query_composition"),
    )
    .unwrap();
    assert_eq!(
        saved.metadata().temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
    saved
}

pub(super) fn planned_focused_inspector_view(
    composed: &crate::composition::ComposedCanonicalQueryBundle,
) -> ViewShapePlanArtifact {
    plan_admitted_view_shape(
        validate_canonical_bundle_for_admitted_view_shape(
            composed.canonical(),
            detail_schema_view(),
            admit_view_shape(
                composed.canonical(),
                ViewShapeDescriptor::inspector_detail_focused(
                    worth_foundational::facade::AspectKey::new("profile").unwrap(),
                ),
            )
            .unwrap(),
        )
        .unwrap(),
        basis_intent(),
    )
    .unwrap()
}

pub(super) fn basis_aware_composed_collection() -> crate::composition::ComposedCanonicalQueryBundle
{
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let direct = direct_collection();
    let evidence =
        crate::composition::BasisScopeEvidence::from_admitted_context_for_canonical_query(
            &admitted,
            direct.query().digest(),
        );
    let (_artifact, expanded) = GuidedCompositionPath::expand_collection_scopes(
        crate::authoring::RawAuthoredQuery::collection_builder(
            crate::authoring::RootEntityKey::new("user").unwrap(),
        )
        .project(crate::authoring::AspectFieldSelector::new("identity", "id").unwrap())
        .project(crate::authoring::AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(crate::authoring::AspectFieldSelector::new("status", "lane").unwrap())
        .order_by(crate::authoring::OrderingSelector::ascending("profile", "display_name").unwrap())
        .build()
        .unwrap(),
        crate::authoring::RawAuthoredResultShape::collection_builder()
            .field(crate::authoring::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                crate::authoring::AuthoredResultShapeField::new(
                    "profile",
                    "display_name",
                    "display_name",
                )
                .unwrap(),
            )
            .field(
                crate::authoring::AuthoredResultShapeField::new("status", "lane", "lane").unwrap(),
            )
            .build()
            .unwrap(),
        [QueryScopeDescriptor::basis_aware("current_basis", evidence)],
    )
    .unwrap();

    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

pub(super) fn basis_aware_composed_detail() -> crate::composition::ComposedCanonicalQueryBundle {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .unwrap();
    let admitted = admit_query_basis_context(binding).unwrap();
    let direct = direct_detail();
    let evidence =
        crate::composition::BasisScopeEvidence::from_admitted_context_for_canonical_query(
            &admitted,
            direct.query().digest(),
        );
    let (_artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
        crate::authoring::RawAuthoredQuery::detail_builder(
            crate::authoring::RootEntityKey::new("user").unwrap(),
        )
        .project(crate::authoring::AspectFieldSelector::new("identity", "id").unwrap())
        .project(crate::authoring::AspectFieldSelector::new("profile", "display_name").unwrap())
        .build()
        .unwrap(),
        crate::authoring::RawAuthoredResultShape::detail_builder()
            .field(crate::authoring::AuthoredResultShapeField::new("identity", "id", "id").unwrap())
            .field(
                crate::authoring::AuthoredResultShapeField::new(
                    "profile",
                    "display_name",
                    "display_name",
                )
                .unwrap(),
            )
            .build()
            .unwrap(),
        [QueryScopeDescriptor::basis_aware("current_basis", evidence)],
    )
    .unwrap();

    GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
}

pub(super) fn exact_policy_basis_reuse_descriptor(
    saved_query_digest: &str,
    basis_family: &QueryContextFamily,
) -> SavedQueryPolicyReuseDescriptor {
    let execution_mode = policy_mode_for_basis_family(basis_family);
    SavedQueryPolicyReuseDescriptor::new(
        saved_query_digest,
        "policy:a",
        "tenant-truth:a",
        "tenant-schema:a",
        "branch:a",
        execution_mode,
        "policy:a",
        "tenant-truth:a",
        "tenant-schema:a",
        "branch:a",
        execution_mode,
    )
    .with_temporal_async_surface(
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        Some(basis_family.clone()),
        Some(basis_family.clone()),
    )
}

pub(super) fn saved_query_reuse_descriptor_for_saved(
    saved: &SavedQueryArtifact,
) -> SavedQueryReuseDescriptor {
    SavedQueryReuseDescriptor::new(
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
    .with_identity_consumption(saved.metadata().identity_consumption().clone())
}

pub(super) fn saved_query_reuse_descriptor_for_target_view(
    saved: &SavedQueryArtifact,
    target_view: &ViewShapePlanArtifact,
    basis_family: Option<QueryContextFamily>,
) -> SavedQueryReuseDescriptor {
    SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        basis_family,
        saved.metadata().template_binding_digest().cloned(),
        saved.metadata().template_slot_count(),
        target_view.view_shape_digest().clone(),
        target_view.family(),
        saved.metadata().result_shape_family().clone(),
        saved.metadata().composition_digest().clone(),
        saved.metadata().scope_lineage_digest().cloned(),
        saved.metadata().support_profile_digest().to_string(),
        saved.metadata().capability_family_identity().to_string(),
    )
    .with_identity_consumption(
        target_view
            .delivery_metadata()
            .identity_consumption()
            .clone(),
    )
}

pub(super) fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("saved-query temporal-async test aspect should be foundational")
}

fn policy_mode_for_basis_family(family: &QueryContextFamily) -> PolicyExecutionModeRequest {
    match family {
        QueryContextFamily::CurrentBranchHead => PolicyExecutionModeRequest::CurrentRead,
        QueryContextFamily::BranchHead => PolicyExecutionModeRequest::BranchRead,
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            PolicyExecutionModeRequest::HistoricalRead
        }
        QueryContextFamily::PreviewDerivedHistorical | QueryContextFamily::DiffComparison => {
            panic!("temporal-async support should not admit preview or diff policy basis")
        }
    }
}
