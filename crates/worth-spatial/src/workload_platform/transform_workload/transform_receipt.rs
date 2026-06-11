use crate::planar_contracts::motion_posture::{
    PlanarMotionCancellation, PlanarReorientation, PlanarRotationPosture,
};
use crate::workload_platform::vocabulary::{TransformWorkloadReceipt, WorkloadStageIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransformWorkloadCounters {
    transform_steps: usize,
    changed_coordinate_rows: usize,
    transformed_entities: usize,
    evidence_rows: usize,
    parity_rows: usize,
    cancellation_steps: usize,
}

impl TransformWorkloadCounters {
    pub(crate) fn new(
        transform_steps: usize,
        changed_coordinate_rows: usize,
        transformed_entities: usize,
        evidence_rows: usize,
        parity_rows: usize,
        cancellation_steps: usize,
    ) -> Self {
        Self {
            transform_steps,
            changed_coordinate_rows,
            transformed_entities,
            evidence_rows,
            parity_rows,
            cancellation_steps,
        }
    }

    pub fn transform_steps(self) -> usize {
        self.transform_steps
    }

    pub fn changed_coordinate_rows(self) -> usize {
        self.changed_coordinate_rows
    }

    pub fn transformed_entities(self) -> usize {
        self.transformed_entities
    }

    pub fn evidence_rows(self) -> usize {
        self.evidence_rows
    }

    pub fn parity_rows(self) -> usize {
        self.parity_rows
    }

    pub fn cancellation_steps(self) -> usize {
        self.cancellation_steps
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformPostureReceipt {
    transform_stage_identity: WorkloadStageIdentity,
    projected_workload_identity: String,
    rotation_posture: PlanarRotationPosture,
    reorientation: PlanarReorientation,
    cancellation: PlanarMotionCancellation,
    posture_identity: String,
}

impl TransformPostureReceipt {
    pub(crate) fn new(
        transform_stage_identity: WorkloadStageIdentity,
        projected_workload_identity: impl Into<String>,
        rotation_posture: PlanarRotationPosture,
        reorientation: PlanarReorientation,
        cancellation: PlanarMotionCancellation,
    ) -> Self {
        let projected_workload_identity = projected_workload_identity.into();
        let transform_stage_receipt_identity = transform_stage_identity.receipt_identity();
        let posture_identity = format!(
            "transform-posture:{}:{}:{}:{}:{}",
            transform_stage_receipt_identity,
            projected_workload_identity,
            rotation_posture.as_str(),
            reorientation.as_str(),
            cancellation.as_str(),
        );
        Self {
            transform_stage_identity,
            projected_workload_identity,
            rotation_posture,
            reorientation,
            cancellation,
            posture_identity,
        }
    }

    pub fn transform_stage_identity(&self) -> &WorkloadStageIdentity {
        &self.transform_stage_identity
    }

    pub fn projected_workload_identity(&self) -> &str {
        &self.projected_workload_identity
    }

    pub fn rotation_posture(&self) -> PlanarRotationPosture {
        self.rotation_posture
    }

    pub fn reorientation(&self) -> PlanarReorientation {
        self.reorientation
    }

    pub fn cancellation(&self) -> PlanarMotionCancellation {
        self.cancellation
    }

    pub fn posture_identity(&self) -> &str {
        &self.posture_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformReceiptSet {
    stage_receipt: TransformWorkloadReceipt,
    projected_workload_identity: String,
    transform_evidence_identity: String,
    transform_posture_receipt: TransformPostureReceipt,
    counters: TransformWorkloadCounters,
}

impl TransformReceiptSet {
    pub(crate) fn new(
        stage_receipt: TransformWorkloadReceipt,
        projected_workload_identity: impl Into<String>,
        transform_evidence_identity: impl Into<String>,
        transform_posture_receipt: TransformPostureReceipt,
        counters: TransformWorkloadCounters,
    ) -> Self {
        Self {
            stage_receipt,
            projected_workload_identity: projected_workload_identity.into(),
            transform_evidence_identity: transform_evidence_identity.into(),
            transform_posture_receipt,
            counters,
        }
    }

    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.stage_receipt.identity()
    }

    pub fn stage_receipt(&self) -> &TransformWorkloadReceipt {
        &self.stage_receipt
    }

    pub fn projected_workload_identity(&self) -> &str {
        &self.projected_workload_identity
    }

    pub fn transform_evidence_identity(&self) -> &str {
        &self.transform_evidence_identity
    }

    pub fn transform_posture_receipt(&self) -> &TransformPostureReceipt {
        &self.transform_posture_receipt
    }

    pub fn counters(&self) -> TransformWorkloadCounters {
        self.counters
    }
}
