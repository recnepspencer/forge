use crate::basis::ResolvedBasisProof;
use crate::identity_authority::QueryCanonicalAuthority;

use super::{
    evidence::IdentityEvolutionCertificationResultEvidence, metadata::BranchLocalityClass,
    results::IdentityEvolutionResultBundle, IdentityEvolutionOutcomeFamily,
};

pub use worth_query_declaration::facade::view_declaration::{
    InspectorIdentityClassification, InspectorIdentityDigest,
};

fn inspector_identity_classification_from_outcome_family(
    family: IdentityEvolutionOutcomeFamily,
) -> InspectorIdentityClassification {
    match family {
        IdentityEvolutionOutcomeFamily::SingularIdentityContinuity => {
            InspectorIdentityClassification::AuthoritativeContinuity
        }
        IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet => {
            InspectorIdentityClassification::IdentitySummary
        }
        IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet => {
            InspectorIdentityClassification::AdvisoryCandidates
        }
        IdentityEvolutionOutcomeFamily::Ambiguity => InspectorIdentityClassification::Ambiguity,
        IdentityEvolutionOutcomeFamily::IdentityBreak => {
            InspectorIdentityClassification::IdentityBreak
        }
        IdentityEvolutionOutcomeFamily::GeneratedIdentity => {
            InspectorIdentityClassification::AuthoritativeContinuity
        }
        IdentityEvolutionOutcomeFamily::RetiredIdentity => {
            InspectorIdentityClassification::IdentityBreak
        }
        IdentityEvolutionOutcomeFamily::Denied => InspectorIdentityClassification::Denied,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectorIdentityArtifact {
    query_authority: QueryCanonicalAuthority,
    basis: ResolvedBasisProof,
    digest: InspectorIdentityDigest,
    classification: InspectorIdentityClassification,
    branch_locality_class: BranchLocalityClass,
    authoritative: bool,
    identity_break: bool,
    replay_stable_digest: String,
}

impl InspectorIdentityArtifact {
    pub fn query_authority(&self) -> &QueryCanonicalAuthority {
        &self.query_authority
    }

    pub fn basis_proof(&self) -> &ResolvedBasisProof {
        &self.basis
    }

    pub fn digest(&self) -> &InspectorIdentityDigest {
        &self.digest
    }

    pub fn classification(&self) -> InspectorIdentityClassification {
        self.classification
    }

    pub fn branch_locality_class(&self) -> BranchLocalityClass {
        self.branch_locality_class
    }

    pub fn authoritative(&self) -> bool {
        self.authoritative
    }

    pub fn identity_break(&self) -> bool {
        self.identity_break
    }

    pub fn replay_stable_digest(&self) -> &str {
        &self.replay_stable_digest
    }

    pub(crate) fn from_parts(
        query_authority: QueryCanonicalAuthority,
        basis: ResolvedBasisProof,
        digest: InspectorIdentityDigest,
        classification: InspectorIdentityClassification,
        branch_locality_class: BranchLocalityClass,
        replay_stable_digest: impl Into<String>,
    ) -> Self {
        Self {
            query_authority,
            basis,
            digest,
            classification,
            branch_locality_class,
            authoritative: matches!(
                classification,
                InspectorIdentityClassification::AuthoritativeContinuity
            ),
            identity_break: classification == InspectorIdentityClassification::IdentityBreak,
            replay_stable_digest: replay_stable_digest.into(),
        }
    }

    pub fn from_result_bundle(bundle: &IdentityEvolutionResultBundle) -> Self {
        let metadata = bundle.metadata();
        let classification =
            inspector_identity_classification_from_outcome_family(bundle.outcome_family());
        let digest = InspectorIdentityDigest::from_parts(&[
            format!("metadata_digest:{}", metadata.metadata_digest().as_str()),
            format!("classification:{}", classification.as_str()),
            format!(
                "branch_locality_class:{}",
                metadata.branch_locality_class().as_str()
            ),
        ]);
        let replay_stable_digest = inspector_replay_stable_digest(
            metadata.query_digest().as_str(),
            metadata.basis_digest().as_str(),
            metadata.lineage_digest().as_str(),
            bundle.outcome_family(),
            classification,
            metadata.branch_locality_class(),
        );
        Self::from_parts(
            metadata.query_authority().clone(),
            metadata.basis_proof().clone(),
            digest,
            classification,
            metadata.branch_locality_class(),
            replay_stable_digest,
        )
    }

    pub fn from_result_evidence(evidence: &IdentityEvolutionCertificationResultEvidence) -> Self {
        let classification =
            inspector_identity_classification_from_outcome_family(evidence.outcome_family());
        let digest = InspectorIdentityDigest::from_parts(&[
            format!("query_digest:{}", evidence.query_digest().as_str()),
            format!("result_digest:{}", evidence.result_digest()),
            format!("classification:{}", classification.as_str()),
            format!(
                "branch_locality_class:{}",
                evidence.branch_locality_class().as_str()
            ),
        ]);
        let replay_stable_digest = inspector_replay_stable_digest(
            evidence.query_digest().as_str(),
            evidence.basis_digest().as_str(),
            evidence.lineage_digest().as_str(),
            evidence.outcome_family(),
            classification,
            evidence.branch_locality_class(),
        );
        Self::from_parts(
            evidence.query_authority().clone(),
            evidence.basis_proof().clone(),
            digest,
            classification,
            evidence.branch_locality_class(),
            replay_stable_digest,
        )
    }
}

fn inspector_replay_stable_digest(
    query_digest: &str,
    basis_digest: &str,
    lineage_digest: &str,
    outcome_family: IdentityEvolutionOutcomeFamily,
    classification: InspectorIdentityClassification,
    branch_locality_class: BranchLocalityClass,
) -> String {
    InspectorIdentityDigest::from_parts(&[
        format!("query_digest:{query_digest}"),
        format!("basis_digest:{basis_digest}"),
        format!("lineage_digest:{lineage_digest}"),
        format!("outcome_family:{}", outcome_family.as_str()),
        format!("classification:{}", classification.as_str()),
        format!("branch_locality_class:{}", branch_locality_class.as_str()),
    ])
    .as_str()
    .to_string()
}
