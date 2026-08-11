use crate::logic::transaction::{
    branch_state_proof_report, canonical_digest, BranchStateDenseGridProofBasis,
    BranchStateProofBasis, ReplayArtifactProofInput, ReplayMismatchClass,
    BRANCH_STATE_PROOF_BASIS_VERSION, MERGE_PROOF_SCHEMA_VERSION,
};
use std::collections::BTreeMap;

#[test]
fn branch_state_proof_basis_is_stable_at_core_boundary() {
    let basis = BranchStateProofBasis {
        proof_schema_version: BRANCH_STATE_PROOF_BASIS_VERSION.to_owned(),
        catalog_ids: vec!["gearTeeth".to_owned(), "hudModel".to_owned()],
        dense_grids: vec![BranchStateDenseGridProofBasis {
            family_id: "gearToothModel".to_owned(),
            width: 8,
            height: 1,
            key_count: 8,
            ids: vec!["tooth-0".to_owned(), "tooth-1".to_owned()],
        }],
        store: BTreeMap::from([
            ("gearTeeth".to_owned(), 22_u64),
            ("lightIntensity".to_owned(), 178_u64),
        ]),
    };

    let left = branch_state_proof_report(
        7,
        "main",
        Some(42),
        BRANCH_STATE_PROOF_BASIS_VERSION,
        &basis,
    );
    let right = branch_state_proof_report(
        7,
        "main",
        Some(42),
        BRANCH_STATE_PROOF_BASIS_VERSION,
        &basis,
    );

    assert_eq!(
        left.proof_schema_version,
        format!(
            "{}:{}",
            MERGE_PROOF_SCHEMA_VERSION, BRANCH_STATE_PROOF_BASIS_VERSION
        )
    );
    assert_eq!(left.state_digest, right.state_digest);
    assert_eq!(left.state_digest, canonical_digest(&basis));
}

#[test]
fn replay_artifact_proof_report_surfaces_typed_mismatch_classes() {
    let expected = ReplayArtifactProofInput {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        registry_bundle_digest: Some("registry-a".to_owned()),
        lowered_strategy_bundle_digest: Some("bundle-a".to_owned()),
        merge_plan_digest: Some("plan-a".to_owned()),
        merge_result_digest: Some("result-a".to_owned()),
        lineage_digest: Some("lineage-a".to_owned()),
        strategy_witness: None,
        compatibility_witness: None,
        scoped_merge_proof: None,
        branch_state_digest: "state-a".to_owned(),
    };
    let replayed = ReplayArtifactProofInput {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        registry_bundle_digest: Some("registry-b".to_owned()),
        lowered_strategy_bundle_digest: Some("bundle-b".to_owned()),
        merge_plan_digest: Some("plan-b".to_owned()),
        merge_result_digest: Some("result-b".to_owned()),
        lineage_digest: Some("lineage-b".to_owned()),
        strategy_witness: None,
        compatibility_witness: None,
        scoped_merge_proof: None,
        branch_state_digest: "state-b".to_owned(),
    };

    let report =
        crate::logic::transaction::replay_artifact_proof_report(expected.clone(), replayed.clone());

    assert!(!report.parity);
    assert_eq!(
        report.expected.registry_bundle_digest,
        expected.registry_bundle_digest
    );
    assert_eq!(
        report.replayed.branch_state_digest,
        replayed.branch_state_digest
    );
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::RegistryBundleDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::LoweredStrategyBundleDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::MergePlanDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::MergeResultDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::LineageDigestMismatch));
    assert!(report
        .mismatch_classes
        .contains(&ReplayMismatchClass::BranchStateDigestMismatch));
}
