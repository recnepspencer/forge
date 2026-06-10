use crate::planar_contracts::planar_recovery::{
    PlanarRecoveryBlockerKind, PlanarRecoveryPostureReceipt,
};

use super::{
    PlanarDiagnosticEvidence, PlanarDiagnosticEvidenceKind, PlanarDiagnosticTriggerLocality,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDiagnosticSubjectKind {
    PredicateFailure,
    TopologyFailure,
    BindingFailure,
    PolicyRequired,
    ProjectionFailure,
    TransformFailure,
    MotionFailure,
    UnsupportedPlanarClass,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticSubject {
    kind: PlanarDiagnosticSubjectKind,
    trigger_locality: PlanarDiagnosticTriggerLocality,
    source_digest: String,
    evidence: Vec<PlanarDiagnosticEvidence>,
}

impl PlanarDiagnosticSubject {
    pub fn from_recovery_posture(receipt: PlanarRecoveryPostureReceipt) -> Self {
        let trigger_locality = match receipt.blocker_kind() {
            PlanarRecoveryBlockerKind::ProjectionBasis => {
                PlanarDiagnosticTriggerLocality::ProjectionBasis
            }
            PlanarRecoveryBlockerKind::RetainedOrProjectionBasis => {
                PlanarDiagnosticTriggerLocality::BindingOrRebinding
            }
            PlanarRecoveryBlockerKind::DirtyInput => {
                PlanarDiagnosticTriggerLocality::TopologyContract
            }
            PlanarRecoveryBlockerKind::UnsupportedPlanarClass => {
                PlanarDiagnosticTriggerLocality::UnsupportedPlanarClass
            }
        };
        Self::with_planar_receipt(
            subject_kind_for_locality(trigger_locality),
            trigger_locality,
            receipt.recovery_posture_digest(),
        )
    }

    pub fn predicate_failure(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::PredicateFailure,
            PlanarDiagnosticTriggerLocality::PredicateAuthority,
            source_digest,
        )
    }

    pub fn topology_failure(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::TopologyFailure,
            PlanarDiagnosticTriggerLocality::TopologyContract,
            source_digest,
        )
    }

    pub fn binding_failure(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::BindingFailure,
            PlanarDiagnosticTriggerLocality::BindingOrRebinding,
            source_digest,
        )
    }

    pub fn policy_required(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::PolicyRequired,
            PlanarDiagnosticTriggerLocality::PolicyBoundary,
            source_digest,
        )
    }

    pub fn projection_failure(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::ProjectionFailure,
            PlanarDiagnosticTriggerLocality::ProjectionBasis,
            source_digest,
        )
    }

    pub fn retained_transform_failure(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::TransformFailure,
            PlanarDiagnosticTriggerLocality::RetainedTransformStep,
            source_digest,
        )
    }

    pub fn motion_failure(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::MotionFailure,
            PlanarDiagnosticTriggerLocality::MotionOrRotationPosture,
            source_digest,
        )
    }

    pub fn unsupported_planar_class(source_digest: impl Into<String>) -> Self {
        Self::with_planar_receipt(
            PlanarDiagnosticSubjectKind::UnsupportedPlanarClass,
            PlanarDiagnosticTriggerLocality::UnsupportedPlanarClass,
            source_digest,
        )
    }

    pub(crate) fn push_evidence(&mut self, evidence: PlanarDiagnosticEvidence) {
        self.evidence.push(evidence);
    }

    pub fn kind(&self) -> PlanarDiagnosticSubjectKind {
        self.kind
    }

    pub fn trigger_locality(&self) -> PlanarDiagnosticTriggerLocality {
        self.trigger_locality
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    pub fn evidence(&self) -> &[PlanarDiagnosticEvidence] {
        &self.evidence
    }

    fn with_planar_receipt(
        kind: PlanarDiagnosticSubjectKind,
        trigger_locality: PlanarDiagnosticTriggerLocality,
        source_digest: impl Into<String>,
    ) -> Self {
        let source_digest = source_digest.into();
        Self {
            kind,
            trigger_locality,
            source_digest: source_digest.clone(),
            evidence: vec![PlanarDiagnosticEvidence::new(
                PlanarDiagnosticEvidenceKind::PlanarReceipt,
                source_digest,
            )],
        }
    }
}

fn subject_kind_for_locality(
    locality: PlanarDiagnosticTriggerLocality,
) -> PlanarDiagnosticSubjectKind {
    match locality {
        PlanarDiagnosticTriggerLocality::PredicateAuthority => {
            PlanarDiagnosticSubjectKind::PredicateFailure
        }
        PlanarDiagnosticTriggerLocality::TopologyContract => {
            PlanarDiagnosticSubjectKind::TopologyFailure
        }
        PlanarDiagnosticTriggerLocality::BindingOrRebinding => {
            PlanarDiagnosticSubjectKind::BindingFailure
        }
        PlanarDiagnosticTriggerLocality::PolicyBoundary => {
            PlanarDiagnosticSubjectKind::PolicyRequired
        }
        PlanarDiagnosticTriggerLocality::ProjectionBasis => {
            PlanarDiagnosticSubjectKind::ProjectionFailure
        }
        PlanarDiagnosticTriggerLocality::RetainedTransformStep => {
            PlanarDiagnosticSubjectKind::TransformFailure
        }
        PlanarDiagnosticTriggerLocality::MotionOrRotationPosture => {
            PlanarDiagnosticSubjectKind::MotionFailure
        }
        PlanarDiagnosticTriggerLocality::UnsupportedPlanarClass => {
            PlanarDiagnosticSubjectKind::UnsupportedPlanarClass
        }
    }
}
