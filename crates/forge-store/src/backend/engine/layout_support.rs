use crate::failure::StoreError;
use crate::layout::Milestone6LayoutMaterialization;
use forge_relational::facade::history::CommitId;

use crate::backend::records::{
    Milestone6ChunkMembershipRecord, Milestone6CommitCoupledLayoutSeedRecord,
    Milestone6ScopeSliceMembershipRecord, Milestone6StructuralBlockRecord, StoreState,
    StoredCommitEnvelope,
};

pub(super) fn milestone_6_scope_slice_membership_record(
    materialization: &Milestone6LayoutMaterialization,
) -> Result<Milestone6ScopeSliceMembershipRecord, StoreError> {
    Ok(Milestone6ScopeSliceMembershipRecord {
        artifact_id: crate::layout::layout_scope_membership_artifact_id(
            materialization.admitted_plan().request(),
        )?,
        branch_id: materialization
            .admitted_plan()
            .request()
            .target()
            .branch_id()
            .clone(),
        frontier_commit_id: materialization
            .admitted_plan()
            .request()
            .target()
            .frontier_commit_id(),
        scope_class: materialization
            .admitted_plan()
            .request()
            .scope_class()
            .label()
            .to_string(),
        projection_digest: materialization
            .milestone_7_reference()
            .projection_digest()
            .to_string(),
        slice_ids: materialization.admitted_plan().slice_ids().to_vec(),
        layout_materialization_artifact_id: materialization.artifact_id().to_string(),
    })
}

pub(super) fn milestone_6_commit_coupled_layout_seed_rebuild_records(
    state: &StoreState,
) -> Result<Vec<Milestone6CommitCoupledLayoutSeedRecord>, StoreError> {
    let mut artifact_ids = state
        .commit_support_summaries
        .values()
        .flat_map(|summary| {
            summary
                .milestone_6_published_layout_request_artifact_ids
                .iter()
                .cloned()
        })
        .collect::<Vec<_>>();
    artifact_ids.sort();
    artifact_ids.dedup();
    artifact_ids
        .into_iter()
        .map(|artifact_id| {
            state
                .milestone_6_commit_coupled_layout_seed_records
                .get(&artifact_id)
                .cloned()
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "milestone 6 rebuild seed `{artifact_id}` was listed by commit support publication but missing from commit-coupled layout seed storage"
                    ))
                })
        })
        .collect()
}

pub(super) fn milestone_6_commit_coupled_layout_seed_record(
    materialization: &Milestone6LayoutMaterialization,
    authority_basis_commit: &StoredCommitEnvelope,
) -> Result<Milestone6CommitCoupledLayoutSeedRecord, StoreError> {
    Ok(Milestone6CommitCoupledLayoutSeedRecord {
        artifact_id: crate::layout::published_layout_request_artifact_id(
            materialization.admitted_plan().request(),
        )?,
        request: materialization.admitted_plan().request().clone(),
        layout_materialization_artifact_id: materialization.artifact_id().to_string(),
        authority_basis_commit_id: authority_basis_commit.envelope.commit.commit_id,
        authority_basis_commit_digest: authority_basis_commit.envelope_digest.clone(),
        authority_basis_commit_sequence: authority_basis_commit.commit_sequence,
    })
}

pub(super) fn milestone_6_chunk_membership_record(
    materialization: &Milestone6LayoutMaterialization,
) -> Milestone6ChunkMembershipRecord {
    Milestone6ChunkMembershipRecord {
        artifact_id: crate::layout::chunk_membership_artifact_id(materialization.frozen_layout()),
        physical_chunk_id: materialization
            .frozen_layout()
            .witness()
            .physical_chunk_id()
            .clone(),
        chunk_shape_version: materialization
            .frozen_layout()
            .witness()
            .chunk_shape_version(),
        determinism_digest: materialization
            .frozen_layout()
            .witness()
            .determinism_digest()
            .to_string(),
        slice_ids: materialization
            .frozen_layout()
            .witness()
            .ordered_slice_ids()
            .to_vec(),
        layout_materialization_artifact_id: materialization.artifact_id().to_string(),
    }
}

