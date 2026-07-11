use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority, StoreDurabilityAdmission,
    StoreDurabilityBoundaryReached, StoreDurabilityDenial, StoreDurabilityFileSyncKind,
    StoreDurabilityOrderingBarrierDurable, StoreDurabilityRequirement, StoreDurabilityRuntime,
    StoreDurabilityWriteAccepted, WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use forge_store_security::{
    admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test, StoreKeyVersionPosture,
    StoreLegacySecurityPosture,
};

use super::baseline_lsm_counter_observation::{
    BaselineLsmExecutionIntent, BaselineLsmExecutionWitness, BaselineLsmPhysicalPublicationBinding,
};
use super::WalLayoutAccess;
use crate::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind,
    CheckpointDurablePublicationScope, DurablePublicationDeclaration,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};

/// Certification drives the same durability and WAL facades as production.
/// It supplies observations to the backend boundary but cannot construct WAL receipts.
pub fn execute_baseline_lsm_persisted_fixture(
    physical: BaselineLsmPhysicalPublicationBinding,
) -> BaselineLsmExecutionWitness {
    let access = WalLayoutAccess::s8();
    let security = admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test();
    let metadata = crate::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let key = access
        .admit_baseline_lsm_key(metadata, *b"lsm-key1")
        .expect("security-scoped canonical key");
    let (first_envelope, first_durable) =
        durable_record_binding(&access, key, 41, BlobWalRecordKind::LsmValue);
    let mut persisted = access
        .open_baseline_lsm_index(&first_durable)
        .expect("WAL-owned persistent membership index");
    let first_record = access
        .persist_baseline_lsm_record(&mut persisted, first_envelope, &first_durable, key)
        .expect("first record binds the index to its WAL store");
    let _persisted_records = [
        first_record,
        durable_record(
            &access,
            &mut persisted,
            key,
            42,
            BlobWalRecordKind::GenerationPublication,
        ),
        durable_record(
            &access,
            &mut persisted,
            key,
            43,
            BlobWalRecordKind::LsmTombstone,
        ),
    ];
    drop(persisted);
    let mut persisted = access
        .open_baseline_lsm_index(&first_durable)
        .expect("reopen re-admits membership from persisted WAL artifacts");
    let plan = access
        .lower_baseline_lsm_compaction(&persisted, key)
        .expect("persisted WAL membership lowers to one compaction plan");
    let output_digest = plan.output_frame_digest(physical);
    let output_scope = wal_scope(44, output_digest, 4096);
    let output_artifact =
        super::baseline_lsm_counter_observation::baseline_lsm_output_artifact_bytes(&output_scope);
    let output = access
        .admit_baseline_lsm_append_durability(&wal_receipt(output_scope, &output_artifact))
        .expect("executed output durability");
    let manifest_scope = plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(1), 41, 45)
        .expect("manifest coverage");
    let manifest = access
        .admit_baseline_lsm_manifest_durability(&manifest_receipt(manifest_scope))
        .expect("executed manifest durability");
    let inputs = access
        .admit_baseline_lsm_persisted_source(plan.clone(), manifest.clone(), output.clone())
        .expect("manifest admits the complete durable input set");
    let stale_inputs = access
        .admit_baseline_lsm_persisted_source(plan, manifest, output)
        .expect("the same current persisted membership can be planned concurrently");
    let witness = access
        .execute_baseline_lsm(
            &mut persisted,
            BaselineLsmExecutionIntent::new(physical),
            inputs,
        )
        .expect("durable LSM inputs execute");
    assert_eq!(
        access.execute_baseline_lsm(
            &mut persisted,
            BaselineLsmExecutionIntent::new(physical),
            stale_inputs,
        ),
        Err(super::baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial::PersistedMembershipStale),
        "retirement must stale every concurrent plan derived from the prior key-local version",
    );
    witness
}

