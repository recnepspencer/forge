mod canonical_artifact_impl;
mod checked_declaration_ext;
mod checker_artifacts;
mod colorability_artifacts;
pub(crate) mod core_artifact;
pub(crate) mod digest_basis;
mod graph_artifacts;
mod proof_and_advisory_artifacts;
mod query_references;

pub use checked_declaration_ext::HadwigerDeclaredFamilyCheckedExt;
pub use checker_artifacts::{
    HadwigerCheckerBoundaryKind, HadwigerCheckerCausalEvidence, HadwigerCheckerPosture,
    UnitDistanceVerification,
};
pub(crate) use colorability_artifacts::assignment_from_model;
pub use colorability_artifacts::{
    ColorabilityEncoding, ColorabilityVerification, ColorabilityVerificationPosture,
    ColoringRefutationCertificate, SolverRun, SolverRunPosture, UnsatCoreArtifact,
};
pub use core_artifact::{
    HadwigerArtifactAuthorityOwner, HadwigerArtifactDigest, HadwigerArtifactKind,
    HadwigerArtifactReference, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
    HadwigerCanonicalArtifact,
};
pub use graph_artifacts::{
    EdgeIdentity, EmbeddingCandidate, GadgetContract, GadgetDefinition, GraphComposition,
    GraphIdentity, GraphVersion, GraphVersionBuilder, ReductionTrace, VertexIdentity,
};
pub(crate) use proof_and_advisory_artifacts::{admitted_proof_claim, blocked_proof_claim};
pub use proof_and_advisory_artifacts::{
    AIAdvisoryArtifact, HadwigerProofClaimKind, HadwigerProofClaimPosture,
    LowerBoundWitnessArtifact, ProofClaim, RetainedBackgroundTheorem,
};
pub use query_references::HadwigerQueryDeclarationReference;
