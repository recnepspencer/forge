#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedSignedArea2DDenialKind {
    MissingWindingReceipt,
    MissingPrecisionReceipt,
    MissingDegeneracyPolicy,
    WindingPrecisionBasisMismatch,
    MovementRotationMismatch,
    TolerancePolicyMismatch,
    NonFiniteProjectedCoordinate,
    PrecisionBudgetExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertifiedSignedArea2DDenialBasisLocus {
    WindingReceipt,
    PrecisionReceipt,
    DegeneracyPolicy,
    FrameBasis,
    MovementRotationPosture,
    TolerancePolicy,
    ProjectedCoordinate,
    PrecisionBudget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedSignedArea2DDenial {
    kind: CertifiedSignedArea2DDenialKind,
    basis_locus: CertifiedSignedArea2DDenialBasisLocus,
    reason: &'static str,
}

impl CertifiedSignedArea2DDenial {
    pub fn new(kind: CertifiedSignedArea2DDenialKind, reason: &'static str) -> Self {
        Self {
            kind,
            basis_locus: basis_locus_for(kind),
            reason,
        }
    }

    pub fn kind(&self) -> CertifiedSignedArea2DDenialKind {
        self.kind
    }

    pub fn basis_locus(&self) -> CertifiedSignedArea2DDenialBasisLocus {
        self.basis_locus
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

fn basis_locus_for(kind: CertifiedSignedArea2DDenialKind) -> CertifiedSignedArea2DDenialBasisLocus {
    match kind {
        CertifiedSignedArea2DDenialKind::MissingWindingReceipt => {
            CertifiedSignedArea2DDenialBasisLocus::WindingReceipt
        }
        CertifiedSignedArea2DDenialKind::MissingPrecisionReceipt => {
            CertifiedSignedArea2DDenialBasisLocus::PrecisionReceipt
        }
        CertifiedSignedArea2DDenialKind::MissingDegeneracyPolicy => {
            CertifiedSignedArea2DDenialBasisLocus::DegeneracyPolicy
        }
        CertifiedSignedArea2DDenialKind::WindingPrecisionBasisMismatch => {
            CertifiedSignedArea2DDenialBasisLocus::FrameBasis
        }
        CertifiedSignedArea2DDenialKind::MovementRotationMismatch => {
            CertifiedSignedArea2DDenialBasisLocus::MovementRotationPosture
        }
        CertifiedSignedArea2DDenialKind::TolerancePolicyMismatch => {
            CertifiedSignedArea2DDenialBasisLocus::TolerancePolicy
        }
        CertifiedSignedArea2DDenialKind::NonFiniteProjectedCoordinate => {
            CertifiedSignedArea2DDenialBasisLocus::ProjectedCoordinate
        }
        CertifiedSignedArea2DDenialKind::PrecisionBudgetExceeded => {
            CertifiedSignedArea2DDenialBasisLocus::PrecisionBudget
        }
    }
}
