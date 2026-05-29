use forge_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticOutcomeKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoverySourceFamily {
    Binding,
    Continuation,
    ContributionComposed,
    DeclarationEntry,
    DeclarationReceipt,
    DeclarationRoutePlan,
    GroupedNeighborhood,
    SignalCompatibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryEvidenceStrength {
    OrdinaryProjection,
    CheckedRetained,
    ProofRetained,
    SupportGrade,
    ReceiptBacked,
    ProvenanceBacked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryBasisPosture {
    Unknown,
    CompleteBasis,
    StaleBasis,
    ReducedBasis,
    ReducedAndStaleBasis,
    BasisMismatch,
    CurrentHead,
    HistoricalSnapshot,
    PreviewDerived,
    Mixed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryAspectPosture {
    None,
    RequiredContract,
    RetainedContractAndCoverage,
    AspectSensitiveReadmission,
    CategoryScopedAspectComposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRecoveryConflictPosture {
    None,
    ManualInspectionRequired,
    MixedContributionFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryFoundationalSupportContext {
    truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
}

impl ForgeQueryRecoveryFoundationalSupportContext {
    pub const fn new(
        truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
        recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
    ) -> Self {
        Self {
            truth_kind,
            basis_disclosure,
            recovery_posture,
        }
    }

    pub const fn truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        self.truth_kind
    }

    pub const fn basis_disclosure(&self) -> FoundationalBoundaryEvidenceSupportBasisDisclosure {
        self.basis_disclosure
    }

    pub const fn recovery_posture(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceSupportRecoveryPosture> {
        self.recovery_posture
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryRecoveryFoundationalDiagnosticContext {
    outcome_kind: FoundationalDiagnosticOutcomeKind,
    denial_class: Option<FoundationalDiagnosticDenialClass>,
}

impl ForgeQueryRecoveryFoundationalDiagnosticContext {
    pub const fn new(
        outcome_kind: FoundationalDiagnosticOutcomeKind,
        denial_class: Option<FoundationalDiagnosticDenialClass>,
    ) -> Self {
        Self {
            outcome_kind,
            denial_class,
        }
    }

    pub const fn outcome_kind(&self) -> FoundationalDiagnosticOutcomeKind {
        self.outcome_kind
    }

    pub const fn denial_class(&self) -> Option<FoundationalDiagnosticDenialClass> {
        self.denial_class
    }
}
