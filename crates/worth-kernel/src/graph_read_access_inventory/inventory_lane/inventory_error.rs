#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessInventoryErrorKind {
    MissingSelectedObligations,
    MissingSelectedRegistrations,
    MissingExecutionRows,
    MissingAuthorityDigest,
    MissingTouchDescriptorDigest,
    MissingSelectedRegistrationDigest,
    MissingResidueManifestDigest,
    MissingExecutionProofDigest,
    MissingAdoptionManifestDigest,
    MissingSelectorPrecisionReportDigest,
    AuthorityDigestCountMismatch,
    TouchDescriptorDigestCountMismatch,
    SelectedRegistrationDigestCountMismatch,
    ResidueManifestDigestCountMismatch,
    ExecutionProofDigestCountMismatch,
    AdoptionManifestDigestCountMismatch,
    SelectorPrecisionReportDigestCountMismatch,
    GraphReadAccessPlanningAlreadyClaimed,
    MissingSourcePath,
    MissingOwner,
    MissingCurrentCaller,
    MissingClassification,
    MissingCostPosture,
    MissingDeletionAction,
    MissingMilestoneSevenDisposition,
    MissingResidueCurrentCount,
    MissingResidueCap,
    MissingResidueBlocker,
    MissingResidueRemovalTrigger,
    ResidueCountExceedsCap,
    CappedResidueMissingResidueRow,
    ResidueRowOnNonResidueClassification,
    ResidueGrowthRequiresCapUpdate,
    DuplicateInventoryRowIdentity,
    FabricatedReceiptProofDenied,
    LocalSupportRowProofDenied,
    MissingOutOfScopeReason,
    OutOfScopeReasonOnGraphReadClassification,
    OutOfScopeCostPostureMismatch,
    DuplicateCoveredSourcePath,
    MissingRequiredCoveredSource,
    MissingCoverageGuardReport,
    GraphReadBypassBoundaryAuditFailed,
    GraphReadBypassResidueManifestFailed,
    GraphReadBypassAdoptionFailed,
    DeletedGraphReadSourceStillExists,
    MissingScopeBinding,
    MissingScopeEvidence,
    ScopeSourcePathMismatch,
    ScopeClassificationMismatch,
    SelectedObligationRelabelledAsReadAccessPlan,
    UnclassifiedGraphReadSurface,
    ProductionShapedTestSupportUnclassified,
    DeclarationCandidateContractMismatch,
    DeletionTargetContractMismatch,
    CappedResidueContractMismatch,
    CertificationOnlyContractMismatch,
    CapabilityGapContractMismatch,
    OutOfScopeContractMismatch,
    EmptyInventoryRows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessInventoryError {
    kind: WorthGraphReadAccessInventoryErrorKind,
    message: Option<String>,
}

impl WorthGraphReadAccessInventoryError {
    pub(super) const fn new(kind: WorthGraphReadAccessInventoryErrorKind) -> Self {
        Self {
            kind,
            message: None,
        }
    }

    pub(super) fn with_message(
        kind: WorthGraphReadAccessInventoryErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: Some(message.into()),
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessInventoryErrorKind {
        self.kind
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
