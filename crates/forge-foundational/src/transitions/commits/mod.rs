mod authority;
mod vocabulary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalCommittedAuthorityAdmitted;
impl forge_proof::ProofMarker for FoundationalCommittedAuthorityAdmitted {}

pub use authority::{
    foundational_committed_authority_admission, FoundationalCommittedAuthorityAdmission,
    FoundationalCommittedAuthorityAdmissionBasis, FoundationalCommittedAuthorityArtifact,
    FoundationalCommittedAuthorityPhase,
};
pub use vocabulary::{
    FoundationalAuthorityTransitionClass, FoundationalAuthorityTransitionDenial,
    FoundationalAuthorityTransitionOutcomeKind, FoundationalCommitDeltaSummary,
    FoundationalCommitParentBasis, FoundationalCommitParentage,
    FoundationalCommittedAuthorityConstructionDenial, FoundationalCommittedAuthorityInput,
    FoundationalCommittedDeltaLocus, FoundationalMergeAncestryBasis, FoundationalNoOpCause,
};
