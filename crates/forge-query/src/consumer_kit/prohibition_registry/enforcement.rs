#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryProhibitionEnforcementTier {
    SealedByVisibility,
    TypedAdmissionRequired,
    PhaseThreeAuditResidue,
}

impl ForgeQueryProhibitionEnforcementTier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SealedByVisibility => "sealed-by-visibility",
            Self::TypedAdmissionRequired => "typed-admission-required",
            Self::PhaseThreeAuditResidue => "phase-three-audit-residue",
        }
    }
}
