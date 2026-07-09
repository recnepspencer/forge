mod admission;
mod builder;
mod scope_diagnostic_rows;
mod scope_diagnostics;
mod scope_evidence;
mod scope_evidence_validation;
mod scope_non_success;
mod scoped;
mod strategy;
mod verdict;
mod vocabulary;

pub use builder::{foundational_merge, FoundationalMergeBuilder, FoundationalMergeCandidate};
pub use scope_diagnostics::{
    prepare_scoped_merge_diagnostic_explanation, FoundationalScopedMergeDiagnosticInput,
};
pub use scope_evidence::{
    FoundationalAdmittedMergeScopeEvidence, FoundationalScopeAdmissionBasis,
    FoundationalScopeBreadthSummary, FoundationalSelectedScopeLocus,
    FoundationalSelectedScopeNoOpCause, FoundationalSelectedScopeNoOpEvidence,
    FoundationalSkippedOutOfScopeEvidence,
};
pub use scope_non_success::{
    FoundationalDeniedScopeLocus, FoundationalScopedMergeDenialEvidence,
    FoundationalScopedMergeDenialKind, FoundationalScopedMergeUnavailableOutcomeCategory,
    FoundationalScopedMergeUnavailablePosture, FoundationalScopedMergeUnavailableReason,
};
pub use scoped::{
    FoundationalMergeScope, FoundationalMergeScopeFamily, FoundationalSelectedAspectLocus,
    FoundationalSelectedAspectRequestEntry, FoundationalSelectedNodeLocus,
};
pub use strategy::{
    FoundationalMergeBaseSelectionBasis, FoundationalMergeBasis, FoundationalStrategyBasis,
    FoundationalTransitionBasisFamily, FoundationalTransitionBasisIdentity,
    FoundationalTransitionBasisVersion, FoundationalTransitionCorrespondenceBasis,
    FoundationalTransitionRemapBasis, FoundationalTransitionStrategyContractBasis,
    FoundationalTransitionStrategyDescriptorDigest, FoundationalTransitionStrategyFamily,
    FoundationalTransitionStrategyId, FoundationalTransitionStrategyIdentity,
    FoundationalTransitionStrategyOwnershipClass, FoundationalTransitionStrategySemanticName,
    FoundationalTransitionStrategyVersion,
};
pub use verdict::FoundationalMergeVerdict;
pub use vocabulary::{
    FoundationalBranchBasisDrift, FoundationalBranchBasisDriftKind,
    FoundationalMergeAdmissionDeferred, FoundationalMergeAdmissionDenial,
    FoundationalMergeAdmissionFailure, FoundationalMergeAdmissionOutcome,
    FoundationalMergeAdmissionRebindRequired, FoundationalMergeConflictLocus,
    FoundationalMergeConstructionDenial, FoundationalMergeIntent,
    FoundationalMergeStructuralSummary, FoundationalMergeVerdictKind,
};
