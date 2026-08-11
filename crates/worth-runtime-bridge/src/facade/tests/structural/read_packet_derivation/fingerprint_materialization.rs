use super::*;

#[test]
fn runtime_materializes_structural_fingerprint_from_truth_view_read() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = registered_structural(
        "structural:analysis-snapshot",
        StructuralFingerprintFamily::TopologyFingerprint,
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
            crate::truth_identity_fixtures::truth_branch_fixture("analysis"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        )),
    );
    let contract = runtime
        .admit_structural_comparison(declaration)
        .expect("registered structural declaration should be admitted");

    let fingerprint = runtime
        .materialize_structural_fingerprint(
            &contract,
            SnapshotReadPacket::new(vec![crate::snapshot::SnapshotReadRequest::for_coarse(
                "entity-1",
                crate::snapshot::SnapshotReadContract::scalar(
                    worth_foundational::facade::AspectKey::new("profile")
                        .expect("valid snapshot aspect key"),
                    worth_foundational::facade::ScalarAspectType::String,
                ),
            )]),
        )
        .expect("structural fingerprint should materialize");

    assert_eq!(
        fingerprint.family(),
        StructuralFingerprintFamily::TopologyFingerprint
    );
    assert_eq!(
        fingerprint.snapshot_identity(),
        &crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a")
    );
    assert_eq!(
        fingerprint.snapshot_identity_text(),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a").as_str()
    );
    assert!(fingerprint
        .digest()
        .starts_with("structural-fingerprint:sha256:"));
    assert_eq!(fingerprint.record_value_evidence().records().len(), 1);
    assert_eq!(fingerprint.equivalence_member_evidence().members().len(), 1);
    assert!(fingerprint.record_value_evidence().records()[0]
        .canonical_basis()
        .starts_with("structural-record-value-evidence|"));
    assert!(fingerprint.equivalence_member_evidence().members()[0]
        .canonical_basis()
        .starts_with("structural-equivalence-member|"));
    assert!(fingerprint
        .canonical_basis()
        .contains("record-values=structural-record-value-evidence-set|"));
    assert!(fingerprint
        .canonical_basis()
        .contains("equivalence-members=structural-equivalence-member-set|"));
}
