use crate::{
    evidence::StoreCounterSnapshot,
    publication::PublicationWriteOutcome,
    DurableMediaReport,
};
use serde::Serialize;

use super::common::{
    MediaBarrierMatrix, ObservedPublicationFailure, TailValidationReport,
    WritePathCertificationSummary, WritePathDigestBasis, stable_digest,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone35CertificationBundle {
    pub artifact_digest: String,
    pub write_path_digest: String,
    pub ack_boundary_report: PublicationWriteOutcome,
    pub certification_summary: WritePathCertificationSummary,
    pub media_barrier_matrix: MediaBarrierMatrix,
    pub tail_validation_report: TailValidationReport,
    pub observed_failures: Vec<ObservedPublicationFailure>,
    pub failure_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
}

impl Milestone35CertificationBundle {
    pub fn new(
        media_report: DurableMediaReport,
        ack_boundary_report: PublicationWriteOutcome,
        counter_snapshot: StoreCounterSnapshot,
        failures: &[ObservedPublicationFailure],
    ) -> Self {
        let media_barrier_matrix = MediaBarrierMatrix {
            backend_family: media_report.backend_family(),
            content_barrier: media_report.content_barrier(),
            metadata_barrier: media_report.metadata_barrier(),
            ack_required_barrier: media_report.ack_required_barrier(),
            family_states: ack_boundary_report.family_states().to_vec(),
        };
        let certification_summary = WritePathCertificationSummary {
            family_count: ack_boundary_report.family_states().len(),
            published_family_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| matches!(state.state(), crate::PublicationState::Published))
                .count(),
            publication_gap_family_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| matches!(state.state(), crate::PublicationState::PublicationGap))
                .count(),
            non_source_admitted_family_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| !state.source_admitted())
                .count(),
            barrier_complete_not_published_count: ack_boundary_report
                .family_states()
                .iter()
                .filter(|state| matches!(state.state(), crate::PublicationState::BarrierCompleteButNotPublished))
                .count(),
            sufficient_for_published_truth: ack_boundary_report.sufficient_for_published_truth(),
            acknowledgment_eligible: ack_boundary_report.acknowledgment_eligible(),
        };
        let tail_validation_report = TailValidationReport {
            durable_frame_scan_count: counter_snapshot.durable_frame_scan_count,
            durable_frame_reject_count: counter_snapshot.durable_frame_reject_count,
            durable_truncated_tail_count: counter_snapshot.durable_truncated_tail_count,
            durable_torn_write_count: counter_snapshot.durable_torn_write_count,
        };
        let artifact_digest = stable_digest(
            &ack_boundary_report
                .family_states()
                .iter()
                .map(|state| (state.family(), state.state(), state.source_admitted()))
                .collect::<Vec<_>>(),
        );
        let write_path_digest = stable_digest(&WritePathDigestBasis {
            media_report,
            ack_boundary_report: &ack_boundary_report,
            media_barrier_matrix: &media_barrier_matrix,
            tail_validation_report: &tail_validation_report,
        });

        Self {
            artifact_digest,
            write_path_digest,
            ack_boundary_report,
            certification_summary,
            media_barrier_matrix,
            tail_validation_report,
            observed_failures: failures.to_vec(),
            failure_digest: stable_digest(failures),
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 3.5 certification serialization")
    }
}
