#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalMergeIntent {
    ReconcileIntoTarget,
}
use forge_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalMergeStructuralSummary {
    source_scope_width: u64,
    target_scope_width: u64,
    touched_scope_width: u64,
    conflict_check_width: u64,
}

impl FoundationalMergeStructuralSummary {
    pub const fn new(
        source_scope_width: u64,
        target_scope_width: u64,
        touched_scope_width: u64,
        conflict_check_width: u64,
    ) -> Self {
        Self {
            source_scope_width,
            target_scope_width,
            touched_scope_width,
            conflict_check_width,
        }
    }

    pub const fn source_scope_width(&self) -> u64 {
        self.source_scope_width
    }

    pub const fn target_scope_width(&self) -> u64 {
        self.target_scope_width
    }

    pub const fn touched_scope_width(&self) -> u64 {
        self.touched_scope_width
    }

    pub const fn conflict_check_width(&self) -> u64 {
        self.conflict_check_width
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMergeConflictLocus {
    category: String,
    source_detail: String,
    target_detail: String,
}

impl FoundationalMergeConflictLocus {
    pub fn new(
        category: impl Into<String>,
        source_detail: impl Into<String>,
        target_detail: impl Into<String>,
    ) -> Self {
        Self {
            category: category.into(),
            source_detail: source_detail.into(),
            target_detail: target_detail.into(),
        }
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn source_detail(&self) -> &str {
        &self.source_detail
    }

    pub fn target_detail(&self) -> &str {
        &self.target_detail
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalMergeVerdictKind {
    Accepted,
    Advisory,
    Conflict,
    Denied,
    Superseded,
    StaleBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBranchBasisDriftKind {
    TargetAdvanced,
    SourceAdvanced,
    MergeBasisInvalidated,
    ParentBasisUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBranchBasisDrift {
    kind: FoundationalBranchBasisDriftKind,
    reason: &'static str,
}

impl FoundationalBranchBasisDrift {
    pub const fn new(kind: FoundationalBranchBasisDriftKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub const fn kind(&self) -> FoundationalBranchBasisDriftKind {
        self.kind
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }

    pub const fn verdict_kind(&self) -> FoundationalMergeVerdictKind {
        FoundationalMergeVerdictKind::StaleBasis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalMergeConstructionDenial {
    EmptyStrategyFamily,
    EmptyStrategySemanticName,
    EmptyStrategyVersion,
    EmptyBasisFamily,
    EmptyBasisVersion,
    MissingTargetBranch,
    MissingIntent,
    MissingStructuralSummary,
    MissingMergeBasis,
    MissingMergeBaseSelectionBasis,
    MissingStrategyIdentity,
    MissingStrategyDescriptorDigest,
    MissingStrategyContractBasis,
    MissingStrategyBasis,
    SourceAndTargetBranchMustDiffer,
    MergeBasisSourceBranchMismatch,
    MergeBasisTargetBranchMismatch,
    ComparisonBasisTargetBranchMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalMergeAdmissionDenial {
    EmptyConflictLoci,
    PolicyDenied { reason: &'static str },
}

impl FoundationalMergeAdmissionDenial {
    pub const fn verdict_kind(&self) -> FoundationalMergeVerdictKind {
        FoundationalMergeVerdictKind::Denied
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalMergeAdmissionDeferred {
    reason: &'static str,
}

impl FoundationalMergeAdmissionDeferred {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalMergeAdmissionRebindRequired {
    reason: &'static str,
}

impl FoundationalMergeAdmissionRebindRequired {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalMergeAdmissionFailure {
    reason: &'static str,
}

impl FoundationalMergeAdmissionFailure {
    pub const fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

pub type FoundationalMergeAdmissionOutcome<T> = TransitionOutcome<
    T,
    FoundationalMergeAdmissionDenial,
    FoundationalMergeAdmissionDeferred,
    FoundationalBranchBasisDrift,
    FoundationalMergeAdmissionRebindRequired,
    FoundationalMergeAdmissionFailure,
>;
