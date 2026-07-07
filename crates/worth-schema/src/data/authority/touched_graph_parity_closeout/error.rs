#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchedGraphParityReadinessErrorKind {
    ClaimKindMustBeReadinessParity,
    MissingSelectedRouteIdentity,
    MissingSelectedFamilyIdentity,
    MissingSelectedProductIdentity,
    MissingTouchedOrOverlapIdentity,
    MissingRepresentativeFamilyCoverage,
    MissingQueryPostureEvidence,
    MissingResidueOrFirewallDigest,
    MissingArchitectureClaimDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchedGraphParityReadinessError {
    kind: TouchedGraphParityReadinessErrorKind,
    detail: String,
}

impl TouchedGraphParityReadinessError {
    #[cfg(any(test, feature = "touched-graph-parity-internal-authority"))]
    pub(crate) fn new(
        kind: TouchedGraphParityReadinessErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> TouchedGraphParityReadinessErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
