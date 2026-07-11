use crate::layout_access::baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial;
use crate::DurablePublicationDeclaration;
use forge_store_security::{
    admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test, StoreKeyVersionPosture,
    StoreLegacySecurityPosture,
};

use super::*;

#[test]
fn incomplete_and_duplicate_membership_are_owner_denials() {
    let key = admitted_key(*b"key-0001");
    let mut persisted = test_session();
    let value = record(&persisted, key, 1, BlobWalRecordKind::LsmValue);
    persisted.persist(value).unwrap();
    assert_eq!(
        BaselineLsmCompactionPlan::lower_from_persisted(&persisted, key),
        Err(super::super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete)
    );
    assert_eq!(
        {
            let duplicate = record(&persisted, key, 2, BlobWalRecordKind::LsmValue);
            persisted.persist(duplicate)
        },
        Err(super::super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous)
    );
}

#[test]
fn unrelated_key_appends_do_not_stale_or_expand_membership_selection() {
    let selected_key = admitted_key(*b"key-0001");
    let mut persisted = test_session();
    persist_complete(&mut persisted, selected_key, 10);
    let first = BaselineLsmCompactionPlan::lower_from_persisted(&persisted, selected_key).unwrap();

    for index in 1..=64_u64 {
        let bytes = index.to_be_bytes();
        let key = admitted_key(bytes);
        if key != selected_key {
            persist_complete(&mut persisted, key, 100 + index * 3);
        }
    }

    let second = BaselineLsmCompactionPlan::lower_from_persisted(&persisted, selected_key).unwrap();
    assert_eq!(first.key_version, second.key_version);
    assert_eq!(second.membership_observation().partition_probes(), 1);
    assert_eq!(second.membership_observation().component_probes(), 3);
    assert_eq!(
        first.manifest_membership_digest,
        second.manifest_membership_digest
    );
}

#[test]
fn reopen_replays_the_same_key_local_membership_and_cost_shape() {
    let key = admitted_key(*b"reopen01");
    let access = crate::layout_access::WalLayoutAccess::s8();
    let (envelope, anchor) =
        crate::layout_access::baseline_lsm_certification_execution::durable_record_binding(
            &access,
            key,
            300,
            BlobWalRecordKind::LsmValue,
        );
    let mut first = access.open_baseline_lsm_index(&anchor).unwrap();
    access
        .persist_baseline_lsm_record(&mut first, envelope, &anchor, key)
        .unwrap();
    for (sequence, kind) in [
        (301, BlobWalRecordKind::GenerationPublication),
        (302, BlobWalRecordKind::LsmTombstone),
    ] {
        let (envelope, durable) =
            crate::layout_access::baseline_lsm_certification_execution::durable_record_binding(
                &access, key, sequence, kind,
            );
        access
            .persist_baseline_lsm_record(&mut first, envelope, &durable, key)
            .unwrap();
    }
    let before = BaselineLsmCompactionPlan::lower_from_persisted(&first, key).unwrap();
    drop(first);

    let reopened = access.open_baseline_lsm_index(&anchor).unwrap();
    let after = BaselineLsmCompactionPlan::lower_from_persisted(&reopened, key).unwrap();
    assert_eq!(
        before.manifest_membership_digest,
        after.manifest_membership_digest
    );
    assert_eq!(after.membership_observation().partition_probes(), 1);
    assert_eq!(after.membership_observation().component_probes(), 3);
}

#[test]
fn reopen_rejects_post_admission_artifact_mutation() {
    let (access, _key, anchor, session) = production_anchor(*b"mutate01", 400);
    drop(session);
    std::fs::write(anchor.persisted_path(), b"tampered-after-admission").unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch
    );
}

#[test]
fn reopen_rejects_missing_authoritative_artifact() {
    let (access, _key, anchor, session) = production_anchor(*b"delete01", 410);
    drop(session);
    std::fs::remove_file(anchor.persisted_path()).unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch
    );
}

