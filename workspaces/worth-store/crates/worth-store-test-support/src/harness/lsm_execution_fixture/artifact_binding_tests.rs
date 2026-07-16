use super::*;

#[test]
fn durable_scope_cannot_authorize_different_wal_bytes() {
    let access = lsm_strategy();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let metadata = worth_store_wal::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let compaction = layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(99),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    let key = access.admit_key(metadata, compaction).unwrap();
    let scope = wal_scope(91, "claimed-frame".into(), 11);
    let receipt = admit_durable_append(&wal_receipt(scope.clone(), b"wrong-bytes")).unwrap();
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(91, BlobWalRecordKind::LsmValue).unwrap(),
        DurablePublicationDeclaration::wal_frame(scope),
        "claimed-frame",
    )
    .unwrap();
    let mut index = open_lsm_index(&receipt).unwrap();
    assert_eq!(
        access.persist_record(&mut index, envelope, &receipt, key),
        Err(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)
    );
}

#[test]
fn duplicate_active_component_is_denied_before_membership_admission() {
    let (access, key) = admitted_test_index(99);
    let (first_envelope, first) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
    let mut index = open_lsm_index(&first).unwrap();
    access
        .persist_record(&mut index, first_envelope, &first, key)
        .unwrap();
    let (duplicate_envelope, duplicate) =
        durable_record_binding(key, 92, BlobWalRecordKind::LsmValue);

    assert_eq!(
        access.persist_record(&mut index, duplicate_envelope, &duplicate, key),
        Err(BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous)
    );
}

#[test]
fn segment_or_generation_substitution_is_denied_before_membership_admission() {
    let (access, key) = admitted_test_index(99);
    let (_, anchor) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
    let mut index = open_lsm_index(&anchor).unwrap();
    let (foreign_envelope, foreign) =
        durable_record_binding_for_store(key, 92, BlobWalRecordKind::GenerationPublication, 2, 7);

    assert_eq!(
        access.persist_record(&mut index, foreign_envelope, &foreign, key),
        Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch)
    );
}

#[test]
fn equal_scope_from_a_different_store_directory_is_denied() {
    begin_durability_fixture();
    let (access, key) = admitted_test_index(99);
    let (first_envelope, anchor) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
    let mut index = open_lsm_index(&anchor).unwrap();
    access
        .persist_record(&mut index, first_envelope, &anchor, key)
        .unwrap();

    begin_durability_fixture();
    let (foreign_envelope, foreign) =
        durable_record_binding(key, 92, BlobWalRecordKind::GenerationPublication);

    assert_eq!(
        access.persist_record(&mut index, foreign_envelope, &foreign, key),
        Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch)
    );
}

#[test]
fn artifact_swap_after_record_admission_is_denied_before_membership_admission() {
    let (_access, key) = admitted_test_index(99);
    let (envelope, durable) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
    let record = LsmMembershipRecord::admit(envelope, &durable, key).unwrap();
    let mut index = open_lsm_index(&durable).unwrap();
    use std::io::{Seek, SeekFrom, Write};
    let mut artifact = std::fs::OpenOptions::new()
        .write(true)
        .open(durable.persisted_path())
        .unwrap();
    artifact
        .seek(SeekFrom::Start(durable.persisted_offset()))
        .unwrap();
    artifact.write_all(b"substituted-after-admission").unwrap();

    assert_eq!(
        worth_store_lsm_authority::persist_lsm_membership_record(&mut index, record).into_result(),
        Err(worth_store_lsm_authority::LsmMembershipDenial::DurableRecordBindingMismatch)
    );
}

fn admitted_test_index(sequence: u64) -> (LsmStrategy, LsmMembershipKey) {
    let access = lsm_strategy();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let metadata = worth_store_wal::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let compaction = layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(sequence),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    let key = access.admit_key(metadata, compaction).unwrap();
    (access, key)
}
