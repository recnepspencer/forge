use crate::{
    derived_index_damage_tests::inspect_with_damaged_authority,
    courtroom::harness::test_support::physical_scope_admission_test_support::{
        root_with_slot, scope_membership, validation, with_checked_frame,
    },
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ExecutedQuarantineFinding, ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest,
    PhysicalQuarantineAuthority, PhysicalScopeAdmission, PhysicalScopeAdmissionRequest,
    QuarantineRecord, QuarantineSealRequest, ScopedPhysicalValidatorInput,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    IntegrityDamageMap, RecoveryBlockedByIntegrityDamage, S4IntegrityHandoffDenialKind,
};

#[test]
fn damage_map_rejects_cross_bucket_recovery_blockers() {
    let wal_damage = inspect_wal_denial(
        wal_payload("crc32c", 4, "checksum-fail", b"DATA"),
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
    );
    let checkpoint_damage = inspect_wal_denial(
        wal_payload("crc32c", 4, "checkpoint-damage", b"DATA"),
        CheckpointAdjacencyPosture::CheckpointAdjacent,
    );
    let manifest_damage = ManifestIntegrityAuthority::s3()
        .inspect_manifest(ManifestIntegrityInspectionRequest::damaged_root(
            root_with_slot(1, 2, 3, 7).root_publication().owner(),
        ))
        .unwrap_err();
    let unresolved = RecoveryBlockedByIntegrityDamage::unresolved_authority_damage(
        &unresolved_authority_record(),
    )
    .unwrap();

    assert_damage_map_source_mismatch(
        IntegrityDamageMap::new()
            .with_unresolved_authority_damage(
                RecoveryBlockedByIntegrityDamage::damaged_manifest_root(&manifest_damage),
            )
            .unwrap_err()
            .kind(),
    );
    assert_damage_map_source_mismatch(
        IntegrityDamageMap::new()
            .with_wal_damage(
                RecoveryBlockedByIntegrityDamage::checkpoint_adjacent_damage(&checkpoint_damage),
            )
            .unwrap_err()
            .kind(),
    );
    assert_damage_map_source_mismatch(
        IntegrityDamageMap::new()
            .with_manifest_root_damage(RecoveryBlockedByIntegrityDamage::damaged_wal_frame(
                &wal_damage,
            ))
            .unwrap_err()
            .kind(),
    );
    assert_damage_map_source_mismatch(
        IntegrityDamageMap::new()
            .with_checkpoint_damage(unresolved)
            .unwrap_err()
            .kind(),
    );
}

fn inspect_wal_denial(
    payload: Vec<u8>,
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    let mut denial = None;
    with_wal_payload_input(payload, adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        denial = Some(
            WalFrameIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn with_wal_payload_input(
    payload: Vec<u8>,
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let validation = validation(1, 2, 3, 7);
    let scope = PhysicalReferenceScope::wal_frame(validation);
    let root = root_with_slot(1, 2, 3, 7);
    let membership = scope_membership(&root, scope);
    with_checked_frame(&payload, validation, |checked| {
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            adjacency,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        run(ScopedPhysicalValidatorInput::wal_frame(admission).unwrap());
    });
}

fn assert_damage_map_source_mismatch(kind: S4IntegrityHandoffDenialKind) {
    assert_eq!(kind, S4IntegrityHandoffDenialKind::DamageMapSourceMismatch);
}

fn unresolved_authority_record() -> QuarantineRecord {
    let finding =
        ExecutedQuarantineFinding::from_index_page_denial(&inspect_with_damaged_authority())
            .unwrap();
    PhysicalQuarantineAuthority::seal(QuarantineSealRequest::from_executed_finding(finding))
        .unwrap()
}

fn wal_payload(algorithm: &str, declared_len: usize, status: &str, body: &[u8]) -> Vec<u8> {
    let mut payload = format!("WALF|{algorithm}|{declared_len}|{status}|").into_bytes();
    payload.extend_from_slice(body);
    payload
}
