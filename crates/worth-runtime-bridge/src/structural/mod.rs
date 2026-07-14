mod contracts;
mod declaration;
mod fingerprints;
mod matching;
mod planning;
mod publication;
mod reduction;
mod taxonomy;
mod validation;

pub use contracts::{AdmittedStructuralComparisonContract, AdmittedStructuralRegistry};
pub use declaration::{
    StructuralFingerprintEquivalenceContract, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
    StructuralTruthViewBasisIdentity,
};
pub use fingerprints::{
    StructuralFingerprint, StructuralFingerprintEquivalenceMemberEvidence,
    StructuralFingerprintEquivalenceMemberSet, StructuralFingerprintIdentity,
    StructuralFingerprintRecordValueEvidence, StructuralFingerprintRecordValueEvidenceSet,
};
pub use matching::{classify_advisory_candidates, classify_branch_comparison};
pub use planning::{
    PlannedStructuralMatchPacketSet, StructuralCandidateIdentity, StructuralMatchCandidate,
    StructuralMatchCandidateKind,
};
pub use publication::{PublishedBranchComparisonArtifact, PublishedStructuralRemapArtifact};
pub use reduction::ReducedStructuralMatchSet;
pub use taxonomy::{
    StructuralCandidateSearchScope, StructuralComparisonMode, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralMatchOutcomeClass, StructuralMismatchClass,
    StructuralTruthViewBasisKind,
};
pub use validation::ValidatedStructuralIdentityDeclaration;
