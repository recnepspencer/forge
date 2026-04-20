use crate::{
    authority::AuthoritativeExportBundle,
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::super::continuation_session::LiveQueryContinuationSessionEvidence;

pub(super) fn validate_continuation_session_surface(
    lane: &str,
    export: &AuthoritativeExportBundle,
    branch_id: &BranchId,
    basis_frontier_commit_id: CommitId,
    session: &LiveQueryContinuationSessionEvidence,
) -> Result<(), StoreError> {
    if session.covered_commit_count != session.covered_commit_ids.len() as u64 {
        return Err(StoreError::new(
            StoreErrorKind::ContinuationBatchOrderingViolation,
            format!(
                "milestone 8 {lane} continuation evidence claimed covered_commit_count={} but emitted {} commit ids",
                session.covered_commit_count,
                session.covered_commit_ids.len()
            ),
        ));
    }
    if session.covered_commit_ids.is_empty() {
        if session.final_frontier_commit_id != basis_frontier_commit_id {
            return Err(StoreError::new(
                StoreErrorKind::ContinuationBatchOrderingViolation,
                format!(
                    "milestone 8 {lane} continuation evidence emitted no covered commits but advanced frontier from {} to {}",
                    basis_frontier_commit_id.0,
                    session.final_frontier_commit_id.0
                ),
            ));
        }
        return Ok(());
    }

    let last_covered_commit_id = session
        .covered_commit_ids
        .last()
        .copied()
        .expect("non-empty continuation evidence must have a last covered commit");
    if session.final_frontier_commit_id != last_covered_commit_id {
        return Err(StoreError::new(
            StoreErrorKind::ContinuationBatchOrderingViolation,
            format!(
                "milestone 8 {lane} continuation evidence final frontier {} did not match last covered commit {}",
                session.final_frontier_commit_id.0,
                last_covered_commit_id.0
            ),
        ));
    }

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
                    "milestone 8 {lane} continuation evidence could not find basis frontier commit {} in authoritative export",
                    basis_frontier_commit_id.0
                ),
            )
        })?;

    let mut previous_sequence = basis_sequence;
    for commit_id in &session.covered_commit_ids {
        let envelope = canonical
            .commit_envelopes
            .iter()
            .find(|envelope| envelope.envelope.commit.commit_id == *commit_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CommitNotFound,
                    format!(
                        "milestone 8 {lane} continuation evidence referenced missing commit {}",
                        commit_id.0
                    ),
                )
            })?;
        if envelope.envelope.branch_context != *branch_id {
            return Err(StoreError::new(
                StoreErrorKind::ContinuationBranchIncompatibility,
                format!(
                    "milestone 8 {lane} continuation evidence referenced commit {} on branch `{}` instead of `{}`",
                    commit_id.0,
                    envelope.envelope.branch_context.0,
                    branch_id.0
                ),
            ));
        }
        if envelope.commit_sequence <= previous_sequence {
            let kind = if envelope.commit_sequence == previous_sequence {
                StoreErrorKind::ContinuationBatchDuplicate
            } else {
                StoreErrorKind::ContinuationBatchOrderingViolation
            };
            return Err(StoreError::new(
                kind,
                format!(
                    "milestone 8 {lane} continuation evidence emitted non-monotonic commit sequence {} after {}",
                    envelope.commit_sequence,
                    previous_sequence
                ),
            ));
        }
        previous_sequence = envelope.commit_sequence;
    }

    Ok(())
}
