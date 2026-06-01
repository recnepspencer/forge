use std::collections::BTreeSet;

use forge_relational::facade::history::BranchId;

use crate::data::authority::{RawTopologyIntent, TopologyMutationBatch};
use crate::data::tracing::{
    AuthorityTraceEvidence, BoundaryFailure, DecisionTrace, IntegrityMarkers, PerformanceAccounting,
};

use super::{TopologyAuthorityError, VerifiedTopologyCommit};

pub(super) fn authority_failure_for_intent(
    error: TopologyAuthorityError,
    branch_id: &BranchId,
    intent: &RawTopologyIntent,
) -> BoundaryFailure<TopologyAuthorityError> {
    BoundaryFailure::failure(
        error,
        Vec::new(),
        DecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: None,
            signal_anchor: None,
            authority: None,
            bridge: None,
            derived: None,
            signal: None,
        },
        IntegrityMarkers::new(
            Some(branch_id.clone()),
            BTreeSet::new(),
            Some(intent.mutation_origin),
            None,
            intent.precision_fallbacks.len(),
            intent.precision_budget_fallbacks.len(),
        ),
        PerformanceAccounting::default(),
    )
}

pub(super) fn authority_failure_for_batch(
    error: TopologyAuthorityError,
    branch_id: &BranchId,
    batch: &TopologyMutationBatch,
) -> BoundaryFailure<TopologyAuthorityError> {
    let authority = match &error {
        TopologyAuthorityError::Commit(commit_error) => {
            Some(AuthorityTraceEvidence::from_commit_logs(
                branch_id.clone(),
                vec![commit_error.commit_log().clone()],
            ))
        }
        _ => None,
    };
    let performance_accounting = authority
        .as_ref()
        .map(AuthorityTraceEvidence::performance_accounting)
        .unwrap_or_default();
    BoundaryFailure::failure(
        error,
        Vec::new(),
        DecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: None,
            signal_anchor: None,
            authority,
            bridge: None,
            derived: None,
            signal: None,
        },
        IntegrityMarkers::new(
            Some(branch_id.clone()),
            batch.touched_aspects.clone(),
            Some(batch.mutation_origin),
            None,
            batch.precision_fallbacks.len(),
            batch.precision_budget_fallbacks.len(),
        ),
        performance_accounting,
    )
}

pub(super) fn integrity_markers_for_verified_commit(
    commit: &VerifiedTopologyCommit,
) -> IntegrityMarkers {
    IntegrityMarkers::new(
        Some(commit.branch_id.clone()),
        commit.canonical_batch.batch.touched_aspects.clone(),
        Some(commit.canonical_batch.batch.mutation_origin),
        Some(commit.read_basis.authority.truth_basis_identity.clone()),
        commit.canonical_batch.batch.precision_fallbacks.len(),
        commit
            .canonical_batch
            .batch
            .precision_budget_fallbacks
            .len(),
    )
}
