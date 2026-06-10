use super::PlanarStructuralIdentityCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarStructuralIdentityDenialKind {
    MissingBooleanReadinessReceipt,
    MissingCanonicalTransformBasis,
    MissingContrastIdentity,
    CoordinateOnlyIdentityBasis,
    IdentityAuthoritySubstitution,
    BundleTransformMismatch,
}

impl PlanarStructuralIdentityDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingBooleanReadinessReceipt => "missing-boolean-readiness-receipt",
            Self::MissingCanonicalTransformBasis => "missing-canonical-transform-basis",
            Self::MissingContrastIdentity => "missing-contrast-identity",
            Self::CoordinateOnlyIdentityBasis => "coordinate-only-identity-basis",
            Self::IdentityAuthoritySubstitution => "identity-authority-substitution",
            Self::BundleTransformMismatch => "bundle-transform-mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarStructuralIdentityDenial {
    kind: PlanarStructuralIdentityDenialKind,
    reason: String,
    counters: PlanarStructuralIdentityCounters,
}

impl PlanarStructuralIdentityDenial {
    pub(crate) fn new(kind: PlanarStructuralIdentityDenialKind, reason: impl Into<String>) -> Self {
        let counters = match kind {
            PlanarStructuralIdentityDenialKind::CoordinateOnlyIdentityBasis => {
                PlanarStructuralIdentityCounters::rejected_coordinate_only()
            }
            PlanarStructuralIdentityDenialKind::IdentityAuthoritySubstitution => {
                PlanarStructuralIdentityCounters::rejected_identity_substitution()
            }
            _ => PlanarStructuralIdentityCounters::certified(0, 0, 0),
        };
        Self {
            kind,
            reason: reason.into(),
            counters,
        }
    }

    pub fn kind(&self) -> PlanarStructuralIdentityDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> PlanarStructuralIdentityCounters {
        self.counters
    }
}
