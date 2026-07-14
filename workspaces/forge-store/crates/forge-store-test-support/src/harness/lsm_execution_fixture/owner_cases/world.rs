use forge_store_layout_indexes::{
    layout_lsm_maintenance, lsm_strategy, BaselineLsmCompactionAdmission,
    LsmCompactionAdmissionRequest,
};
use forge_store_lsm_authority::{
    AdmittedLsmMembershipReplacement, LsmCompactionMembership, LsmMembershipArtifactDeclaration,
    LsmMembershipKey, LsmMembershipRecord, LsmMembershipSession,
};
use forge_store_wal::{
    admit_checkpoint_publication, admit_durable_append, AdmittedWalAppendReceipt,
    BlobWalRecordKind, StoreWalRecordIdentity, WalSecurityMetadataCarrier,
};

use super::super::{
    begin_durability_fixture, durable_record_binding, durable_record_binding_for_store,
    manifest_receipt_for_artifact, physical_compaction_fixture, wal_receipt, wal_scope,
    PreExecutionBudgetEnvelope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    WalRecordFamily,
};

pub(super) struct CompleteMembershipWorld {
    pub session: LsmMembershipSession,
    pub key: LsmMembershipKey,
    pub selected: LsmCompactionMembership,
    pub anchor: AdmittedWalAppendReceipt,
    pub record_paths: [std::path::PathBuf; 3],
}

pub(super) struct ReplacementWorld {
    pub session: LsmMembershipSession,
    pub key: LsmMembershipKey,
    pub selected: LsmCompactionMembership,
    pub replacement: AdmittedLsmMembershipReplacement,
    pub anchor: AdmittedWalAppendReceipt,
    pub record_paths: [std::path::PathBuf; 3],
    pub activation_path: std::path::PathBuf,
    pub output_path: std::path::PathBuf,
}

pub(super) fn admission_and_key(
    sequence: u64,
) -> (BaselineLsmCompactionAdmission, LsmMembershipKey) {
    let security =
        forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
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

pub(super) fn empty_session(anchor: &AdmittedWalAppendReceipt) -> LsmMembershipSession {
    let security =
        forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    forge_store_lsm_authority::open_lsm_membership(anchor, security.witnesses())
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
    let selected = forge_store_lsm_authority::select_lsm_compaction_membership(&session, key)
        .into_result()
        .unwrap();
    CompleteMembershipWorld {
        session,
        key,
        selected,
        record_paths: [
            anchor.persisted_path().to_path_buf(),
            publication_durable.persisted_path().to_path_buf(),
            tombstone_durable.persisted_path().to_path_buf(),
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
    let anchor = admit_durable_append(&wal_receipt(anchor_scope, b"a")).unwrap();
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
) -> (LsmMembershipRecord, AdmittedWalAppendReceipt) {
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

pub(super) fn replacement_world() -> ReplacementWorld {
    let CompleteMembershipWorld {
        session,
        key,
        selected,
        anchor,
        record_paths,
    } = complete_membership();
    let (physical_intent, physical_publication) = physical_compaction_fixture();
    let output_scope = wal_scope(
        selected.expected_output_identity().unwrap().sequence(),
        selected.compaction_output_digest(
            physical_intent.root_scope(),
            physical_intent.target_epoch(),
            physical_intent.manifest_epoch(),
        ),
        4096,
    );
    let output_artifact = LsmMembershipArtifactDeclaration::compaction_output(&output_scope);
    let output_durable =
        admit_durable_append(&wal_receipt(output_scope, output_artifact.bytes())).unwrap();
    let output = forge_store_lsm_authority::admit_lsm_replacement_output(
        &selected,
        output_durable,
        physical_intent,
    )
    .unwrap();
    let output_path = output.persisted_path().to_path_buf();
    let activation = forge_store_lsm_authority::prepare_lsm_membership_activation(
        &selected,
        output,
        &physical_publication,
    )
    .unwrap();
    let artifact = activation.artifact();
    let checkpoint = admit_checkpoint_publication(&manifest_receipt_for_artifact(
        activation.scope().clone(),
        artifact.bytes(),
    ))
    .unwrap();
    let activation_path = checkpoint.persisted_path().to_path_buf();
    let replacement = forge_store_lsm_authority::admit_lsm_membership_replacement(
        &selected, activation, checkpoint,
    )
    .unwrap();
    ReplacementWorld {
        session,
        key,
        selected,
        replacement,
        anchor,
        record_paths,
        activation_path,
        output_path,
    }
}

pub(super) fn persist_untrusted_artifact(sequence: u64, bytes: &[u8]) {
    let scope = wal_scope(
        sequence,
        format!("untrusted-lsm-membership-artifact:{sequence}"),
        bytes.len() as u64,
    );
    admit_durable_append(&wal_receipt(scope, bytes)).unwrap();
}
