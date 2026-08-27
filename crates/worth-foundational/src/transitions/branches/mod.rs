mod artifacts;
mod builder;
mod candidate;
mod comparison;
mod fork;
mod identity;
mod local_state;
mod reference;

pub use artifacts::{FoundationalBranchCandidateArtifact, FoundationalStagedBranchArtifact};
pub use builder::{
    foundational_branch_candidate, FoundationalBranchCandidateBuilder,
    FoundationalBranchLocalConstructionDenial,
};
pub use candidate::{
    FoundationalBranchCandidateComparisonBasis, FoundationalBranchCandidateForkBasis,
    FoundationalBranchCandidateForkObservationBasis, FoundationalBranchCandidateId,
    FoundationalBranchCandidateObservationBasis,
};
pub use comparison::{
    FoundationalBranchComparisonBasis, FoundationalBranchReferenceMovement,
    FoundationalBranchReferenceMovementKind,
};
pub use fork::FoundationalBranchForkBasis;
pub use identity::{FoundationalBranchId, FoundationalBranchIdConstructionDenial};
pub use local_state::{
    foundational_branch_local_state_definitions, FoundationalBranchLocalStateDefinition,
    FoundationalBranchLocalStateKind,
};
pub use reference::{
    FoundationalBranchReferenceGeneration, FoundationalBranchReferenceGenerationAdvanceDenial,
    FoundationalBranchReferenceMismatch, FoundationalBranchReferenceMismatchAxis,
    FoundationalBranchReferenceObservation, FoundationalBranchTarget,
    FoundationalBranchTargetBasis, FoundationalBranchTargetEncoding,
    FoundationalBranchTargetEncodingConstructionDenial,
};
