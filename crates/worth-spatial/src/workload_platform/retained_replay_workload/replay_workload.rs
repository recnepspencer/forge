use super::{
    CapturedRetainedWorkload, ReplayEvidenceSet, ReplayParityReport, ReplayReceiptSet,
    ReplayWorkloadCounters, RetainedArtifactCaptureReceipt, RetainedArtifactSet,
    UnsupportedReplayReasonCode, UnsupportedReplayWorkload,
};
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
use crate::workload_platform::transform_workload::TransformedWorkload;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

pub struct ReplayWorkload {
    transformed_workload: TransformedWorkload,
    declaration: String,
    retained_artifacts: Option<RetainedArtifactSet>,
    retained_capture_receipt: Option<RetainedArtifactCaptureReceipt>,
}

impl ReplayWorkload {
    pub fn for_transformed_workload(transformed_workload: TransformedWorkload) -> Self {
        Self {
            transformed_workload,
            declaration: "retained replay workload".to_string(),
            retained_artifacts: None,
            retained_capture_receipt: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_retained_artifacts(mut self, retained_artifacts: RetainedArtifactSet) -> Self {
        self.retained_capture_receipt = Some(RetainedArtifactCaptureReceipt::from_artifacts(
            "capture retained artifacts supplied directly to replay workload",
            &retained_artifacts,
        ));
        self.retained_artifacts = Some(retained_artifacts);
        self
    }

    pub fn with_captured_retained_workload(
        mut self,
        captured_retained_workload: CapturedRetainedWorkload,
    ) -> Self {
        self.retained_capture_receipt = Some(captured_retained_workload.capture_receipt().clone());
        self.retained_artifacts = Some(captured_retained_workload.into_retained_artifacts());
        self
    }

    pub fn replay(mut self) -> Result<ReplayedWorkload, UnsupportedReplayWorkload> {
        reject_blank_replay_declaration(&self.declaration)?;
        let retained_artifacts = require_retained_replay_artifacts(&mut self.retained_artifacts)?;
        let capture_receipt =
            replay_capture_receipt(&mut self.retained_capture_receipt, &retained_artifacts);
        let projection = retained_artifacts.require_projection_consumed_facts()?;
        let historical = replay_retained_historical_subject(&retained_artifacts)?;
        let stage_receipt = admit_retained_replay_stage_receipt(
            self.transformed_workload.receipts().stage_receipt(),
            self.declaration,
        )?;
        let transformed_identity = transformed_replay_workload_identity(&self.transformed_workload);
        let retained_artifact_identity = retained_artifacts.retained_artifact_identity();
        let evidence = replay_evidence_set(&retained_artifact_identity, &historical, projection);
        let parity_report =
            ReplayParityReport::from_retained_projection_match(&historical, projection);
        let receipts = replay_receipts(
            stage_receipt,
            &transformed_identity,
            &retained_artifact_identity,
            &capture_receipt,
            &historical,
            projection,
            &retained_artifacts,
            &evidence,
        );
        Ok(ReplayedWorkload::new(evidence, parity_report, receipts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayedWorkload {
    evidence: ReplayEvidenceSet,
    parity_report: ReplayParityReport,
    receipts: ReplayReceiptSet,
}

impl ReplayedWorkload {
    pub(crate) fn new(
        evidence: ReplayEvidenceSet,
        parity_report: ReplayParityReport,
        receipts: ReplayReceiptSet,
    ) -> Self {
        Self {
            evidence,
            parity_report,
            receipts,
        }
    }

    pub fn evidence(&self) -> &ReplayEvidenceSet {
        &self.evidence
    }

    pub fn parity_report(&self) -> &ReplayParityReport {
        &self.parity_report
    }

    pub fn receipts(&self) -> &ReplayReceiptSet {
        &self.receipts
    }

    pub fn can_enter_diagnostics_workload(&self) -> bool {
        true
    }
}

fn reject_blank_replay_declaration(declaration: &str) -> Result<(), UnsupportedReplayWorkload> {
    if declaration.trim().is_empty() {
        Err(UnsupportedReplayWorkload::new(
            UnsupportedReplayReasonCode::MissingDeclaration,
            "Retained replay workload requires a human-readable declaration.",
        ))
    } else {
        Ok(())
    }
}

fn require_retained_replay_artifacts(
    retained_artifacts: &mut Option<RetainedArtifactSet>,
) -> Result<RetainedArtifactSet, UnsupportedReplayWorkload> {
    retained_artifacts.take().ok_or_else(|| {
        UnsupportedReplayWorkload::new(
            UnsupportedReplayReasonCode::MissingRetainedArtifacts,
            "Retained replay workload requires retained artifacts captured before replay.",
        )
    })
}

fn replay_capture_receipt(
    retained_capture_receipt: &mut Option<RetainedArtifactCaptureReceipt>,
    retained_artifacts: &RetainedArtifactSet,
) -> RetainedArtifactCaptureReceipt {
    retained_capture_receipt.take().unwrap_or_else(|| {
        RetainedArtifactCaptureReceipt::from_artifacts(
            "capture retained artifacts supplied directly to replay workload",
            retained_artifacts,
        )
    })
}

fn replay_retained_historical_subject(
    retained_artifacts: &RetainedArtifactSet,
) -> Result<RetainedPlanarHistoricalInspection, UnsupportedReplayWorkload> {
    retained_artifacts
        .retained_planar_facts()
        .historical_replay(&retained_artifacts.retained_planar_facts().replay_subject())
        .map_err(|_| {
            UnsupportedReplayWorkload::new(
                UnsupportedReplayReasonCode::RetainedHistoricalReplayDenied,
                "Retained replay workload could not replay the retained planar fact subject.",
            )
        })
}

fn transformed_replay_workload_identity(transformed_workload: &TransformedWorkload) -> String {
    transformed_workload
        .receipts()
        .stage_identity()
        .receipt_identity()
}

fn replay_evidence_set(
    retained_artifact_identity: &str,
    historical: &RetainedPlanarHistoricalInspection,
    projection: &ProjectionConsumedPlanarFactsReceipt,
) -> ReplayEvidenceSet {
    ReplayEvidenceSet::from_retained_replay(retained_artifact_identity, historical, projection)
}

fn replay_receipts(
    stage_receipt: RetainedReplayWorkloadReceipt,
    transformed_identity: &str,
    retained_artifact_identity: &str,
    capture_receipt: &RetainedArtifactCaptureReceipt,
    historical: &RetainedPlanarHistoricalInspection,
    projection: &ProjectionConsumedPlanarFactsReceipt,
    retained_artifacts: &RetainedArtifactSet,
    evidence: &ReplayEvidenceSet,
) -> ReplayReceiptSet {
    ReplayReceiptSet::new(
        stage_receipt,
        transformed_identity,
        retained_artifact_identity,
        capture_receipt.capture_identity(),
        capture_receipt.retained_basis_identity(),
        capture_receipt.replay_checkpoint_identity(),
        replay_evidence_identity(
            transformed_identity,
            retained_artifact_identity,
            historical.historical_digest(),
            projection.projection_consumption_digest(),
        ),
        replay_counters(retained_artifacts, historical, evidence),
    )
}

fn replay_counters(
    retained_artifacts: &RetainedArtifactSet,
    historical: &RetainedPlanarHistoricalInspection,
    evidence: &ReplayEvidenceSet,
) -> ReplayWorkloadCounters {
    ReplayWorkloadCounters::new(
        retained_artifacts.retained_artifact_rows(),
        evidence.row_count(),
        historical.counters().replay_basis_rows_inspected(),
        retained_artifacts.projection_consumed_rows(),
    )
}

fn admit_retained_replay_stage_receipt(
    transform_receipt: &crate::workload_platform::vocabulary::TransformWorkloadReceipt,
    declaration: String,
) -> Result<RetainedReplayWorkloadReceipt, UnsupportedReplayWorkload> {
    crate::workload_platform::vocabulary::RetainedReplayWorkload::for_transform(transform_receipt)
        .declared(declaration)
        .admit()
        .map_err(|_| {
            UnsupportedReplayWorkload::new(
                UnsupportedReplayReasonCode::RetainedReplayStageReceiptDenied,
                "Retained replay workload could not produce a stage receipt from transform evidence.",
            )
        })
}

fn replay_evidence_identity(
    transformed_identity: &str,
    retained_artifact_identity: &str,
    historical_digest: &str,
    projection_consumption_digest: &str,
) -> String {
    format!(
        "replay-evidence:{transformed_identity}:{retained_artifact_identity}:{historical_digest}:{projection_consumption_digest}"
    )
}