#[test]
fn reopen_rejects_torn_membership_journal_tail() {
    let (access, _key, anchor, session) = production_anchor(*b"torn-001", 420);
    drop(session);
    let journal = anchor
        .persisted_path()
        .parent()
        .unwrap()
        .join("baseline-lsm-membership.journal");
    use std::io::Write as _;
    writeln!(
        std::fs::OpenOptions::new()
            .append(true)
            .open(journal)
            .unwrap(),
        "A torn"
    )
    .unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::PersistedIndexIo
    );
}

#[test]
fn reopen_rejects_valid_shape_semantic_artifact_forgery() {
    let (access, _key, anchor, session) = production_anchor(*b"forge001", 430);
    drop(session);
    let mut artifact = std::fs::read(anchor.persisted_path()).unwrap();
    let tenant_offset = b"forge-store:baseline-lsm-record:v2 ".len();
    artifact[tenant_offset] = if artifact[tenant_offset] == b'1' {
        b'2'
    } else {
        b'1'
    };
    std::fs::write(anchor.persisted_path(), artifact).unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch
    );
}

#[test]
fn production_session_rejects_cross_store_record_redirection() {
    let (access, key, _anchor, mut session) = production_anchor(*b"xstore01", 440);
    let (envelope, redirected) =
        crate::layout_access::baseline_lsm_certification_execution::durable_record_binding_for_store(
            &access,
            key,
            441,
            BlobWalRecordKind::GenerationPublication,
            2,
            1,
        );
    assert_eq!(
        access.persist_baseline_lsm_record(&mut session, envelope, &redirected, key),
        Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch)
    );
}

#[test]
fn reopen_rejects_duplicate_active_membership_rows() {
    let (access, _key, anchor, session) = production_anchor(*b"dupe-001", 450);
    drop(session);
    let journal = membership_journal(&anchor);
    let first = std::fs::read_to_string(&journal)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .to_owned();
    use std::io::Write as _;
    writeln!(
        std::fs::OpenOptions::new()
            .append(true)
            .open(journal)
            .unwrap(),
        "{first}"
    )
    .unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous
    );
}

#[test]
fn reopen_rejects_retirement_bound_to_another_membership_manifest() {
    let (access, first_key, anchor, mut session) = production_anchor(*b"retire01", 460);
    persist_production_tail(&access, &mut session, first_key, 461);
    let other_key = admitted_key(*b"retire02");
    persist_production_complete(&access, &mut session, other_key, 470);
    let other_plan = BaselineLsmCompactionPlan::lower_from_persisted(&session, other_key).unwrap();
    let other_scope = other_plan
        .manifest_scope(crate::StoreCheckpointRecordIdentity::new(8), 470, 474)
        .unwrap();
    let wrong_manifest = access
        .admit_baseline_lsm_manifest_durability(
            &crate::layout_access::baseline_lsm_certification_execution::manifest_receipt(
                other_scope,
            ),
        )
        .unwrap();
    drop(session);

    let forged = format!(
        "R {} {} {} {} {}",
        super::persisted_codec::tenant_code(first_key.tenant_scope()),
        super::persisted_codec::key_scope_code(first_key.key_scope()),
        super::persisted_codec::hex(&first_key.canonical_key_bytes()),
        super::persisted_codec::hex(
            wrong_manifest
                .persisted_path()
                .as_os_str()
                .to_string_lossy()
                .as_bytes()
        ),
        wrong_manifest.persisted_bytes(),
    );
    use std::io::Write as _;
    writeln!(
        std::fs::OpenOptions::new()
            .append(true)
            .open(membership_journal(&anchor))
            .unwrap(),
        "{forged}"
    )
    .unwrap();
    assert_eq!(
        access.open_baseline_lsm_index(&anchor).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch
    );
}

