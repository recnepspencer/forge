use super::{
    BaselineLsmCounterObservation, BaselineLsmExecutionWitness, BaselineLsmLookupDisposition,
};
use crate::BlobWalRecordKind;

fn admitted_key(bytes: [u8; 8]) -> super::BaselineLsmAdmittedKey {
    use forge_store_security::{
        admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test,
        StoreKeyVersionPosture, StoreLegacySecurityPosture,
    };
    let security = admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test();
    let metadata = crate::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    super::BaselineLsmAdmittedKey::admit(metadata, bytes).unwrap()
}

#[test]
fn baseline_lsm_execution_selects_memtable_before_sorted_run() {
    let witness = BaselineLsmExecutionWitness::seeded();
    assert_eq!(
        witness.lookup_disposition_for(43),
        BaselineLsmLookupDisposition::Memtable
    );
    assert_eq!(
        witness.lookup_disposition_for(42),
        BaselineLsmLookupDisposition::SortedRun
    );
    assert_eq!(
        witness.lookup_disposition_for(99),
        BaselineLsmLookupDisposition::NotFound
    );
}

#[test]
fn baseline_lsm_execution_observes_named_strategy_lanes() {
    let witness = BaselineLsmExecutionWitness::seeded();
    assert_eq!(
        witness.execute_lookup_latest_visible_record(43).counters(),
        BaselineLsmCounterObservation::new(1, 1, 0, 0, 0)
    );
    assert_eq!(
        witness.execute_manifest_publication().counters(),
        BaselineLsmCounterObservation::new(0, 0, 0, 2, 2)
    );
    assert_eq!(
        witness.execute_replay_wal_tail().counters(),
        BaselineLsmCounterObservation::new(0, 0, 3, 0, 1)
    );
}

#[test]
fn durable_executor_mints_one_identity_and_exact_publication_receipt() {
    let receipt = BaselineLsmExecutionWitness::seeded()
        .compaction_publication_receipt()
        .clone();
    assert!(receipt
        .compaction_transition()
        .is_tombstone_retention_admitted());
    assert_eq!(receipt.input_runs().len(), 3);
    assert!(receipt
        .input_runs()
        .windows(2)
        .all(|pair| pair[0].generation() < pair[1].generation()));
    assert!(receipt.output_generation() > receipt.input_runs()[2].generation());
    assert_eq!(
        receipt.tombstone_record().key(),
        receipt.retired_value_record().key()
    );
    assert!(receipt.tombstone_blocks_older());
    assert!(receipt.stale_runs_retired());
    assert!(receipt.publication_is_bound());
    assert_eq!(receipt.counters().publications(), 1);
    assert_eq!(
        receipt.counters().maintenance_reads(),
        receipt.rewritten_runs()
    );
    assert_eq!(receipt.bytes_in(), 12_288);
    assert_eq!(receipt.bytes_out(), 4_096);
    assert_eq!(receipt.target_physical_epoch(), 2);
    assert_eq!(receipt.physical_publication().root_scope(), 2);
    assert_eq!(receipt.physical_publication().manifest_epoch(), 2);
    assert_eq!(
        receipt.tombstone_record().wal_record().kind(),
        BlobWalRecordKind::LsmTombstone
    );
}

#[test]
fn generated_output_is_bound_to_durable_input_generation() {
    let witness = BaselineLsmExecutionWitness::seeded();
    let receipt = witness.compaction_publication_receipt();
    assert_eq!(
        receipt.output_generation(),
        witness.replay_tail()[2].identity().sequence() + 1
    );
    assert_eq!(
        receipt.output_publication().identity(),
        receipt.output_run().root_record()
    );
    assert_eq!(
        receipt.replay_binding(),
        &witness
            .replay_tail()
            .clone()
            .map(|record| record.identity())
    );
}

#[test]
fn manifest_membership_identity_binds_every_key_byte_and_record() {
    let records = [
        crate::BlobWalRecordIdentity::new(41, BlobWalRecordKind::LsmValue).unwrap(),
        crate::BlobWalRecordIdentity::new(42, BlobWalRecordKind::GenerationPublication).unwrap(),
        crate::BlobWalRecordIdentity::new(43, BlobWalRecordKind::LsmTombstone).unwrap(),
    ];
    let first = super::baseline_lsm_manifest_membership_digest(
        admitted_key(*b"same-001"),
        records,
        "store-a",
    );
    let distinct_key = super::baseline_lsm_manifest_membership_digest(
        admitted_key(*b"same-002"),
        records,
        "store-a",
    );
    let mut incomplete = records;
    incomplete[1] =
        crate::BlobWalRecordIdentity::new(44, BlobWalRecordKind::GenerationPublication).unwrap();
    let distinct_membership = super::baseline_lsm_manifest_membership_digest(
        admitted_key(*b"same-001"),
        incomplete,
        "store-a",
    );
    assert_ne!(first, distinct_key);
    assert_ne!(first, distinct_membership);
}
