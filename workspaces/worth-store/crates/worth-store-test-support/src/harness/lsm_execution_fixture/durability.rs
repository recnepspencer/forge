use worth_store_layout_indexes::LsmStrategy;
use worth_store_lsm_authority::{
    LsmMembershipArtifactDeclaration, LsmMembershipKey, LsmMembershipRecord, LsmMembershipSession,
};
use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority, StoreDurabilityAdmission,
    StoreDurabilityBoundaryReached, StoreDurabilityDenial, StoreDurabilityFileSyncKind,
    StoreDurabilityOrderingBarrierDurable, StoreDurabilityRequirement, StoreDurabilityRuntime,
    StoreDurabilityWriteAccepted, WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use worth_store_wal::{
    admit_durable_append, AdmittedWalAppendReceipt, BlobWalRecordEnvelope, BlobWalRecordIdentity,
    BlobWalRecordKind, CheckpointDurablePublicationScope, DurablePublicationDeclaration,
    WalFrameDurablePublicationScope,
};

pub(super) fn durable_record(
    access: &LsmStrategy,
    persisted: &mut LsmMembershipSession,
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> LsmMembershipRecord {
    let (envelope, durable) = durable_record_binding(key, sequence, kind);
    access
        .persist_record(persisted, envelope, &durable, key)
        .expect("record binds to durable append")
}

pub(crate) fn durable_record_binding(
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> (BlobWalRecordEnvelope, AdmittedWalAppendReceipt) {
    durable_record_binding_for_store(key, sequence, kind, 1, 1)
}

pub(crate) fn durable_record_binding_for_store(
    key: LsmMembershipKey,
    sequence: u64,
    kind: BlobWalRecordKind,
    segment_id: u64,
    generation: u64,
) -> (BlobWalRecordEnvelope, AdmittedWalAppendReceipt) {
    durable_record_binding_with_lsn(key, sequence, kind, segment_id, generation, sequence)
}

pub(super) fn durable_record_binding_with_lsn(
    key: LsmMembershipKey,
    identity_sequence: u64,
    kind: BlobWalRecordKind,
    segment_id: u64,
    generation: u64,
    lsn: u64,
) -> (BlobWalRecordEnvelope, AdmittedWalAppendReceipt) {
    let payload_digest = format!("lsm-input:{identity_sequence}:{kind:?}:lsn={lsn}");
    let scope = WalFrameDurablePublicationScope::new(
        segment_id,
        generation,
        lsn,
        lsn.checked_add(1).expect("bounded fixture LSN"),
        payload_digest.clone(),
        4096,
    )
    .expect("WAL frame scope");
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(identity_sequence, kind).expect("record identity"),
        DurablePublicationDeclaration::wal_frame(scope.clone()),
        payload_digest,
    )
    .expect("WAL envelope");
    let artifact = LsmMembershipArtifactDeclaration::record(&envelope, key);
    let durable = admit_durable_append(&wal_receipt(scope, artifact.bytes()))
        .expect("executed WAL durability");
    (envelope, durable)
}

pub(super) fn wal_scope(
    sequence: u64,
    frame_digest: String,
    expected_bytes: u64,
) -> WalFrameDurablePublicationScope {
    WalFrameDurablePublicationScope::new(
        1,
        1,
        sequence,
        sequence.checked_add(1).expect("bounded fixture sequence"),
        frame_digest,
        expected_bytes,
    )
    .expect("WAL frame scope")
}

pub(crate) fn wal_receipt(
    scope: WalFrameDurablePublicationScope,
    artifact: &[u8],
) -> StoreDurabilityOrderingBarrierDurable<WalFrameDurablePublicationScope> {
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync),
    );
    reach_boundary(
        admitted(requirement).submit_write(scope).backend_accepted(),
        StoreDurabilityFileSyncKind::Fdatasync,
        false,
        false,
        artifact,
    )
    .expect("WAL boundary")
    .ordering_barrier_durable()
    .expect("WAL ordering barrier")
}

pub(crate) fn manifest_receipt(
    scope: CheckpointDurablePublicationScope,
) -> StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope> {
    let artifact = LsmMembershipArtifactDeclaration::manifest(&scope);
    manifest_receipt_for_artifact(scope, artifact.bytes())
}

pub(crate) fn manifest_receipt_for_artifact(
    scope: CheckpointDurablePublicationScope,
    artifact: &[u8],
) -> StoreDurabilityOrderingBarrierDurable<CheckpointDurablePublicationScope> {
    let requirement = StoreDurabilityRequirement::manifest_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    reach_boundary(
        admitted(requirement).submit_write(scope).backend_accepted(),
        StoreDurabilityFileSyncKind::Fsync,
        true,
        true,
        artifact,
    )
    .expect("manifest boundary")
    .parent_namespace_durable()
    .expect("manifest namespace")
    .rename_durable()
    .expect("manifest rename")
    .ordering_barrier_durable()
    .expect("manifest ordering barrier")
}

fn admitted(requirement: StoreDurabilityRequirement) -> StoreDurabilityAdmission {
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend capability");
    StoreDurabilityAdmission::admit(requirement, &witness).expect("durability admission")
}

fn reach_boundary<S>(
    accepted: StoreDurabilityWriteAccepted<S>,
    sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    artifact: &[u8],
) -> Result<StoreDurabilityBoundaryReached<S>, StoreDurabilityDenial>
where
    S: Clone + Eq + core::fmt::Debug,
{
    assert_eq!(accepted.requirement().required_file_sync(), sync);
    assert_eq!(
        accepted.requirement().requires_directory_sync(),
        directory_sync_completed
    );
    assert_eq!(
        accepted.requirement().requires_rename_durable(),
        rename_completed
    );
    let proof = execution_directory(|directory| {
        StoreDurabilityRuntime::new()
            .persist_and_execute(directory, artifact, &accepted)
            .expect("physical durability execution")
    });
    assert!(proof.persisted_path().is_file());
    assert_eq!(proof.persisted_bytes(), artifact.len() as u64);
    accepted.reach_durability_boundary(proof)
}

thread_local! {
    static FIXTURE_DIRECTORY: std::cell::RefCell<Option<std::path::PathBuf>> = const {
        std::cell::RefCell::new(None)
    };
}

pub(super) fn begin_durability_fixture() {
    FIXTURE_DIRECTORY.with(|slot| *slot.borrow_mut() = Some(new_execution_directory()));
}

fn execution_directory<T>(execute: impl FnOnce(&std::path::Path) -> T) -> T {
    FIXTURE_DIRECTORY.with(|slot| {
        if slot.borrow().is_none() {
            *slot.borrow_mut() = Some(new_execution_directory());
        }
        execute(slot.borrow().as_ref().expect("fixture directory"))
    })
}

fn new_execution_directory() -> std::path::PathBuf {
    static FIXTURE_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    loop {
        let directory = std::env::temp_dir().join(format!(
            "worth-store-lsm-durability-{}-{}",
            std::process::id(),
            FIXTURE_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        ));
        match std::fs::create_dir(&directory) {
            Ok(()) => return directory,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => panic!("cannot create durability execution directory: {error}"),
        }
    }
}
