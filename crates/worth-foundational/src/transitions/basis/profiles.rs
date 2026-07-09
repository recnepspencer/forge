use worth_proof::AuthorityWitness;

use crate::profiles::{
    attach_boundary_profiled_artifact, attach_proof_bearing_profiled_artifact,
    attach_support_profiled_artifact, AdmittedFoundationalProfileArtifact,
    BoundaryProfiledArtifact, FoundationalProfileAttachmentOutcome,
    FoundationalProfileNarrowingRecord, FoundationalProfileProgressionAuthority,
    FoundationalProfileProgressionOutcome, FoundationalProfileSet, ProofBearingProfiledArtifact,
    SupportProfiledArtifact,
};
use crate::transitions::{
    FoundationalBranchCandidateArtifact, FoundationalCommitReceiptArtifact,
    FoundationalCommittedAuthorityArtifact, FoundationalMergeVerdict,
    FoundationalStagedBranchArtifact,
};

pub fn attach_boundary_profiled_branch_candidate<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    candidate: FoundationalBranchCandidateArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<
    BoundaryProfiledArtifact<FoundationalBranchCandidateArtifact<T>>,
> {
    attach_boundary_profiled_artifact(admitted, materialized, narrowing, candidate, authority)
}

pub fn attach_boundary_profiled_staged_branch<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    staged: FoundationalStagedBranchArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileProgressionOutcome<
    BoundaryProfiledArtifact<FoundationalStagedBranchArtifact<T>>,
> {
    attach_boundary_profiled_artifact(admitted, materialized, narrowing, staged, authority)
}

pub fn attach_support_profiled_merge_verdict<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    verdict: FoundationalMergeVerdict<T>,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileAttachmentOutcome<SupportProfiledArtifact<FoundationalMergeVerdict<T>>> {
    attach_support_profiled_artifact(admitted, materialized, narrowing, verdict, authority)
}

pub fn attach_proof_bearing_profiled_committed_authority<T>(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    committed: FoundationalCommittedAuthorityArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileAttachmentOutcome<
    ProofBearingProfiledArtifact<FoundationalCommittedAuthorityArtifact<T>>,
> {
    attach_proof_bearing_profiled_artifact(admitted, materialized, narrowing, committed, authority)
}

pub fn attach_proof_bearing_profiled_commit_receipt(
    admitted: AdmittedFoundationalProfileArtifact,
    materialized: FoundationalProfileSet,
    narrowing: Option<FoundationalProfileNarrowingRecord>,
    receipt: FoundationalCommitReceiptArtifact,
    authority: AuthorityWitness<FoundationalProfileProgressionAuthority>,
) -> FoundationalProfileAttachmentOutcome<
    ProofBearingProfiledArtifact<FoundationalCommitReceiptArtifact>,
> {
    attach_proof_bearing_profiled_artifact(admitted, materialized, narrowing, receipt, authority)
}
