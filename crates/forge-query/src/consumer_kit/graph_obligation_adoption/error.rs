#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphObligationConsumerKitErrorKind {
    BlankConsumerName,
    MissingRegistrationDeclaration,
    MissingSelectorCoverage,
    SelectorCoverageMismatch,
    MissingSupportPins,
    MissingLocalCeremonyAudit,
    UnevaluatedLocalCeremonyAudit,
    MissingInMemoryProof,
    EmptyInMemoryProof,
    InMemoryProofRegistrationMismatch,
    DuplicateResidueClass,
    ResidueCapExceeded,
    ResidueGrowthAfterIntroduction,
    ResidueContractDrift,
    IncompleteResidueRow,
    SupportPinDrift,
    LocalCeremonyDetected,
    EmptyRegistrationDeclaration,
    InMemoryWorkspaceBuildFailed,
}

impl ForgeQueryGraphObligationConsumerKitErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlankConsumerName => "blank-consumer-name",
            Self::MissingRegistrationDeclaration => "missing-registration-declaration",
            Self::MissingSelectorCoverage => "missing-selector-coverage",
            Self::SelectorCoverageMismatch => "selector-coverage-mismatch",
            Self::MissingSupportPins => "missing-support-pins",
            Self::MissingLocalCeremonyAudit => "missing-local-ceremony-audit",
            Self::UnevaluatedLocalCeremonyAudit => "unevaluated-local-ceremony-audit",
            Self::MissingInMemoryProof => "missing-in-memory-proof",
            Self::EmptyInMemoryProof => "empty-in-memory-proof",
            Self::InMemoryProofRegistrationMismatch => "in-memory-proof-registration-mismatch",
            Self::DuplicateResidueClass => "duplicate-residue-class",
            Self::ResidueCapExceeded => "residue-cap-exceeded",
            Self::ResidueGrowthAfterIntroduction => "residue-growth-after-introduction",
            Self::ResidueContractDrift => "residue-contract-drift",
            Self::IncompleteResidueRow => "incomplete-residue-row",
            Self::SupportPinDrift => "support-pin-drift",
            Self::LocalCeremonyDetected => "local-ceremony-detected",
            Self::EmptyRegistrationDeclaration => "empty-registration-declaration",
            Self::InMemoryWorkspaceBuildFailed => "in-memory-workspace-build-failed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationConsumerKitError {
    kind: ForgeQueryGraphObligationConsumerKitErrorKind,
    message: String,
}

impl ForgeQueryGraphObligationConsumerKitError {
    pub(crate) fn new(
        kind: ForgeQueryGraphObligationConsumerKitErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryGraphObligationConsumerKitErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ForgeQueryGraphObligationConsumerKitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}

impl std::error::Error for ForgeQueryGraphObligationConsumerKitError {}
