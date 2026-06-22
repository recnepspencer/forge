#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedSegmentSegment2DDenialKind {
    MissingFirstSegmentIdentity,
    MissingSecondSegmentIdentity,
    MissingTopologyBasisIdentity,
    MissingContactPolicyIdentity,
    MissingProjectionReceipt,
    MissingOrientationReceipt,
    DegenerateFirstSegment,
    DegenerateSecondSegment,
    FrameBasisMismatch,
    MovementRotationMismatch,
    TolerancePolicyMismatch,
    PredicateBasisMismatch,
    PredicateKindMismatch,
    UnsupportedCollinearPolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedSegmentSegment2DDenialBasisLocus {
    FirstSegmentIdentity,
    SecondSegmentIdentity,
    TopologyBasisIdentity,
    ContactPolicyIdentity,
    ProjectionReceipt,
    OrientationReceipt,
    SegmentLength,
    FrameBasis,
    MovementRotationPosture,
    TolerancePolicy,
    PredicateBasis,
    PredicateKind,
    CollinearPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedSegmentSegment2DDenial {
    kind: CertifiedSegmentSegment2DDenialKind,
    basis_locus: CertifiedSegmentSegment2DDenialBasisLocus,
    reason: &'static str,
}

impl CertifiedSegmentSegment2DDenial {
    pub(crate) const fn new(
        kind: CertifiedSegmentSegment2DDenialKind,
        reason: &'static str,
    ) -> Self {
        Self {
            kind,
            basis_locus: denial_basis_locus(kind),
            reason,
        }
    }

    pub fn kind(&self) -> CertifiedSegmentSegment2DDenialKind {
        self.kind
    }

    pub fn basis_locus(&self) -> CertifiedSegmentSegment2DDenialBasisLocus {
        self.basis_locus
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

const fn denial_basis_locus(
    kind: CertifiedSegmentSegment2DDenialKind,
) -> CertifiedSegmentSegment2DDenialBasisLocus {
    match kind {
        CertifiedSegmentSegment2DDenialKind::MissingFirstSegmentIdentity => {
            CertifiedSegmentSegment2DDenialBasisLocus::FirstSegmentIdentity
        }
        CertifiedSegmentSegment2DDenialKind::MissingSecondSegmentIdentity => {
            CertifiedSegmentSegment2DDenialBasisLocus::SecondSegmentIdentity
        }
        CertifiedSegmentSegment2DDenialKind::MissingTopologyBasisIdentity => {
            CertifiedSegmentSegment2DDenialBasisLocus::TopologyBasisIdentity
        }
        CertifiedSegmentSegment2DDenialKind::MissingContactPolicyIdentity => {
            CertifiedSegmentSegment2DDenialBasisLocus::ContactPolicyIdentity
        }
        CertifiedSegmentSegment2DDenialKind::MissingProjectionReceipt => {
            CertifiedSegmentSegment2DDenialBasisLocus::ProjectionReceipt
        }
        CertifiedSegmentSegment2DDenialKind::MissingOrientationReceipt => {
            CertifiedSegmentSegment2DDenialBasisLocus::OrientationReceipt
        }
        CertifiedSegmentSegment2DDenialKind::DegenerateFirstSegment
        | CertifiedSegmentSegment2DDenialKind::DegenerateSecondSegment => {
            CertifiedSegmentSegment2DDenialBasisLocus::SegmentLength
        }
        CertifiedSegmentSegment2DDenialKind::FrameBasisMismatch => {
            CertifiedSegmentSegment2DDenialBasisLocus::FrameBasis
        }
        CertifiedSegmentSegment2DDenialKind::MovementRotationMismatch => {
            CertifiedSegmentSegment2DDenialBasisLocus::MovementRotationPosture
        }
        CertifiedSegmentSegment2DDenialKind::TolerancePolicyMismatch => {
            CertifiedSegmentSegment2DDenialBasisLocus::TolerancePolicy
        }
        CertifiedSegmentSegment2DDenialKind::PredicateBasisMismatch => {
            CertifiedSegmentSegment2DDenialBasisLocus::PredicateBasis
        }
        CertifiedSegmentSegment2DDenialKind::PredicateKindMismatch => {
            CertifiedSegmentSegment2DDenialBasisLocus::PredicateKind
        }
        CertifiedSegmentSegment2DDenialKind::UnsupportedCollinearPolicy => {
            CertifiedSegmentSegment2DDenialBasisLocus::CollinearPolicy
        }
    }
}
