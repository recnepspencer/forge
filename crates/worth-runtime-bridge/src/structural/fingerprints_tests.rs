use crate::snapshot::BridgeTruthViewSelector;
use crate::structural::{
    AdmittedStructuralRegistry, StructuralFingerprintEquivalenceContract,
    StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
    StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity,
    StructuralTruthViewBasis,
};

use super::StructuralFingerprint;

#[test]
fn fingerprint_value_evidence_canonicalizes_authoritative_absence_without_panicking() {
    let read = crate::snapshot::SnapshotReadRequest::for_coarse(
        "absent-fingerprint-fixture",
        crate::snapshot::SnapshotReadContract::scalar(
            worth_foundational::facade::AspectKey::new("status").unwrap(),
            worth_foundational::facade::ScalarAspectType::String,
        ),
    );
    let record =
        crate::snapshot::ValidatedSnapshotReadRecord::absent(read.correlation_id().clone());

    let evidence = super::StructuralFingerprintRecordValueEvidence::from_validated_record(&record);

    assert!(evidence.canonical_basis().contains("correlation="));
    assert!(evidence
        .aspect_value_digest()
        .starts_with("structural-record-aspect-value:sha256:"));
}

#[test]
fn fingerprint_is_canonical_for_same_contract_and_read_packet() {
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
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::committed_snapshot(
            crate::truth_identity_fixtures::truth_branch_fixture("main"),
            crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
        )),
    );
    let registry = AdmittedStructuralRegistry::freeze(vec![declaration]).unwrap();
    let contract = registry.contracts()[0].clone();
    let read_packet = crate::snapshot::SnapshotReadPacket::new(vec![]);

    let snapshot_identity = crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a");
    let left = StructuralFingerprint::from_snapshot_read_packet(
        &contract,
        &read_packet,
        snapshot_identity.clone(),
    );
    let right = StructuralFingerprint::from_snapshot_read_packet(
        &contract,
        &read_packet,
        snapshot_identity,
    );

    assert_eq!(left, right);
    assert_eq!(
        left.family(),
        StructuralFingerprintFamily::TopologyFingerprint
    );
    assert!(left.record_value_evidence().records().is_empty());
    assert!(left.equivalence_member_evidence().members().is_empty());
    assert_eq!(
        left.snapshot_identity(),
        &crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a")
    );
    assert_eq!(
        left.snapshot_identity_text(),
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a").as_str()
    );
}
