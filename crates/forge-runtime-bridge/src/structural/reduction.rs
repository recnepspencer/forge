use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    PlannedStructuralMatchPacketSet, StructuralComparisonMode, StructuralMatchCandidateKind,
    StructuralMatchOutcomeClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReducedStructuralMatchSet {
    planned_packet_set: PlannedStructuralMatchPacketSet,
    outcome_class: StructuralMatchOutcomeClass,
    retained_candidates: Arc<[Arc<str>]>,
    exact_match_count: usize,
    ambiguity_count: usize,
    branch_diff_count: usize,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl ReducedStructuralMatchSet {
    pub(crate) fn from_planned_packet_set(
        planned_packet_set: PlannedStructuralMatchPacketSet,
    ) -> Self {
        let exact_match_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::ExactAdvisoryMatch
                )
            })
            .count();
        let reuse_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::AdvisoryReuseCandidate
                )
            })
            .count();
        let conflict_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::IdentityAuthorityConflict
                )
            })
            .count();
        let lineage_divergence_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::LineageStructuralDivergence
                )
            })
            .count();
        let branch_diff_count = planned_packet_set
            .candidates()
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.candidate_kind(),
                    StructuralMatchCandidateKind::BranchDiff
                )
            })
            .count();
        let ambiguity_count = exact_match_count + reuse_count;

        let outcome_class = match planned_packet_set.comparison_mode() {
            StructuralComparisonMode::AdvisoryRemap => {
                if lineage_divergence_count > 0 {
                    StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence
                } else if conflict_count > 0 {
                    StructuralMatchOutcomeClass::RejectedIdentityAuthorityConflict
                } else if ambiguity_count == 0 {
                    StructuralMatchOutcomeClass::RejectedNoStructuralMatch
                } else if ambiguity_count > 1 {
                    StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch
                } else if exact_match_count == 1 {
                    StructuralMatchOutcomeClass::ExactAdvisoryMatch
                } else {
                    StructuralMatchOutcomeClass::AdvisoryReuseCandidate
                }
            }
            StructuralComparisonMode::BranchComparison => {
                StructuralMatchOutcomeClass::BranchComparisonArtifact
            }
        };

        let retained_candidates: Arc<[Arc<str>]> = Arc::from(
            planned_packet_set
                .candidates()
                .iter()
                .map(|candidate| Arc::<str>::from(candidate.candidate_identity().as_str()))
                .collect::<Vec<_>>(),
        );

        let canonical_basis = Arc::<str>::from(format!(
            "reduced-structural-match-set|planned={}|outcome:{outcome_class:?}|candidates={}",
            planned_packet_set.digest(),
            retained_candidates
                .iter()
                .map(|candidate| candidate.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            planned_packet_set,
            outcome_class,
            retained_candidates,
            exact_match_count,
            ambiguity_count,
            branch_diff_count,
            canonical_basis,
            digest: Arc::from(format!("reduced-structural-match-set:sha256:{digest:x}")),
        }
    }

    pub fn planned_packet_set(&self) -> &PlannedStructuralMatchPacketSet {
        &self.planned_packet_set
    }

    pub fn outcome_class(&self) -> StructuralMatchOutcomeClass {
        self.outcome_class
    }

    pub fn retained_candidates(&self) -> &[Arc<str>] {
        &self.retained_candidates
    }

    pub fn exact_match_count(&self) -> usize {
        self.exact_match_count
    }

    pub fn ambiguity_count(&self) -> usize {
        self.ambiguity_count
    }

    pub fn branch_diff_count(&self) -> usize {
        self.branch_diff_count
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
    use super::ReducedStructuralMatchSet;

    use crate::snapshot::BridgeTruthViewSelector;
    use crate::structural::{
        AdmittedStructuralRegistry, PlannedStructuralMatchPacketSet, StructuralCandidateIdentity,
        StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
        StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
        StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
        StructuralIdentityDeclarationIdentity, StructuralMatchCandidate,
        StructuralMatchCandidateKind, StructuralMatchOutcomeClass, StructuralSchemaIdentity,
        StructuralTruthViewBasis, ValidatedStructuralIdentityDeclaration,
    };

    fn admitted_contract(
        mode_branch_compare: bool,
    ) -> crate::structural::AdmittedStructuralComparisonContract {
        let declaration = if mode_branch_compare {
            StructuralIdentityDeclaration::branch_comparison(
                StructuralIdentityDeclarationIdentity::new("structural:compare"),
                StructuralSchemaIdentity::new("schema:geometry"),
                StructuralFingerprintEquivalenceContract::new(
                    StructuralSchemaIdentity::new("schema:geometry"),
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
            )
        } else {
            StructuralIdentityDeclaration::advisory_remap(
                StructuralIdentityDeclarationIdentity::new("structural:remap"),
                StructuralSchemaIdentity::new("schema:geometry"),
                StructuralFingerprintEquivalenceContract::new(
                    StructuralSchemaIdentity::new("schema:geometry"),
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
            )
        };
        let registry = AdmittedStructuralRegistry::freeze(vec![declaration]).unwrap();
        registry.contracts()[0].clone()
    }

    #[test]
    fn advisory_remap_reduces_single_exact_candidate() {
        let contract = admitted_contract(false);
        let planned = PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            ValidatedStructuralIdentityDeclaration::from_contract(&contract),
            None,
            None,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:a"),
                StructuralMatchCandidateKind::ExactAdvisoryMatch,
            )],
        );
        let reduced = ReducedStructuralMatchSet::from_planned_packet_set(planned);
        assert_eq!(
            reduced.outcome_class(),
            StructuralMatchOutcomeClass::ExactAdvisoryMatch
        );
    }

    #[test]
    fn advisory_remap_reduces_multiple_candidates_to_ambiguity() {
        let contract = admitted_contract(false);
        let planned = PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            ValidatedStructuralIdentityDeclaration::from_contract(&contract),
            None,
            None,
            vec![
                StructuralMatchCandidate::new(
                    StructuralCandidateIdentity::new("candidate:a"),
                    StructuralMatchCandidateKind::ExactAdvisoryMatch,
                ),
                StructuralMatchCandidate::new(
                    StructuralCandidateIdentity::new("candidate:b"),
                    StructuralMatchCandidateKind::AdvisoryReuseCandidate,
                ),
            ],
        );
        let reduced = ReducedStructuralMatchSet::from_planned_packet_set(planned);
        assert_eq!(
            reduced.outcome_class(),
            StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch
        );
    }

    #[test]
    fn advisory_remap_reduces_lineage_divergence_to_typed_rejection() {
        let contract = admitted_contract(false);
        let planned = PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            ValidatedStructuralIdentityDeclaration::from_contract(&contract),
            None,
            None,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("candidate:lineage-divergence"),
                StructuralMatchCandidateKind::LineageStructuralDivergence,
            )],
        );
        let reduced = ReducedStructuralMatchSet::from_planned_packet_set(planned);
        assert_eq!(
            reduced.outcome_class(),
            StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence
        );
    }

    #[test]
    fn branch_comparison_reduces_to_branch_artifact() {
        let contract = admitted_contract(true);
        let planned = PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            ValidatedStructuralIdentityDeclaration::from_contract(&contract),
            None,
            None,
            vec![StructuralMatchCandidate::new(
                StructuralCandidateIdentity::new("diff:a"),
                StructuralMatchCandidateKind::BranchDiff,
            )],
        );
        let reduced = ReducedStructuralMatchSet::from_planned_packet_set(planned);
        assert_eq!(
            reduced.outcome_class(),
            StructuralMatchOutcomeClass::BranchComparisonArtifact
        );
    }
}
