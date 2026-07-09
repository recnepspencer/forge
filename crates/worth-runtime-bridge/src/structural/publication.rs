use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{ReducedStructuralMatchSet, StructuralMatchOutcomeClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedStructuralRemapArtifact {
    reduced_match_set: ReducedStructuralMatchSet,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PublishedStructuralRemapArtifact {
    pub(crate) fn from_reduced_match_set(
        reduced_match_set: ReducedStructuralMatchSet,
    ) -> Option<Self> {
        match reduced_match_set.outcome_class() {
            StructuralMatchOutcomeClass::ExactAdvisoryMatch
            | StructuralMatchOutcomeClass::AdvisoryReuseCandidate => {
                let canonical_basis = Arc::<str>::from(format!(
                    "published-structural-remap-artifact|reduced={}|outcome:{:?}|candidates={}",
                    reduced_match_set.digest(),
                    reduced_match_set.outcome_class(),
                    reduced_match_set
                        .retained_candidates()
                        .iter()
                        .map(|candidate| candidate.as_ref())
                        .collect::<Vec<_>>()
                        .join(","),
                ));
                let digest = Sha256::digest(canonical_basis.as_bytes());
                Some(Self {
                    reduced_match_set,
                    canonical_basis,
                    digest: Arc::from(format!(
                        "published-structural-remap-artifact:sha256:{digest:x}"
                    )),
                })
            }
            _ => None,
        }
    }

    pub fn reduced_match_set(&self) -> &ReducedStructuralMatchSet {
        &self.reduced_match_set
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedBranchComparisonArtifact {
    reduced_match_set: ReducedStructuralMatchSet,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl PublishedBranchComparisonArtifact {
    pub(crate) fn from_reduced_match_set(
        reduced_match_set: ReducedStructuralMatchSet,
    ) -> Option<Self> {
        match reduced_match_set.outcome_class() {
            StructuralMatchOutcomeClass::BranchComparisonArtifact => {
                let canonical_basis = Arc::<str>::from(format!(
                    "published-branch-comparison-artifact|reduced={}|branch-diff-count={}|candidates={}",
                    reduced_match_set.digest(),
                    reduced_match_set.branch_diff_count(),
                    reduced_match_set
                        .retained_candidates()
                        .iter()
                        .map(|candidate| candidate.as_ref())
                        .collect::<Vec<_>>()
                        .join(","),
                ));
                let digest = Sha256::digest(canonical_basis.as_bytes());
                Some(Self {
                    reduced_match_set,
                    canonical_basis,
                    digest: Arc::from(format!(
                        "published-branch-comparison-artifact:sha256:{digest:x}"
                    )),
                })
            }
            _ => None,
        }
    }

    pub fn reduced_match_set(&self) -> &ReducedStructuralMatchSet {
        &self.reduced_match_set
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{PublishedBranchComparisonArtifact, PublishedStructuralRemapArtifact};

    use crate::snapshot::BridgeTruthViewSelector;
    use crate::structural::{
        AdmittedStructuralRegistry, PlannedStructuralMatchPacketSet, ReducedStructuralMatchSet,
        StructuralCandidateIdentity, StructuralFingerprintEquivalenceContract,
        StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
        StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
        StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity,
        StructuralMatchCandidate, StructuralMatchCandidateKind, StructuralSchemaIdentity,
        StructuralTruthViewBasis, ValidatedStructuralIdentityDeclaration,
    };

    fn reduced_remap() -> ReducedStructuralMatchSet {
        let declaration = StructuralIdentityDeclaration::advisory_remap(
            StructuralIdentityDeclarationIdentity::admit_bridge_owned("structural:remap"),
            StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
            StructuralFingerprintEquivalenceContract::new(
                StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
                StructuralFingerprintFamily::TopologyFingerprint,
                "topology-v1",
                StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
                StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
                StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
            ),
            StructuralTruthViewBasis::explicit_snapshot(
                BridgeTruthViewSelector::committed_snapshot(
                    crate::truth_identity_fixtures::truth_branch_fixture("main"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                ),
            ),
        );
        let registry = AdmittedStructuralRegistry::freeze(vec![declaration]).unwrap();
        let contract = registry.contracts()[0].clone();
        let planned = PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            ValidatedStructuralIdentityDeclaration::from_contract(&contract),
            None,
            None,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::admit_bridge_owned("candidate:a"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            )],
        );
        ReducedStructuralMatchSet::from_planned_packet_set(planned)
    }

    fn reduced_branch_compare() -> ReducedStructuralMatchSet {
        let declaration = StructuralIdentityDeclaration::branch_comparison(
            StructuralIdentityDeclarationIdentity::admit_bridge_owned("structural:compare"),
            StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
            StructuralFingerprintEquivalenceContract::new(
                StructuralSchemaIdentity::admit_bridge_owned("schema:geometry"),
                StructuralFingerprintFamily::BranchComparisonFingerprint,
                "branch-v1",
                StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
                StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
                StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
            ),
            StructuralTruthViewBasis::explicit_branch_pair(
                BridgeTruthViewSelector::branch_snapshot(
                    crate::truth_identity_fixtures::truth_branch_fixture("left"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-left"),
                ),
                BridgeTruthViewSelector::branch_snapshot(
                    crate::truth_identity_fixtures::truth_branch_fixture("right"),
                    crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-right"),
                ),
            ),
        );
        let registry = AdmittedStructuralRegistry::freeze(vec![declaration]).unwrap();
        let contract = registry.contracts()[0].clone();
        let planned = PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            ValidatedStructuralIdentityDeclaration::from_contract(&contract),
            None,
            None,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::admit_bridge_owned("diff:a"),
                StructuralMatchCandidateKind::BranchDiff,
            )],
        );
        ReducedStructuralMatchSet::from_planned_packet_set(planned)
    }

    #[test]
    fn remap_publication_accepts_advisory_reduced_set() {
        let reduced = reduced_remap();
        let artifact = PublishedStructuralRemapArtifact::from_reduced_match_set(reduced.clone())
            .expect("advisory reduced set should publish a remap artifact");
        assert_eq!(artifact.reduced_match_set(), &reduced);
        assert_eq!(
            artifact.reduced_match_set().outcome_class(),
            crate::structural::StructuralMatchOutcomeClass::ExactAdvisoryMatch
        );
    }

    #[test]
    fn branch_comparison_publication_accepts_branch_artifact_outcome() {
        let reduced = reduced_branch_compare();
        let artifact = PublishedBranchComparisonArtifact::from_reduced_match_set(reduced.clone())
            .expect("branch comparison outcome should publish a branch artifact");
        assert_eq!(artifact.reduced_match_set(), &reduced);
        assert_eq!(artifact.reduced_match_set().branch_diff_count(), 1);
    }
}
