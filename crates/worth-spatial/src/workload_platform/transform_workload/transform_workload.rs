use super::{
    TransformEvidenceSet, TransformParityReport, TransformPostureReceipt, TransformReceiptSet,
    TransformSequence, TransformWorkloadCounters, TransformedEdge, TransformedFace,
    TransformedLoop, UnsupportedTransformReasonCode, UnsupportedTransformWorkload,
};
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;
use crate::workload_platform::vocabulary::{TransformWorkloadReceipt, WorkloadStageIdentity};

pub struct TransformWorkload {
    projected_workload: ProjectedPlanarWorkload,
    declaration: String,
    transform_sequence: Option<TransformSequence>,
}

impl TransformWorkload {
    pub fn for_projected_workload(projected_workload: ProjectedPlanarWorkload) -> Self {
        Self {
            projected_workload,
            declaration: "transform workload".to_string(),
            transform_sequence: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_transform_sequence(mut self, transform_sequence: TransformSequence) -> Self {
        self.transform_sequence = Some(transform_sequence);
        self
    }

    pub fn transform(mut self) -> Result<TransformedWorkload, UnsupportedTransformWorkload> {
        if self.declaration.trim().is_empty() {
            return Err(UnsupportedTransformWorkload::new(
                UnsupportedTransformReasonCode::MissingDeclaration,
                "Transform workload requires a human-readable declaration.",
            ));
        }
        let Some(transform_sequence) = self.transform_sequence.take() else {
            return Err(UnsupportedTransformWorkload::new(
                UnsupportedTransformReasonCode::MissingTransformSequence,
                "Transform workload requires an explicit transform sequence.",
            ));
        };
        reject_label_only_motion(&transform_sequence)?;
        reject_invalid_cancellation_step_count(&transform_sequence)?;

        self.transform_with_sequence(transform_sequence)
    }

    fn transform_with_sequence(
        self,
        transform_sequence: TransformSequence,
    ) -> Result<TransformedWorkload, UnsupportedTransformWorkload> {
        let stage_receipt =
            admit_transform_stage_receipt(&self.projected_workload, self.declaration)?;
        let projected_workload_identity = self
            .projected_workload
            .receipts()
            .stage_identity()
            .receipt_identity();
        let evidence = TransformEvidenceSet::from_sequence(&transform_sequence);
        let transform_evidence_identity =
            transform_evidence_identity(&projected_workload_identity, &transform_sequence);
        let parity_report = TransformParityReport::from_sequence(&transform_sequence);
        let transformed_faces =
            transformed_faces(&self.projected_workload, &transform_evidence_identity);
        let transformed_edges =
            transformed_edges(&self.projected_workload, &transform_evidence_identity);
        let transformed_loops =
            transformed_loops(&self.projected_workload, &transform_evidence_identity);
        let counters = transform_workload_counters(
            &transform_sequence,
            &evidence,
            &parity_report,
            transformed_faces.len() + transformed_edges.len() + transformed_loops.len(),
        );
        let posture_receipt = transform_posture_receipt(
            stage_receipt.identity().clone(),
            &projected_workload_identity,
            &transform_sequence,
        );
        let receipts = TransformReceiptSet::new(
            stage_receipt,
            projected_workload_identity,
            transform_evidence_identity,
            posture_receipt,
            counters,
        );
        Ok(TransformedWorkload::new(
            transformed_faces,
            transformed_edges,
            transformed_loops,
            evidence,
            parity_report,
            receipts,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformedWorkload {
    transformed_faces: Vec<TransformedFace>,
    transformed_edges: Vec<TransformedEdge>,
    transformed_loops: Vec<TransformedLoop>,
    evidence: TransformEvidenceSet,
    parity_report: TransformParityReport,
    receipts: TransformReceiptSet,
}

impl TransformedWorkload {
    pub(crate) fn new(
        transformed_faces: Vec<TransformedFace>,
        transformed_edges: Vec<TransformedEdge>,
        transformed_loops: Vec<TransformedLoop>,
        evidence: TransformEvidenceSet,
        parity_report: TransformParityReport,
        receipts: TransformReceiptSet,
    ) -> Self {
        Self {
            transformed_faces,
            transformed_edges,
            transformed_loops,
            evidence,
            parity_report,
            receipts,
        }
    }

    pub fn transformed_faces(&self) -> &[TransformedFace] {
        &self.transformed_faces
    }

    pub fn transformed_edges(&self) -> &[TransformedEdge] {
        &self.transformed_edges
    }

    pub fn transformed_loops(&self) -> &[TransformedLoop] {
        &self.transformed_loops
    }

    pub fn evidence(&self) -> &TransformEvidenceSet {
        &self.evidence
    }

    pub fn parity_report(&self) -> &TransformParityReport {
        &self.parity_report
    }

    pub fn receipts(&self) -> &TransformReceiptSet {
        &self.receipts
    }

    pub fn can_enter_retained_replay_workload(&self) -> bool {
        true
    }

    pub fn can_enter_operator_execution_without_projection_consumption(&self) -> bool {
        false
    }
}

fn reject_label_only_motion(
    transform_sequence: &TransformSequence,
) -> Result<(), UnsupportedTransformWorkload> {
    if transform_sequence.has_label_only_motion()
        || !transform_sequence.has_real_transform_evidence()
    {
        Err(UnsupportedTransformWorkload::new(
            UnsupportedTransformReasonCode::LabelOnlyMotionEvidence,
            "Transform workload requires coordinate, basis, or posture evidence; label-only motion is not transform evidence.",
        ))
    } else {
        Ok(())
    }
}

fn reject_invalid_cancellation_step_count(
    transform_sequence: &TransformSequence,
) -> Result<(), UnsupportedTransformWorkload> {
    let has_multiple_cancellation_profiles = transform_sequence.cancellation_replay_count() > 1;
    let has_invalid_cancellation_replay = transform_sequence
        .cancellation_replay_counts()
        .any(|steps| steps != 16 && steps != 64);
    if has_multiple_cancellation_profiles || has_invalid_cancellation_replay {
        Err(UnsupportedTransformWorkload::new(
            UnsupportedTransformReasonCode::InvalidCancellationStepCount,
            "Transform workload exact cancellation replay requires one catalog profile: 16 acceptance steps or 64 hostile catalog steps.",
        ))
    } else {
        Ok(())
    }
}

fn admit_transform_stage_receipt(
    projected_workload: &ProjectedPlanarWorkload,
    declaration: String,
) -> Result<TransformWorkloadReceipt, UnsupportedTransformWorkload> {
    crate::workload_platform::vocabulary::TransformWorkload::for_projection(
        projected_workload.receipts().stage_receipt(),
    )
    .declared(declaration)
    .admit()
    .map_err(|_| {
        UnsupportedTransformWorkload::new(
            UnsupportedTransformReasonCode::TransformStageReceiptDenied,
            "Transform workload could not produce a stage receipt from projection evidence.",
        )
    })
}

fn transform_evidence_identity(
    projected_workload_identity: &str,
    transform_sequence: &TransformSequence,
) -> String {
    let step_identities = transform_sequence
        .steps()
        .iter()
        .map(|step| step.identity())
        .collect::<Vec<_>>()
        .join("|");
    format!("transform-evidence:{projected_workload_identity}:{step_identities}")
}

fn transformed_faces(
    projected_workload: &ProjectedPlanarWorkload,
    transform_evidence_identity: &str,
) -> Vec<TransformedFace> {
    projected_workload
        .projected_faces()
        .iter()
        .map(|face| TransformedFace::from_projected_face(face, transform_evidence_identity))
        .collect()
}

fn transformed_edges(
    projected_workload: &ProjectedPlanarWorkload,
    transform_evidence_identity: &str,
) -> Vec<TransformedEdge> {
    projected_workload
        .projected_edges()
        .edges()
        .iter()
        .map(|edge| TransformedEdge::from_projected_edge(edge, transform_evidence_identity))
        .collect()
}

fn transformed_loops(
    projected_workload: &ProjectedPlanarWorkload,
    transform_evidence_identity: &str,
) -> Vec<TransformedLoop> {
    projected_workload
        .projected_loops()
        .iter()
        .map(|loop_entity| {
            TransformedLoop::from_projected_loop(loop_entity, transform_evidence_identity)
        })
        .collect()
}

fn transform_workload_counters(
    transform_sequence: &TransformSequence,
    evidence: &TransformEvidenceSet,
    parity_report: &TransformParityReport,
    transformed_entities: usize,
) -> TransformWorkloadCounters {
    TransformWorkloadCounters::new(
        transform_sequence.step_count(),
        evidence.changed_coordinate_rows(),
        transformed_entities,
        evidence.evidence_rows(),
        parity_report.row_count(),
        transform_sequence.cancellation_steps(),
    )
}

fn transform_posture_receipt(
    transform_stage_identity: WorkloadStageIdentity,
    projected_workload_identity: &str,
    transform_sequence: &TransformSequence,
) -> TransformPostureReceipt {
    TransformPostureReceipt::new(
        transform_stage_identity,
        projected_workload_identity,
        transform_sequence.rotation_posture(),
        transform_sequence.reorientation(),
        transform_sequence.cancellation_policy(),
    )
}
