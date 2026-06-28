use crate::{
    physical_scope_admission_test_support::{
        root_with_slot, scope_membership, validation, with_checked_frame,
    },
    s4_integrity_handoff_test_support::unresolved_authority_record,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest, PhysicalScopeAdmission,
    PhysicalScopeAdmissionRequest, ScopedPhysicalValidatorInput, WalFrameDamageDenialKind,
    WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest,
};
use forge_store_recovery_physics::{
    IntegrityDamageMap, RecoveryBlockedByIntegrityDamage, RecoveryBlockingIntegritySource,
};

pub(crate) struct RecoveryBlockingDamageFixture {
    damage_map: IntegrityDamageMap,
    wal_kind: WalFrameDamageDenialKind,
    checkpoint_kind: WalFrameDamageDenialKind,
}

impl RecoveryBlockingDamageFixture {
    pub(crate) fn damage_map(&self) -> &IntegrityDamageMap {
        &self.damage_map
    }

    pub(crate) fn wal_kind(&self) -> WalFrameDamageDenialKind {
        self.wal_kind
    }

    pub(crate) fn checkpoint_kind(&self) -> WalFrameDamageDenialKind {
        self.checkpoint_kind
    }
}

pub(crate) fn recovery_blocking_damage_fixture() -> RecoveryBlockingDamageFixture {
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
    let damage_map = IntegrityDamageMap::new()
        .with_wal_damage(RecoveryBlockedByIntegrityDamage::damaged_wal_frame(
            &wal_damage,
        ))
        .unwrap()
        .with_checkpoint_damage(
            RecoveryBlockedByIntegrityDamage::checkpoint_adjacent_damage(&checkpoint_damage),
        )
        .unwrap()
        .with_manifest_root_damage(RecoveryBlockedByIntegrityDamage::damaged_manifest_root(
            &manifest_damage,
        ))
        .unwrap()
        .with_unresolved_authority_damage(
            RecoveryBlockedByIntegrityDamage::unresolved_authority_damage(
                &unresolved_authority_record(),
            )
            .unwrap(),
        )
        .unwrap();
    RecoveryBlockingDamageFixture {
        damage_map,
        wal_kind: wal_damage.kind(),
        checkpoint_kind: checkpoint_damage.kind(),
    }
}

pub(crate) fn recovery_blocking_wal_damage_map() -> IntegrityDamageMap {
    let wal_damage = inspect_wal_denial(
        wal_payload("crc32c", 4, "checksum-fail", b"DATA"),
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
    );
    IntegrityDamageMap::new()
        .with_wal_damage(RecoveryBlockedByIntegrityDamage::damaged_wal_frame(
            &wal_damage,
        ))
        .expect("S.3 WAL damage publishes recovery blocker")
}

pub(crate) fn assert_all_recovery_blocking_sources(map: &IntegrityDamageMap) {
    assert_eq!(
        map.wal_damage()[0].source(),
        RecoveryBlockingIntegritySource::WalFrame
    );
    assert_eq!(
        map.checkpoint_damage()[0].source(),
        RecoveryBlockingIntegritySource::CheckpointAdjacentRecord
    );
    assert_eq!(
        map.manifest_root_damage()[0].source(),
        RecoveryBlockingIntegritySource::ManifestRoot
    );
    assert_eq!(
        map.unresolved_authority_damage()[0].source(),
        RecoveryBlockingIntegritySource::UnresolvedAuthorityDamage
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

fn wal_payload(algorithm: &str, declared_len: usize, status: &str, body: &[u8]) -> Vec<u8> {
    let mut payload = format!("WALF|{algorithm}|{declared_len}|{status}|").into_bytes();
    payload.extend_from_slice(body);
    payload
}
