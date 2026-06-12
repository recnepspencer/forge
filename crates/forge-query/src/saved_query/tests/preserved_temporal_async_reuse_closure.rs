use super::temporal_async_fixtures::{
    basis_aware_composed_detail, exact_policy_basis_reuse_descriptor,
    freeze_future_preserving_detail_saved_query, freeze_future_preserving_grouped_saved_query,
    freeze_ordinary_detail_saved_query, freeze_ordinary_grouped_saved_query,
    planned_focused_inspector_view, saved_query_reuse_descriptor_for_saved,
    saved_query_reuse_descriptor_for_target_view,
};
use crate::policy_basis::SavedQueryPolicyReuseDisposition;
use crate::query_context::QueryContextFamily;
use crate::saved_query::{
    evaluate_saved_query_reuse, SavedQueryFailureClass, SavedQueryRebindingDimension,
    SavedQueryRebindingLegality, SavedQueryReuseDescriptor, SavedQueryReuseOutcome,
    SavedQueryTemporalAsyncSurfacePosture,
};

#[test]
fn grouped_preserved_source_reuse_denies_ordinary_target_with_explicit_temporal_async_drift() {
    let saved = freeze_future_preserving_grouped_saved_query();
    let ordinary_descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        None,
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
    .with_identity_consumption(saved.metadata().identity_consumption().clone());

    assert_temporal_async_downcast_denied(
        evaluate_saved_query_reuse(&saved, &ordinary_descriptor),
        SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly,
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange,
    );
}

#[test]
fn ordinary_grouped_source_reuse_denies_preserved_target_with_explicit_temporal_async_drift() {
    let saved = freeze_ordinary_grouped_saved_query();
    let descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        Some(QueryContextFamily::CurrentBranchHead),
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
    .with_identity_consumption(saved.metadata().identity_consumption().clone());

    assert_temporal_async_downcast_denied(
        evaluate_saved_query_reuse(&saved, &descriptor),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift,
    );
}

#[test]
fn preserved_detail_source_reuse_denies_deferred_inspector_target_without_ordinary_collapse() {
    let composed = basis_aware_composed_detail();
    let saved = freeze_future_preserving_detail_saved_query(&composed);
    let inspector_view = planned_focused_inspector_view(&composed);
    let descriptor = saved_query_reuse_descriptor_for_target_view(
        &saved,
        &inspector_view,
        saved.metadata().basis_family().cloned(),
    );

    assert_temporal_async_downcast_denied(
        evaluate_saved_query_reuse(&saved, &descriptor),
        SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred,
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift,
    );
}

#[test]
fn ordinary_detail_source_reuse_denies_deferred_inspector_target_without_ordinary_collapse() {
    let saved = freeze_ordinary_detail_saved_query();
    let composed = basis_aware_composed_detail();
    let inspector_view = planned_focused_inspector_view(&composed);
    let descriptor = saved_query_reuse_descriptor_for_target_view(
        &saved,
        &inspector_view,
        Some(QueryContextFamily::CurrentBranchHead),
    );

    assert_temporal_async_downcast_denied(
        evaluate_saved_query_reuse(&saved, &descriptor),
        SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred,
        SavedQueryPolicyReuseDisposition::IllegalSemanticDrift,
    );
}

#[test]
fn grouped_preserved_exact_reuse_stays_admitted_without_temporal_async_downcast() {
    let saved = freeze_future_preserving_grouped_saved_query();
    let basis_family = saved
        .metadata()
        .basis_family()
        .cloned()
        .expect("future-preserving grouped saved query should carry a basis family");
    let descriptor =
        saved_query_reuse_descriptor_for_saved(&saved).with_policy_basis_reuse_descriptor(
            exact_policy_basis_reuse_descriptor(saved.digest().as_str(), &basis_family),
        );

    let SavedQueryReuseOutcome::Admitted(decision) =
        evaluate_saved_query_reuse(&saved, &descriptor)
    else {
        panic!("exact grouped preserved reuse should stay admitted");
    };
    assert_eq!(
        decision.overall(),
        SavedQueryRebindingLegality::LegalNoSemanticChange
    );
    assert_eq!(
        decision.temporal_async_surface_posture(),
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    );
    assert_eq!(
        decision.policy_basis_reuse_disposition(),
        SavedQueryPolicyReuseDisposition::LegalNoSemanticChange
    );
}

fn assert_temporal_async_downcast_denied(
    outcome: SavedQueryReuseOutcome,
    expected_target_posture: SavedQueryTemporalAsyncSurfacePosture,
    expected_policy_basis_disposition: SavedQueryPolicyReuseDisposition,
) {
    let SavedQueryReuseOutcome::Denied(denial) = outcome else {
        panic!("temporal/async closure lane must deny");
    };
    assert_eq!(
        denial.overall(),
        SavedQueryRebindingLegality::IllegalSemanticDrift
    );
    assert_eq!(
        denial.failure_class(),
        &SavedQueryFailureClass::IllegalSemanticDrift
    );
    assert_eq!(
        denial.temporal_async_surface_posture(),
        expected_target_posture
    );
    assert_eq!(
        denial.temporal_async_drift_dimension(),
        Some(SavedQueryRebindingDimension::TemporalAsyncSurface)
    );
    assert_eq!(
        denial.policy_basis_reuse_disposition(),
        expected_policy_basis_disposition
    );
}
