use super::{
    ReplayEvidenceSet, ReplayParityReport, ReplayReceiptSet, ReplayWorkloadCounters,
    RetainedArtifactCaptureReceipt, RetainedArtifactSet, UnsupportedReplayReasonCode,
    UnsupportedReplayWorkload,
};
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarHistoricalInspection;
use crate::workload_platform::spatial_compiled_product_consumer_cutover::{
    build_retained_replay_parity_report, require_retained_capture_receipt,
};
use crate::workload_platform::transform_workload::TransformedWorkload;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedRetainedReplayCapture {
    retained_artifacts: RetainedArtifactSet,
    capture_receipt: RetainedArtifactCaptureReceipt,
}

impl AdmittedRetainedReplayCapture {
    pub(crate) fn from_captured_retained_workload(
        captured_retained_workload: super::CapturedRetainedWorkload,
    ) -> Self {
        Self {
            capture_receipt: captured_retained_workload.capture_receipt().clone(),
            retained_artifacts: captured_retained_workload.into_retained_artifacts(),
        }
    }

    fn capture_receipt(&self) -> &RetainedArtifactCaptureReceipt {
        &self.capture_receipt
    }

    fn into_retained_artifacts(self) -> RetainedArtifactSet {
        self.retained_artifacts
    }
}

pub struct ReplayWorkload {
    transformed_workload: TransformedWorkload,
    declaration: String,
    admitted_capture: Option<AdmittedRetainedReplayCapture>,
}

impl ReplayWorkload {
    pub fn for_transformed_workload(transformed_workload: TransformedWorkload) -> Self {
        Self {
            transformed_workload,
            declaration: "retained replay workload".to_string(),
            admitted_capture: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_admitted_retained_replay_capture(
        mut self,
        admitted_capture: AdmittedRetainedReplayCapture,
    ) -> Self {
        self.admitted_capture = Some(admitted_capture);
        self
    }

    pub fn replay(mut self) -> Result<ReplayedWorkload, UnsupportedReplayWorkload> {
        reject_blank_replay_declaration(&self.declaration)?;
        let admitted_capture =
            require_admitted_retained_replay_capture(&mut self.admitted_capture)?;
        let capture_receipt =
            require_retained_capture_receipt(Some(admitted_capture.capture_receipt().clone()))?;
        let retained_artifacts = admitted_capture.into_retained_artifacts();
        let projection = retained_artifacts.require_projection_consumed_facts()?;
        let historical = replay_retained_historical_subject(&retained_artifacts)?;
        let stage_receipt = admit_retained_replay_stage_receipt(
            self.transformed_workload.receipts().stage_receipt(),
            self.declaration,
        )?;
        let transformed_identity = transformed_replay_workload_identity(&self.transformed_workload);
        let retained_artifact_identity = retained_artifacts.retained_artifact_identity();
        let evidence = replay_evidence_set(&retained_artifact_identity, &historical, projection);
        let parity_report = build_retained_replay_parity_report(
            retained_artifacts.retained_planar_facts(),
            &historical,
            projection,
        )
        .map_err(replay_parity_unsupported_workload)?;
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

fn require_admitted_retained_replay_capture(
    admitted_capture: &mut Option<AdmittedRetainedReplayCapture>,
) -> Result<AdmittedRetainedReplayCapture, UnsupportedReplayWorkload> {
    admitted_capture.take().ok_or_else(|| {
        UnsupportedReplayWorkload::new(
            UnsupportedReplayReasonCode::MissingRetainedArtifacts,
            "Retained replay workload requires retained artifacts admitted through the shared cutover lane before replay.",
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

fn replay_parity_unsupported_workload(
    error: crate::workload_platform::retained_replay_workload::ReplayParityError,
) -> UnsupportedReplayWorkload {
    UnsupportedReplayWorkload::new(
        UnsupportedReplayReasonCode::RetainedProjectionDrift,
        format!(
            "Retained replay workload requires parity admitted through the shared consumer cutover lane: {}",
            error.detail()
        ),
    )
}