pub(super) fn production_anchor(
    bytes: [u8; 8],
    sequence: u64,
) -> (
    crate::layout_access::WalLayoutAccess,
    BaselineLsmAdmittedKey,
    crate::AdmittedWalAppendReceipt,
    BaselineLsmWalIndexSession,
) {
    let access = crate::layout_access::WalLayoutAccess::s8();
    let key = admitted_key(bytes);
    let (envelope, anchor) =
        crate::layout_access::baseline_lsm_certification_execution::durable_record_binding(
            &access,
            key,
            sequence,
            BlobWalRecordKind::LsmValue,
        );
    let mut session = access.open_baseline_lsm_index(&anchor).unwrap();
    access
        .persist_baseline_lsm_record(&mut session, envelope, &anchor, key)
        .unwrap();
    (access, key, anchor, session)
}

pub(super) fn persist_production_tail(
    access: &crate::layout_access::WalLayoutAccess,
    session: &mut BaselineLsmWalIndexSession,
    key: BaselineLsmAdmittedKey,
    sequence: u64,
) {
    for (offset, kind) in [
        BlobWalRecordKind::GenerationPublication,
        BlobWalRecordKind::LsmTombstone,
    ]
    .into_iter()
    .enumerate()
    {
        let (envelope, durable) =
            crate::layout_access::baseline_lsm_certification_execution::durable_record_binding(
                access,
                key,
                sequence + offset as u64,
                kind,
            );
        access
            .persist_baseline_lsm_record(session, envelope, &durable, key)
            .unwrap();
    }
}

pub(super) fn persist_production_complete(
    access: &crate::layout_access::WalLayoutAccess,
    session: &mut BaselineLsmWalIndexSession,
    key: BaselineLsmAdmittedKey,
    sequence: u64,
) {
    let (envelope, durable) =
        crate::layout_access::baseline_lsm_certification_execution::durable_record_binding(
            access,
            key,
            sequence,
            BlobWalRecordKind::LsmValue,
        );
    access
        .persist_baseline_lsm_record(session, envelope, &durable, key)
        .unwrap();
    persist_production_tail(access, session, key, sequence + 1);
}

pub(super) fn membership_journal(anchor: &crate::AdmittedWalAppendReceipt) -> std::path::PathBuf {
    anchor
        .persisted_path()
        .parent()
        .unwrap()
        .join("baseline-lsm-membership.journal")
}

fn persist_complete(
    persisted: &mut BaselineLsmWalIndexSession,
    key: BaselineLsmAdmittedKey,
    sequence: u64,
) {
    for (offset, kind) in [
        BlobWalRecordKind::LsmValue,
        BlobWalRecordKind::GenerationPublication,
        BlobWalRecordKind::LsmTombstone,
    ]
    .into_iter()
    .enumerate()
    {
        let record = record(persisted, key, sequence + offset as u64, kind);
        persisted.persist(record).unwrap();
    }
}

fn record(
    persisted: &BaselineLsmWalIndexSession,
    key: BaselineLsmAdmittedKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> BaselineLsmAdmittedRecord {
    let durable_scope = WalFrameDurablePublicationScope::new(
        1,
        1,
        sequence,
        sequence + 1,
        format!("wal:{sequence}"),
        128,
    )
    .unwrap();
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind).unwrap(),
        DurablePublicationDeclaration::wal_frame(durable_scope.clone()),
        format!("payload:{sequence}"),
    )
    .unwrap();
    BaselineLsmAdmittedRecord {
        envelope,
        durable_scope,
        key,
        persisted_path: persisted
            .artifact_root
            .join(format!("synthetic-{sequence}")),
        persisted_bytes: 128,
    }
}

pub(super) fn admitted_key(bytes: [u8; 8]) -> BaselineLsmAdmittedKey {
    let security = admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test();
    let metadata = crate::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    BaselineLsmAdmittedKey::admit(metadata, bytes).unwrap()
}

fn test_session() -> BaselineLsmWalIndexSession {
    let root = test_root();
    BaselineLsmWalIndexSession::open(&root, &root, 1, 1).unwrap()
}

fn test_root() -> std::path::PathBuf {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "forge-store-lsm-index-test-{}-{id}",
        std::process::id()
    ))
}
