use crate::{
    authority::AuthoritativeExportBundle, evidence::StoreCounterSnapshot, failure::StoreError,
    live_query::continuation::ContinuationStrategy,
};
use serde::Serialize;

use super::super::{
    basis::LiveQueryBasisEvidence, continuation_session::LiveQueryContinuationSessionEvidence,
    truth::Milestone8TruthSurface,
};
use super::{
    digest::{
        authoritative_commit_ids_for_truth_surface, projected_commit_envelopes_strict,
        stable_digest, Milestone8TruthDigestBasis,
    },
    model::Milestone8CertificationSummary,
    validation::validate_continuation_session_surface,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone8CertificationBundle {
    pub truth_digest: String,
    pub restore_digest: String,
    pub failure_digest: String,
    pub truth_surface: Milestone8TruthSurface,
    pub basis: LiveQueryBasisEvidence,
    pub continuation: LiveQueryContinuationSessionEvidence,
    pub control_continuation: LiveQueryContinuationSessionEvidence,
    pub certification_summary: Milestone8CertificationSummary,
    pub counter_snapshot: StoreCounterSnapshot,
}

impl Milestone8CertificationBundle {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        primary_export: &AuthoritativeExportBundle,
        control_export: &AuthoritativeExportBundle,
        restored_export: &AuthoritativeExportBundle,
        truth_surface: Milestone8TruthSurface,
        basis: LiveQueryBasisEvidence,
        continuation: LiveQueryContinuationSessionEvidence,
        control_continuation: LiveQueryContinuationSessionEvidence,
        counter_snapshot: StoreCounterSnapshot,
        failure_markers: &[String],
    ) -> Result<Self, StoreError> {
        validate_continuation_session_surface(
            "primary",
            primary_export,
            &truth_surface.branch_id,
            basis.frontier_commit_id,
            &continuation,
        )?;
        validate_continuation_session_surface(
            "control",
            control_export,
            &truth_surface.branch_id,
            basis.frontier_commit_id,
            &control_continuation,
        )?;
        let authoritative_control_commit_ids = authoritative_commit_ids_for_truth_surface(
            control_export,
            basis.frontier_commit_id,
            truth_surface.final_frontier_commit_id,
            &truth_surface.branch_id,
        )?;
        let primary_truth_digest = stable_digest(&Milestone8TruthDigestBasis {
            commit_envelopes: projected_commit_envelopes_strict(
                primary_export,
                &continuation.covered_commit_ids,
            )?,
            truth_surface: truth_surface.clone(),
        });
        let control_truth_digest = stable_digest(&Milestone8TruthDigestBasis {
            commit_envelopes: projected_commit_envelopes_strict(
                control_export,
                &authoritative_control_commit_ids,
            )?,
            truth_surface: truth_surface.clone(),
        });
        let restore_digest = stable_digest(&Milestone8TruthDigestBasis {
            commit_envelopes: projected_commit_envelopes_strict(
                restored_export,
                &continuation.covered_commit_ids,
            )?,
            truth_surface: truth_surface.clone(),
        });
        let failure_digest = stable_digest(&failure_markers);

        let certification_summary = Milestone8CertificationSummary {
            truth_matches_control_lane: primary_truth_digest == control_truth_digest,
            restore_truth_parity: primary_truth_digest == restore_digest,
            control_lane_matches_authoritative_truth: control_continuation.covered_commit_ids
                == authoritative_control_commit_ids,
            admitted_lane_stayed_narrow: continuation.resolved_strategy
                != ContinuationStrategy::AdmittedLayoutNarrow
                || (continuation.broadened_item_count == 0
                    && counter_snapshot.continuation_control_lane_fallback_count == 0),
            no_gap_batches_observed: counter_snapshot.continuation_batch_gap_count == 0,
            no_duplicate_batches_observed: counter_snapshot.continuation_batch_duplicate_count == 0,
            no_illegal_acknowledgments: counter_snapshot.continuation_illegal_acknowledgment_count
                == 0,
            no_hidden_control_lane_fallback: counter_snapshot
                .continuation_control_lane_fallback_count
                == 0,
            no_failure_markers: failure_markers.is_empty(),
        };

        Ok(Self {
            truth_digest: primary_truth_digest,
            restore_digest,
            failure_digest,
            truth_surface,
            basis,
            continuation,
            control_continuation,
            certification_summary,
            counter_snapshot,
        })
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 8 certification serialization")
    }
}
