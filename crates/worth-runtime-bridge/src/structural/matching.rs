use super::{StructuralFingerprint, StructuralMatchCandidate, StructuralMatchCandidateKind};

fn stable_candidate_kind_label(kind: StructuralMatchCandidateKind) -> &'static str {
    match kind {
        StructuralMatchCandidateKind::ExactAdvisoryMatch => "exact-advisory-match",
        StructuralMatchCandidateKind::AdvisoryReuseCandidate => "advisory-reuse-candidate",
        StructuralMatchCandidateKind::IdentityAuthorityConflict => "identity-authority-conflict",
        StructuralMatchCandidateKind::LineageStructuralDivergence => {
            "lineage-structural-divergence"
        }
        StructuralMatchCandidateKind::BranchDiff => "branch-diff",
    }
}

pub fn classify_advisory_candidates(
    target: &StructuralFingerprint,
    candidates: Vec<StructuralFingerprint>,
) -> Vec<StructuralMatchCandidate> {
    candidates
        .into_iter()
        .filter_map(|fingerprint| {
            let kind = if fingerprint.equivalence_digest() == target.equivalence_digest() {
                if fingerprint.authority_digest() == target.authority_digest() {
                    StructuralMatchCandidateKind::ExactAdvisoryMatch
                } else if fingerprint.snapshot_identity() == target.snapshot_identity() {
                    StructuralMatchCandidateKind::IdentityAuthorityConflict
                } else {
                    StructuralMatchCandidateKind::AdvisoryReuseCandidate
                }
            } else {
                return None;
            };

            Some(StructuralMatchCandidate::with_fingerprint(
                super::StructuralCandidateIdentity::admit_bridge_owned(format!(
                    "derived:{}:{}",
                    fingerprint.fingerprint_identity().as_str(),
                    stable_candidate_kind_label(kind)
                )),
                kind,
                Some(fingerprint),
            ))
        })
        .collect()
}

pub fn classify_branch_comparison(
    left: &StructuralFingerprint,
    right: &StructuralFingerprint,
) -> Vec<StructuralMatchCandidate> {
    if left.equivalence_digest() == right.equivalence_digest() {
        return Vec::new();
    }

    vec![StructuralMatchCandidate::with_fingerprint(
        super::StructuralCandidateIdentity::admit_bridge_owned(format!(
            "branch-diff:{}:{}",
            left.fingerprint_identity().as_str(),
            right.fingerprint_identity().as_str()
        )),
        StructuralMatchCandidateKind::BranchDiff,
        Some(right.clone()),
    )]
}

#[cfg(test)]
mod tests {

    use crate::snapshot::BridgeTruthViewSelector;
    use crate::structural::{
        AdmittedStructuralRegistry, StructuralFingerprint,
        StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
        StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
        StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
        StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
    };

    use super::{classify_advisory_candidates, classify_branch_comparison};

    fn contract() -> crate::structural::AdmittedStructuralComparisonContract {
        let declaration = StructuralIdentityDeclaration::advisory_remap(
            StructuralIdentityDeclarationIdentity::admit_bridge_owned("structural:geometry"),
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
        AdmittedStructuralRegistry::freeze(vec![declaration])
            .unwrap()
            .contracts()[0]
            .clone()
    }

    #[test]
    fn advisory_classification_prefers_exact_for_equal_snapshot() {
        let contract = contract();
        let packet = crate::snapshot::SnapshotReadPacket::new(vec![]);
        let target = StructuralFingerprint::from_snapshot_read_packet(
            &contract,
            &packet,
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        );
        let candidates = classify_advisory_candidates(
            &target,
            vec![StructuralFingerprint::from_snapshot_read_packet(
                &contract,
                &packet,
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        );
        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].candidate_kind(),
            crate::structural::StructuralMatchCandidateKind::ExactAdvisoryMatch
        );
    }

    #[test]
    fn branch_comparison_emits_diff_when_fingerprints_diverge() {
        let contract = contract();
        let left = StructuralFingerprint::from_snapshot_read_packet(
            &contract,
            &crate::snapshot::SnapshotReadPacket::new(vec![]),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        );
        let right = StructuralFingerprint::from_snapshot_read_packet(
            &contract,
            &crate::snapshot::SnapshotReadPacket::new(vec![
                crate::snapshot::SnapshotReadRequest::for_coarse(
                    "entity-1",
                    crate::snapshot::SnapshotReadContract::scalar(
                        worth_foundational::facade::AspectKey::new("profile")
                            .expect("valid snapshot aspect key"),
                        worth_foundational::facade::ScalarAspectType::String,
                    ),
                ),
            ]),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        );
        assert_eq!(classify_branch_comparison(&left, &right).len(), 1);
    }

    #[test]
    fn advisory_candidate_identity_uses_stable_semantic_kind_name() {
        let contract = contract();
        let packet = crate::snapshot::SnapshotReadPacket::new(vec![]);
        let target = StructuralFingerprint::from_snapshot_read_packet(
            &contract,
            &packet,
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        );
        let candidates = classify_advisory_candidates(
            &target,
            vec![StructuralFingerprint::from_snapshot_read_packet(
                &contract,
                &packet,
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
            )],
        );

        assert_eq!(
            candidates[0].candidate_identity().as_str(),
            format!(
                "derived:{}:exact-advisory-match",
                target.fingerprint_identity().as_str()
            )
        );
    }
}
