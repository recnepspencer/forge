use crate::construction::certification::motion::witness_report::{
    PrimitiveConstructionMotionWitnessResolutionFailureKind,
    PrimitiveConstructionMotionWitnessResolutionKind,
    PrimitiveConstructionMotionWitnessResolutionReport,
    PrimitiveConstructionMotionWitnessResolutionStatus,
    PrimitiveConstructionRequestedMotionWitness, PrimitiveConstructionResolvedMotionWitness,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::{
    PrimitiveConstructionFamily, PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionMotionReplayParityReport, PrimitiveConstructionMotionRuntimeSurfaceStatus,
    PrimitiveConstructionQueryMotionWitnessParityReport,
};
use worth_spatial::facade::{SpatialAnchorRef, SpatialWitnessResolutionClass};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionCanonicalTruth {
    kind: PrimitiveConstructionMotionWitnessResolutionKind,
    subject_family: PrimitiveConstructionFamily,
    anchor: SpatialAnchorRef,
    requested_witness: PrimitiveConstructionRequestedMotionWitness,
    status: PrimitiveConstructionMotionWitnessResolutionStatus,
    resolved_witness: Option<PrimitiveConstructionResolvedMotionWitness>,
    resolution_class: Option<SpatialWitnessResolutionClass>,
    failure_kind: Option<PrimitiveConstructionMotionWitnessResolutionFailureKind>,
    truth_digest: String,
}

impl PrimitiveConstructionMotionCanonicalTruth {
    pub fn from_witness_report(
        report: &PrimitiveConstructionMotionWitnessResolutionReport,
    ) -> Self {
        let kind = report.kind();
        let subject_family = report.subject_family();
        let anchor = report.anchor().clone();
        let requested_witness = report.requested_witness().clone();
        let status = report.status();
        let resolved_witness = report.resolved_witness();
        let resolution_class = report.resolution_class();
        let failure_kind = report.failure_kind();
        let truth_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                format!("{kind:?}"),
                subject_family.as_str().to_string(),
                format!("{anchor:?}"),
                format!("{requested_witness:?}"),
                format!("{status:?}"),
                format!("{resolved_witness:?}"),
                format!("{resolution_class:?}"),
                format!("{failure_kind:?}"),
            ],
        );
        Self {
            kind,
            subject_family,
            anchor,
            requested_witness,
            status,
            resolved_witness,
            resolution_class,
            failure_kind,
            truth_digest,
        }
    }

    pub fn kind(&self) -> PrimitiveConstructionMotionWitnessResolutionKind {
        self.kind
    }

    pub fn subject_family(&self) -> PrimitiveConstructionFamily {
        self.subject_family
    }

    pub fn anchor(&self) -> &SpatialAnchorRef {
        &self.anchor
    }

    pub fn requested_witness(&self) -> &PrimitiveConstructionRequestedMotionWitness {
        &self.requested_witness
    }

    pub fn status(&self) -> PrimitiveConstructionMotionWitnessResolutionStatus {
        self.status
    }

    pub fn resolved_witness(&self) -> Option<PrimitiveConstructionResolvedMotionWitness> {
        self.resolved_witness
    }

    pub fn resolution_class(&self) -> Option<SpatialWitnessResolutionClass> {
        self.resolution_class
    }

    pub fn failure_kind(&self) -> Option<PrimitiveConstructionMotionWitnessResolutionFailureKind> {
        self.failure_kind
    }

    pub fn truth_digest(&self) -> &str {
        &self.truth_digest
    }

    pub fn matches_witness_report(
        &self,
        report: &PrimitiveConstructionMotionWitnessResolutionReport,
    ) -> bool {
        self.kind == report.kind()
            && self.subject_family == report.subject_family()
            && self.anchor == *report.anchor()
            && self.requested_witness == *report.requested_witness()
            && self.status == report.status()
            && self.resolved_witness == report.resolved_witness()
            && self.resolution_class == report.resolution_class()
            && self.failure_kind == report.failure_kind()
    }

    pub fn replay_matches(&self, report: &PrimitiveConstructionMotionReplayParityReport) -> bool {
        report.parity_verified()
            && self.matches_witness_report(report.direct_report())
            && self.matches_witness_report(report.replay_report())
    }

    pub fn query_matches(
        &self,
        report: &PrimitiveConstructionQueryMotionWitnessParityReport,
    ) -> bool {
        report.parity_verified()
            && self.kind == report.kind()
            && self.subject_family == report.subject_family()
            && self.anchor == *report.anchor()
            && self.requested_witness == *report.requested_witness()
            && self.status == report.status()
            && self.resolved_witness == report.resolved_witness()
            && self.resolution_class == report.resolution_class()
            && self.failure_kind == report.failure_kind()
    }

    pub fn branch_matches(
        &self,
        report: &PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    ) -> bool {
        self.kind == report.kind()
            && self.subject_family == report.family()
            && self.anchor == *report.anchor()
            && self.requested_witness == *report.requested_witness()
            && self.status == report.motion_status()
            && self.resolved_witness == report.resolved_witness()
            && self.resolution_class == report.resolution_class()
            && self.failure_kind == report.failure_kind()
    }

    pub fn runtime_surface_consistent(
        &self,
        report: &PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    ) -> bool {
        match (self.status, report.runtime_surface_status()) {
            (
                PrimitiveConstructionMotionWitnessResolutionStatus::Rejected,
                PrimitiveConstructionMotionRuntimeSurfaceStatus::MotionRejected,
            ) => report.runtime_report().is_none(),
            (
                PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
                PrimitiveConstructionMotionRuntimeSurfaceStatus::Available,
            ) => report.runtime_report().is_some(),
            (
                PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
                PrimitiveConstructionMotionRuntimeSurfaceStatus::PlacementLoweringBlocked(_),
            )
            | (
                PrimitiveConstructionMotionWitnessResolutionStatus::Admitted,
                PrimitiveConstructionMotionRuntimeSurfaceStatus::ConstraintLoweringBlocked(_),
            ) => report.runtime_report().is_none(),
            _ => false,
        }
    }
}
