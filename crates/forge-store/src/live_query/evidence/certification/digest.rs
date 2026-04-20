use crate::{
    authority::AuthoritativeExportBundle,
    backend::records::StoredCommitEnvelope,
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::super::truth::Milestone8TruthSurface;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct Milestone8TruthDigestBasis {
    pub commit_envelopes: Vec<StoredCommitEnvelope>,
    pub truth_surface: Milestone8TruthSurface,
}

pub(super) fn stable_digest<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("milestone 8 evidence should serialize");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}

pub(super) fn projected_commit_envelopes_strict(
    export: &AuthoritativeExportBundle,
    ordered_commit_ids: &[CommitId],
) -> Result<Vec<StoredCommitEnvelope>, StoreError> {
    let canonical = export.clone().into_canonicalized();
    ordered_commit_ids
        .iter()
        .map(|commit_id| {
            canonical
                .commit_envelopes
                .iter()
                .find(|envelope| envelope.envelope.commit.commit_id == *commit_id)
                .cloned()
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CommitNotFound,
                        format!(
                            "milestone 8 certification could not project commit {} from authoritative export",
                            commit_id.0
                        ),
                    )
                })
        })
        .collect()
}

pub(super) fn authoritative_commit_ids_for_truth_surface(
    export: &AuthoritativeExportBundle,
    basis_frontier_commit_id: CommitId,
    final_frontier_commit_id: CommitId,
    branch_id: &BranchId,
) -> Result<Vec<CommitId>, StoreError> {
    let canonical = export.clone().into_canonicalized();
    let basis_sequence = canonical
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == basis_frontier_commit_id)
        .map(|envelope| envelope.commit_sequence)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CommitNotFound,
                format!(
                    "milestone 8 certification could not find basis frontier commit {} in authoritative export",
                    basis_frontier_commit_id.0
                ),
            )
        })?;
    let final_sequence = canonical
        .commit_envelopes
        .iter()
        .find(|envelope| envelope.envelope.commit.commit_id == final_frontier_commit_id)
        .map(|envelope| envelope.commit_sequence)
        .ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CommitNotFound,
                format!(
                    "milestone 8 certification could not find final frontier commit {} in authoritative export",
                    final_frontier_commit_id.0
                ),
            )
        })?;
    Ok(canonical
        .commit_envelopes
        .iter()
        .filter(|envelope| {
            envelope.envelope.branch_context == *branch_id
                && envelope.commit_sequence > basis_sequence
                && envelope.commit_sequence <= final_sequence
        })
        .map(|envelope| envelope.envelope.commit.commit_id)
        .collect())
}
