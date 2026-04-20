use crate::failure::StoreError;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{
    core::{AspectLayoutSliceId, StructuralBlockId},
    physical::ChunkModelFrozenPhysicalLayout,
    planning::AdmittedAspectLayoutReadPlan,
    scopes::{AspectLayoutReadRequest, AspectProjectionSet, CanonicalScopeKey},
};
use super::super::constants::{CHUNK_SHAPE_VERSION, EQUIVALENCE_CONTRACT_VERSION};

pub fn stable_layout_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let canonical = serde_json::to_vec(value).expect("serializing deterministic layout digest input should succeed");
    let mut hasher = Sha256::new();
    hasher.update(canonical);
    format!("{:x}", hasher.finalize())
}

pub(crate) fn stable_layout_truth_digest(export: &crate::AuthoritativeExportBundle) -> String {
    #[derive(Serialize)]
    struct LayoutTruthDigestBasis<'a> {
        branch_records: &'a [crate::backend::records::BranchRecord],
        branch_head_records: &'a [crate::backend::records::BranchHeadRecord],
        commit_envelopes: &'a [crate::backend::records::StoredCommitEnvelope],
        commit_parent_records: &'a [crate::backend::records::CommitParentRecord],
    }
    stable_layout_digest(&LayoutTruthDigestBasis {
        branch_records: &export.branch_records,
        branch_head_records: &export.branch_head_records,
        commit_envelopes: &export.commit_envelopes,
        commit_parent_records: &export.commit_parent_records,
    })
}

pub(crate) fn aspect_projection_digest(projection_set: &AspectProjectionSet) -> Result<String, StoreError> {
    Ok(stable_layout_digest(&projection_set.canonical_aspects()?))
}

pub(crate) fn canonical_slice_ids(request: &AspectLayoutReadRequest) -> Result<Vec<AspectLayoutSliceId>, StoreError> {
    let projection_digest = aspect_projection_digest(request.projection_set())?;
    let scope_key: CanonicalScopeKey = request.scope_class().canonical_scope_key()?;
    Ok(scope_key.members.into_iter().map(|member| {
        AspectLayoutSliceId::new(stable_layout_digest(&(
            request.scope_class().label(),
            &projection_digest,
            &member,
            CHUNK_SHAPE_VERSION.value(),
            EQUIVALENCE_CONTRACT_VERSION.value(),
        )))
    }).collect())
}

pub(crate) fn structural_block_id_for_plan(
    request: &AspectLayoutReadRequest,
    slice_ids: &[AspectLayoutSliceId],
) -> Result<StructuralBlockId, StoreError> {
    let projection_digest = aspect_projection_digest(request.projection_set())?;
    let scope_key = request.scope_class().canonical_scope_key()?;
    Ok(StructuralBlockId::new(stable_layout_digest(&(
        request.scope_class().label(),
        &projection_digest,
        CHUNK_SHAPE_VERSION.value(),
        EQUIVALENCE_CONTRACT_VERSION.value(),
        scope_key.members,
        slice_ids.iter().map(AspectLayoutSliceId::as_str).collect::<Vec<_>>(),
    ))))
}

pub(crate) fn layout_materialization_artifact_id(plan: &AdmittedAspectLayoutReadPlan) -> String {
    let basis = (
        plan.request().target().branch_id().clone(),
        plan.request().target().frontier_commit_id(),
        plan.request().scope_class().label(),
        plan.slice_ids().iter().map(AspectLayoutSliceId::as_str).collect::<Vec<_>>(),
        plan.structural_block_id().as_str(),
    );
    format!("layout-materialization:{}", stable_layout_digest(&basis))
}

pub(crate) fn layout_scope_membership_artifact_id(request: &AspectLayoutReadRequest) -> Result<String, StoreError> {
    Ok(format!(
        "layout-scope-membership:{}",
        stable_layout_digest(&(
            request.target().branch_id().clone(),
            request.target().frontier_commit_id(),
            request.scope_class().label(),
            aspect_projection_digest(request.projection_set())?,
        ))
    ))
}

pub(crate) fn chunk_membership_artifact_id(frozen: &ChunkModelFrozenPhysicalLayout) -> String {
    format!("layout-chunk-membership:{}", frozen.witness().physical_chunk_id().as_str())
}

pub(crate) fn structural_block_artifact_id(structural_block_id: &StructuralBlockId) -> String {
    format!("layout-structural-block:{}", structural_block_id.as_str())
}

pub(crate) fn published_layout_request_artifact_id(request: &AspectLayoutReadRequest) -> Result<String, StoreError> {
    Ok(format!(
        "layout-published-request:{}",
        stable_layout_digest(&(
            request.target().branch_id().clone(),
            request.target().frontier_commit_id(),
            request.scope_class().label(),
            aspect_projection_digest(request.projection_set())?,
        ))
    ))
}
