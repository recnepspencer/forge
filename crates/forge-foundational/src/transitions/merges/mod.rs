mod builder;
mod strategy;
mod verdict;
mod vocabulary;

pub use builder::{foundational_merge, FoundationalMergeBuilder, FoundationalMergeCandidate};
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
