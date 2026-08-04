use worth_store_layout_indexes::{
    layout_lsm_maintenance, lsm_strategy, BaselineLsmCompactionAdmission,
    LsmCompactionAdmissionRequest,
};
use worth_store_lsm_authority::{LsmMembershipKey, LsmMembershipRecord, LsmMembershipSession};
use worth_store_wal::{
    BlobWalRecordKind, StoreWalRecordIdentity, WalFrameArtifactObservation,
    WalSecurityMetadataCarrier,
};

use super::super::{
    begin_durability_fixture, durable_record_binding, durable_record_binding_for_store,
    wal_artifact_observation, wal_scope, PreExecutionBudgetEnvelope, StoreKeyVersionPosture,
    StoreLegacySecurityPosture, WalRecordFamily,
};

pub(super) struct CompleteMembershipWorld {
    pub session: LsmMembershipSession,
    pub key: LsmMembershipKey,
    pub anchor: WalFrameArtifactObservation,
    pub record_paths: [std::path::PathBuf; 3],
    pub record_frame_offsets: [u64; 3],
}

pub(super) fn admission_and_key(
    sequence: u64,
) -> (BaselineLsmCompactionAdmission, LsmMembershipKey) {
    let security =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let admission = layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(sequence),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .unwrap();
    let metadata = WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let key = lsm_strategy()
        .admit_key(metadata, admission.clone())
        .unwrap();
    (admission, key)
}

pub(super) fn empty_session(anchor: &WalFrameArtifactObservation) -> LsmMembershipSession {
    let security =
        worth_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    worth_store_lsm_authority::open_lsm_membership(anchor, security.witnesses())
        .into_result()
        .unwrap()
}

pub(super) fn complete_membership() -> CompleteMembershipWorld {
    begin_durability_fixture();
    let (_, key) = admission_and_key(43);
    let (first_envelope, anchor) = durable_record_binding(key, 41, BlobWalRecordKind::LsmValue);
    let mut session = empty_session(&anchor);
    lsm_strategy()
        .persist_record(&mut session, first_envelope, &anchor, key)
        .unwrap();
    let (publication_envelope, publication_durable) =
        durable_record_binding(key, 42, BlobWalRecordKind::GenerationPublication);
    lsm_strategy()
        .persist_record(
            &mut session,
            publication_envelope,
            &publication_durable,
            key,
        )
        .unwrap();
    let (tombstone_envelope, tombstone_durable) =
        durable_record_binding(key, 43, BlobWalRecordKind::LsmTombstone);
    lsm_strategy()
        .persist_record(&mut session, tombstone_envelope, &tombstone_durable, key)
        .unwrap();
    CompleteMembershipWorld {
        session,
        key,
        record_paths: [
            anchor.path().to_path_buf(),
            publication_durable.path().to_path_buf(),
            tombstone_durable.path().to_path_buf(),
        ],
        record_frame_offsets: [
            anchor.frame_offset(),
            publication_durable.frame_offset(),
            tombstone_durable.frame_offset(),
        ],
        anchor,
    }
}

pub(super) fn membership_missing(
    missing: BlobWalRecordKind,
) -> (LsmMembershipSession, LsmMembershipKey) {
    begin_durability_fixture();
    let (_, key) = admission_and_key(83);
    let anchor_scope = wal_scope(80, "lsm-membership-selection-anchor".to_owned(), 1);
    let anchor = wal_artifact_observation(anchor_scope, b"a");
    let mut session = empty_session(&anchor);
    let (value_envelope, value_durable) =
        durable_record_binding(key, 81, BlobWalRecordKind::LsmValue);
    let records = [
        (value_envelope, value_durable, BlobWalRecordKind::LsmValue),
        {
            let (envelope, durable) =
                durable_record_binding(key, 82, BlobWalRecordKind::GenerationPublication);
            (envelope, durable, BlobWalRecordKind::GenerationPublication)
        },
        {
            let (envelope, durable) =
                durable_record_binding(key, 83, BlobWalRecordKind::LsmTombstone);
            (envelope, durable, BlobWalRecordKind::LsmTombstone)
        },
    ];
    for (envelope, durable, kind) in records {
        if kind != missing {
            lsm_strategy()
                .persist_record(&mut session, envelope, &durable, key)
                .unwrap();
        }
    }
    (session, key)
}

pub(super) fn admitted_record(
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> (LsmMembershipRecord, WalFrameArtifactObservation) {
    let (envelope, durable) = durable_record_binding(key, sequence, kind);
    (
        LsmMembershipRecord::admit(envelope, &durable, key).unwrap(),
        durable,
    )
}

pub(super) fn foreign_store_record(
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> LsmMembershipRecord {
    let (envelope, durable) = durable_record_binding_for_store(key, sequence, kind, 2, 1);
    LsmMembershipRecord::admit(envelope, &durable, key).unwrap()
}

pub(super) fn persist_untrusted_artifact(sequence: u64, bytes: &[u8]) {
    let scope = wal_scope(
        sequence,
        format!("untrusted-lsm-membership-artifact:{sequence}"),
        bytes.len() as u64,
    );
    let _observation = wal_artifact_observation(scope, bytes);
}
