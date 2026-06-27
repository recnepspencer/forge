#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupInventoryErrorKind {
    MissingSourcePath,
    MissingSurfaceName,
    MissingOwner,
    MissingCurrentCaller,
    MissingAuthorityKind,
    MissingDisposition,
    MissingReplacementPhase,
    MissingBlocker,
    MissingRemovalTrigger,
    MissingCertificationPosture,
    MissingCostPosture,
    MissingQuerySurface,
    MissingRowScope,
    QuerySurfaceRequired,
    QuerySurfaceCannotMintLookupAuthority,
    CertificationOnlyRequiresDenialPosture,
    CappedResidueRequiresBlocker,
    DuplicateInventoryRowIdentity,
    ClassifiedRowCountMismatch,
    UnclassifiedEvidenceLookupSurface,
    ProductionShapedTestSupportUnclassified,
    ExpectedCoveredSurfaceMissingLookupShape,
    EmptyInventoryRows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupInventoryError {
    kind: EvidenceLookupInventoryErrorKind,
    message: Option<String>,
}

impl EvidenceLookupInventoryError {
    pub(crate) const fn new(kind: EvidenceLookupInventoryErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }

    pub(crate) fn with_message(
        kind: EvidenceLookupInventoryErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: Some(message.into()),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupInventoryErrorKind {
        self.kind
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
