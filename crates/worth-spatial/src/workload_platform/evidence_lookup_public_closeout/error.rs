#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPublicCloseoutErrorKind {
    EmptyFamilyCoverage,
    DuplicateFamilyStageRow,
    MissingPublicCloseoutQueryRow,
    ForbiddenSourceFirewallAuthority,
    MissingFamilyCoverageDisposition,
    MissingSelectedRouteFamilyRow,
    MismatchedFamilyAuthorityChain,
    MismatchedSelectedRouteFamily,
    MismatchedSelectedRouteProduct,
    MismatchedSelectedRouteSupport,
    ImpossibleResidueSuccessMix,
    SourceFirewallDeletionPressureMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutError {
    kind: EvidenceLookupPublicCloseoutErrorKind,
    detail: String,
}

impl EvidenceLookupPublicCloseoutError {
    pub(crate) fn new(
        kind: EvidenceLookupPublicCloseoutErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupPublicCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