pub(super) fn attach_milestone_6_commit_coupled_layout_seed_to_commit_support_summary(
    state: &mut StoreState,
    commit_id: CommitId,
    materialization: &Milestone6LayoutMaterialization,
) -> Result<(), StoreError> {
    let artifact_id = crate::layout::published_layout_request_artifact_id(
        materialization.admitted_plan().request(),
    )?;
    let summary_digest = {
        let summary = state
            .commit_support_summaries
            .get_mut(&commit_id.0)
            .ok_or_else(|| {
                StoreError::backend_integrity(format!(
                    "milestone 6 layout materialization `{}` targeted commit `{}` without a commit support summary",
                    materialization.artifact_id(),
                    commit_id.0
                ))
            })?;
        if !summary
            .milestone_6_published_layout_request_artifact_ids
            .contains(&artifact_id)
        {
            summary
                .milestone_6_published_layout_request_artifact_ids
                .push(artifact_id);
            summary
                .milestone_6_published_layout_request_artifact_ids
                .sort();
            summary
                .milestone_6_published_layout_request_artifact_ids
                .dedup();
        }
        super::super::integrity::stable_structural_digest(summary)?
    };
    state.upsert_digest_record(
        crate::backend::records::AuthoritativeArtifactFamily::CommitSupportSummary,
        super::super::integrity::commit_support_summary_artifact_id(commit_id),
        summary_digest,
    );
    let authoritative_summary = state
        .commit_support_summaries
        .get(&commit_id.0)
        .cloned()
        .ok_or_else(|| {
            StoreError::backend_integrity(format!(
                "milestone 6 commit support summary for commit `{}` disappeared during publication",
                commit_id.0
            ))
        })?;
    for layer in state.branch_delta_layer_records.values_mut() {
        let mut updated = false;
        for summary in &mut layer.artifacts.commit_support_summaries {
            if summary.commit_id == commit_id {
                *summary = authoritative_summary.clone();
                updated = true;
            }
        }
        if updated {
            layer.artifacts.canonicalize_order();
        }
    }
    Ok(())
}

pub(super) fn milestone_6_structural_block_record(
    materialization: &Milestone6LayoutMaterialization,
) -> Milestone6StructuralBlockRecord {
    Milestone6StructuralBlockRecord {
        artifact_id: format!(
            "layout-structural-block:{}",
            materialization.block_reuse().structural_block_id().as_str()
        ),
        structural_block_id: materialization.block_reuse().structural_block_id().clone(),
        scope_class: materialization.block_reuse().scope_class().to_string(),
        equivalence_contract_version: materialization.block_reuse().equivalence_contract_version(),
        slice_ids: materialization.block_reuse().slice_ids().to_vec(),
        supporting_layout_materialization_artifact_ids: vec![materialization
            .artifact_id()
            .to_string()],
    }
}

pub(super) fn merge_milestone_6_structural_block_record(
    state: &mut StoreState,
    mut record: Milestone6StructuralBlockRecord,
) {
    if let Some(existing) = state
        .milestone_6_structural_block_records
        .get_mut(&record.artifact_id)
    {
        for artifact_id in record
            .supporting_layout_materialization_artifact_ids
            .drain(..)
        {
            if !existing
                .supporting_layout_materialization_artifact_ids
                .contains(&artifact_id)
            {
                existing
                    .supporting_layout_materialization_artifact_ids
                    .push(artifact_id);
            }
        }
        existing
            .supporting_layout_materialization_artifact_ids
            .sort();
        existing
            .supporting_layout_materialization_artifact_ids
            .dedup();
        return;
    }
    state
        .milestone_6_structural_block_records
        .insert(record.artifact_id.clone(), record);
}