fn durable_record(
    access: &WalLayoutAccess,
    persisted: &mut super::baseline_lsm_counter_observation::BaselineLsmWalIndexSession,
    key: super::baseline_lsm_counter_observation::BaselineLsmAdmittedKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> super::baseline_lsm_counter_observation::BaselineLsmAdmittedRecord {
    let (envelope, durable) = durable_record_binding(access, key, sequence, kind);
    access
        .persist_baseline_lsm_record(persisted, envelope, &durable, key)
        .expect("record binds to durable append")
}

pub(crate) fn durable_record_binding(
    access: &WalLayoutAccess,
    key: super::baseline_lsm_counter_observation::BaselineLsmAdmittedKey,
    sequence: u64,
    kind: BlobWalRecordKind,
) -> (BlobWalRecordEnvelope, crate::AdmittedWalAppendReceipt) {
    durable_record_binding_for_store(access, key, sequence, kind, 1, 1)
}

pub(crate) fn durable_record_binding_for_store(
    access: &WalLayoutAccess,
    key: super::baseline_lsm_counter_observation::BaselineLsmAdmittedKey,
    sequence: u64,
    kind: BlobWalRecordKind,
    segment_id: u64,
    generation: u64,
) -> (BlobWalRecordEnvelope, crate::AdmittedWalAppendReceipt) {
    let payload_digest = format!("lsm-input:{sequence}:{kind:?}");
    let scope = WalFrameDurablePublicationScope::new(
        segment_id,
        generation,
        sequence,
        sequence.checked_add(1).expect("bounded fixture sequence"),
        payload_digest.clone(),
        4096,
    )
    .expect("WAL frame scope");
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind).expect("record identity"),
        DurablePublicationDeclaration::wal_frame(scope.clone()),
        payload_digest.clone(),
    )
    .expect("WAL envelope");
    let artifact =
        super::baseline_lsm_counter_observation::baseline_lsm_record_artifact_bytes(&envelope, key);
    let durable = access
        .admit_baseline_lsm_append_durability(&wal_receipt(scope, &artifact))
        .expect("executed WAL durability");
    (envelope, durable)
}

fn wal_scope(
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
    let artifact =
        super::baseline_lsm_counter_observation::baseline_lsm_manifest_artifact_bytes(&scope);
    let requirement = StoreDurabilityRequirement::manifest_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    reach_boundary(
        admitted(requirement).submit_write(scope).backend_accepted(),
        StoreDurabilityFileSyncKind::Fsync,
        true,
        true,
        &artifact,
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
    let proof = execution_directory(|execution_directory| {
        StoreDurabilityRuntime::new()
            .persist_and_execute(execution_directory, artifact, &accepted)
            .expect("physical durability execution")
    });
    assert!(proof.persisted_path().is_file());
    assert_eq!(proof.persisted_bytes(), artifact.len() as u64);
    accepted.reach_durability_boundary(proof)
}

fn execution_directory<T>(execute: impl FnOnce(&std::path::Path) -> T) -> T {
    static FIXTURE_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    thread_local! {
        static DIRECTORY: std::path::PathBuf = {
            let directory = std::env::temp_dir().join(format!(
                "forge-store-lsm-durability-{}-{}",
                std::process::id(),
                FIXTURE_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ));
            std::fs::create_dir_all(&directory)
                .expect("unique durability execution directory");
            directory
        };
    }
    DIRECTORY.with(|directory| execute(directory.as_path()))
}

#[cfg(test)]
mod artifact_binding_tests {
    use super::*;

    #[test]
    fn durable_scope_cannot_authorize_different_wal_bytes() {
        let access = WalLayoutAccess::s8();
        let security = admitted_tenant_wal_checkpoint_security_scope_for_layout_access_test();
        let metadata = crate::WalSecurityMetadataCarrier::for_wal_record(
            security.witnesses(),
            StoreKeyVersionPosture::Current,
            StoreLegacySecurityPosture::NativeScoped,
        );
        let key = access
            .admit_baseline_lsm_key(metadata, *b"mismatch")
            .unwrap();
        let scope = wal_scope(91, "claimed-frame".into(), 11);
        let receipt = access
            .admit_baseline_lsm_append_durability(&wal_receipt(scope.clone(), b"wrong-bytes"))
            .unwrap();
        let envelope = BlobWalRecordEnvelope::new(
            BlobWalRecordIdentity::new(91, BlobWalRecordKind::LsmValue).unwrap(),
            DurablePublicationDeclaration::wal_frame(scope),
            "claimed-frame",
        )
        .unwrap();
        let mut index = access.open_baseline_lsm_index(&receipt).unwrap();
        assert_eq!(
            access.persist_baseline_lsm_record(&mut index, envelope, &receipt, key),
            Err(super::super::baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)
        );
    }
}
