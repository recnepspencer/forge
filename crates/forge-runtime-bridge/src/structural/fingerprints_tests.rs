use crate::input::envelope::TruthBranchIdentity;
use crate::snapshot::{BridgeTruthViewSelector, TruthSnapshotIdentity};
use crate::structural::{
    AdmittedStructuralRegistry, StructuralFingerprintEquivalenceContract,
    StructuralFingerprintFamily, StructuralFingerprintNormalizationRule,
    StructuralFingerprintOmissionPolicy, StructuralFingerprintOrderingRule,
    StructuralIdentityDeclaration, StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity,
    StructuralTruthViewBasis,
};

use super::StructuralFingerprint;

#[test]
fn fingerprint_is_canonical_for_same_contract_and_read_packet() {
    let declaration = StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::new("structural:geometry"),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            StructuralFingerprintFamily::TopologyFingerprint,
            "topology-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::committed_snapshot(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot-a"),
        )),
    );
    let registry = AdmittedStructuralRegistry::freeze(vec![declaration]).unwrap();
    let contract = registry.contracts()[0].clone();
    let read_packet = crate::snapshot::SnapshotReadPacket::new(vec![]);

    let snapshot_identity = TruthSnapshotIdentity::new("snapshot-a");
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
        &TruthSnapshotIdentity::new("snapshot-a")
    );
    assert_eq!(left.snapshot_identity_text(), "snapshot-a");
}
