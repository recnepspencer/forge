use worth_store_lsm_authority::{
    open_lsm_membership, reopen_lsm_membership_from_store, LsmMembershipOwnerCaseObservation,
};
use worth_store_wal::{AdmittedWalArtifactStore, BlobWalRecordKind};

use super::super::begin_durability_fixture;
use super::world;

pub(super) fn observe() -> Vec<LsmMembershipOwnerCaseObservation> {
    vec![
        admitted(),
        canonical_key_required(),
        durable_record_binding_mismatch(),
        unsupported_record_kind(),
        membership_ambiguous(),
        membership_stale(),
        manifest_membership_mismatch(),
        replacement_output_mismatch(),
        persisted_membership_artifact_invalid(),
        io(),
    ]
}

fn admitted() -> LsmMembershipOwnerCaseObservation {
    let world = world::replacement_world();
    open(&world.anchor)
}

fn canonical_key_required() -> LsmMembershipOwnerCaseObservation {
    let world = world::complete_membership();
    let foreign_scope =
        worth_store_security::admitted_tenant_wal_checkpoint_security_scope_for_layout_partition_test();
    open_lsm_membership(&world.anchor, foreign_scope.witnesses()).owner_case_observation()
}

fn durable_record_binding_mismatch() -> LsmMembershipOwnerCaseObservation {
    let world = world::complete_membership();
    let mut bytes = std::fs::read(world.anchor.persisted_path()).unwrap();
    bytes[b"worth-store:wal-lsm-membership:v1 ".len()] ^= 1;
    std::fs::write(world.anchor.persisted_path(), bytes).unwrap();
    open(&world.anchor)
}

fn unsupported_record_kind() -> LsmMembershipOwnerCaseObservation {
    begin_durability_fixture();
    let (_, key) = world::admission_and_key(81);
    let (_, anchor) = world::admitted_record(key, 81, BlobWalRecordKind::RootCandidate);
    open(&anchor)
}

fn membership_ambiguous() -> LsmMembershipOwnerCaseObservation {
    begin_durability_fixture();
    let (_, key) = world::admission_and_key(91);
    let (_, anchor) = world::admitted_record(key, 90, BlobWalRecordKind::LsmValue);
    world::admitted_record(key, 91, BlobWalRecordKind::LsmValue);
    open(&anchor)
}

fn membership_stale() -> LsmMembershipOwnerCaseObservation {
    let world = world::replacement_world();
    let store = AdmittedWalArtifactStore::open(&world.anchor).unwrap();
    for path in &world.record_paths {
        std::fs::remove_file(path).unwrap();
    }
    reopen(store)
}

fn manifest_membership_mismatch() -> LsmMembershipOwnerCaseObservation {
    let world = world::replacement_world();
    std::fs::remove_file(&world.record_paths[2]).unwrap();
    open(&world.anchor)
}

fn replacement_output_mismatch() -> LsmMembershipOwnerCaseObservation {
    let world = world::replacement_world();
    std::fs::remove_file(&world.output_path).unwrap();
    open(&world.anchor)
}

fn persisted_membership_artifact_invalid() -> LsmMembershipOwnerCaseObservation {
    let world = world::complete_membership();
    let body = b"worth-store:wal-lsm-membership:v1 0";
    let malformed = format!(
        "{} {:016x}",
        std::str::from_utf8(body).unwrap(),
        checksum(body)
    );
    world::persist_untrusted_artifact(101, malformed.as_bytes());
    open(&world.anchor)
}

fn io() -> LsmMembershipOwnerCaseObservation {
    let world = world::complete_membership();
    let store = AdmittedWalArtifactStore::open(&world.anchor).unwrap();
    let root = world
        .anchor
        .persisted_path()
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    std::fs::remove_dir_all(root).unwrap();
    let current_scope =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    reopen_lsm_membership_from_store(store, current_scope.witnesses()).owner_case_observation()
}

fn open(anchor: &worth_store_wal::AdmittedWalAppendReceipt) -> LsmMembershipOwnerCaseObservation {
    let current_scope =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    open_lsm_membership(anchor, current_scope.witnesses()).owner_case_observation()
}

fn reopen(store: AdmittedWalArtifactStore) -> LsmMembershipOwnerCaseObservation {
    let current_scope =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    reopen_lsm_membership_from_store(store, current_scope.witnesses()).owner_case_observation()
}

fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}
