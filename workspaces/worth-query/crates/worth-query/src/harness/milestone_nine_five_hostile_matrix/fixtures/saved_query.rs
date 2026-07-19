use crate::policy_basis::{PolicyExecutionModeRequest, SavedQueryPolicyReuseDescriptor};
use crate::query_context::QueryContextFamily;
use crate::saved_query::{
    evaluate_saved_query_reuse, freeze_composed_saved_query, freeze_direct_saved_query,
    SavedQueryArtifact, SavedQueryFreezeContext, SavedQueryReuseDescriptor, SavedQueryReuseOutcome,
    SavedQueryTemporalAsyncSurfacePosture,
};

use super::canonical::{
    basis_aware_composed_collection, basis_aware_composed_detail, direct_collection, direct_detail,
};
use super::views::{detail_view, focused_inspector_view, grouped_view};

pub fn freeze_ordinary_detail_saved_query(
    support_profile_digest: &str,
    capability_identity: &str,
) -> SavedQueryArtifact {
    let canonical = direct_detail();
    freeze_direct_saved_query(
        &canonical,
        &detail_view(&canonical),
        SavedQueryFreezeContext::new(support_profile_digest, capability_identity),
    )
    .unwrap()
}

pub fn freeze_ordinary_grouped_saved_query(
    support_profile_digest: &str,
    capability_identity: &str,
) -> SavedQueryArtifact {
    let canonical = direct_collection();
    freeze_direct_saved_query(
        &canonical,
        &grouped_view(&canonical),
        SavedQueryFreezeContext::new(support_profile_digest, capability_identity),
    )
    .unwrap()
}

pub fn freeze_future_preserving_detail_saved_query(
    support_profile_digest: &str,
    capability_identity: &str,
) -> SavedQueryArtifact {
    let composed = basis_aware_composed_detail();
    freeze_composed_saved_query(
        &composed,
        &detail_view(composed.canonical()),
        SavedQueryFreezeContext::new(support_profile_digest, capability_identity),
    )
    .unwrap()
}

pub fn freeze_future_preserving_grouped_saved_query(
    support_profile_digest: &str,
    capability_identity: &str,
) -> SavedQueryArtifact {
    let composed = basis_aware_composed_collection();
    freeze_composed_saved_query(
        &composed,
        &grouped_view(composed.canonical()),
        SavedQueryFreezeContext::new(support_profile_digest, capability_identity),
    )
    .unwrap()
}

pub fn exact_saved_query_reuse(saved: &SavedQueryArtifact) -> SavedQueryReuseOutcome {
    let basis_family = saved.metadata().basis_family().cloned();
    let mut descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        basis_family.clone(),
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
    if saved.metadata().temporal_async_surface_posture()
        == SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
    {
        descriptor = descriptor
            .with_policy_basis_reuse_descriptor(exact_policy_basis_reuse_descriptor(saved));
    }
    evaluate_saved_query_reuse(saved, &descriptor)
}

pub fn erased_basis_family_reuse(saved: &SavedQueryArtifact) -> SavedQueryReuseOutcome {
    let descriptor = SavedQueryReuseDescriptor::new(
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
    evaluate_saved_query_reuse(saved, &descriptor)
}

pub fn focused_inspector_target_reuse(saved: &SavedQueryArtifact) -> SavedQueryReuseOutcome {
    let composed = basis_aware_composed_detail();
    let target_view = focused_inspector_view(composed.canonical());
    let descriptor = SavedQueryReuseDescriptor::new(
        saved.metadata().schema_basis_digest().clone(),
        saved.metadata().basis_family().cloned(),
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
    );
    evaluate_saved_query_reuse(saved, &descriptor)
}

fn exact_policy_basis_reuse_descriptor(
    saved: &SavedQueryArtifact,
) -> SavedQueryPolicyReuseDescriptor {
    let basis_family = saved
        .metadata()
        .basis_family()
        .expect("future-preserving saved query should carry a basis family");
    let execution_mode = match basis_family {
        QueryContextFamily::CurrentBranchHead => PolicyExecutionModeRequest::CurrentRead,
        QueryContextFamily::BranchHead => PolicyExecutionModeRequest::BranchRead,
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            PolicyExecutionModeRequest::HistoricalRead
        }
        QueryContextFamily::PreviewDerivedHistorical | QueryContextFamily::DiffComparison => {
            panic!("future-preserving reuse fixture should not target preview or diff policy")
        }
    };
    SavedQueryPolicyReuseDescriptor::new(
        saved.digest().as_str(),
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
