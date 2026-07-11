use forge_store_security::{StoreKeyScope, StoreTenantScope};

use forge_store_wal::{
    AdmittedCheckpointPublicationReceipt, AdmittedWalAppendReceipt, BlobWalRecordEnvelope,
    BlobWalRecordIdentity, BlobWalRecordKind, CheckpointDurablePublicationScope,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};

#[path = "execution_binding.rs"]
mod execution_binding;
pub(crate) use execution_binding::BaselineLsmExecutionRequest;

#[path = "artifact_binding.rs"]
mod artifact_binding;
use artifact_binding::persisted_artifact_matches;
pub use artifact_binding::{
    baseline_lsm_manifest_artifact_bytes, baseline_lsm_manifest_membership_digest,
    baseline_lsm_output_artifact_bytes, baseline_lsm_record_artifact_bytes,
};

#[path = "persisted_codec.rs"]
mod persisted_codec;

#[path = "store_binding.rs"]
mod store_binding;

#[path = "persisted_index.rs"]
mod persisted_index;
pub use persisted_index::BaselineLsmWalIndexSession;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmPhysicalPublicationBinding {
    root_scope: u64,
    target_epoch: u64,
    manifest_epoch: u64,
}

impl BaselineLsmPhysicalPublicationBinding {
    pub const fn new(root_scope: u64, target_epoch: u64, manifest_epoch: u64) -> Option<Self> {
        if root_scope == 0 || target_epoch == 0 || manifest_epoch == 0 {
            return None;
        }
        Some(Self {
            root_scope,
            target_epoch,
            manifest_epoch,
        })
    }

    pub const fn root_scope(self) -> u64 {
        self.root_scope
    }
    pub const fn target_epoch(self) -> u64 {
        self.target_epoch
    }
    pub const fn manifest_epoch(self) -> u64 {
        self.manifest_epoch
    }
}

/// Caller intent contains no WAL records, manifests, output identity, or counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaselineLsmExecutionIntent {
    physical_publication: BaselineLsmPhysicalPublicationBinding,
}

impl BaselineLsmExecutionIntent {
    pub const fn new(physical_publication: BaselineLsmPhysicalPublicationBinding) -> Self {
        Self {
            physical_publication,
        }
    }
}

/// Security-scoped canonical key authority admitted once by the WAL facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaselineLsmAdmittedKey {
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    canonical_key_bytes: [u8; 8],
}

impl BaselineLsmAdmittedKey {
    pub(crate) fn admit(
        metadata: forge_store_wal::WalSecurityMetadataCarrier,
        canonical_key_bytes: [u8; 8],
    ) -> Option<Self> {
        if canonical_key_bytes == [0; 8] {
            return None;
        }
        Some(Self {
            tenant_scope: metadata.tenant_scope(),
            key_scope: metadata.key_scope(),
            canonical_key_bytes,
        })
    }

    pub(crate) const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }
    pub(crate) const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }
    pub(crate) const fn canonical_key_bytes(self) -> [u8; 8] {
        self.canonical_key_bytes
    }
}

/// WAL-owned admission of a logical LSM record against a durable append receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmAdmittedRecord {
    envelope: BlobWalRecordEnvelope,
    durable_scope: WalFrameDurablePublicationScope,
    key: BaselineLsmAdmittedKey,
    persisted_path: std::path::PathBuf,
    persisted_bytes: u64,
}

impl BaselineLsmAdmittedRecord {
    pub(crate) fn admit(
        envelope: BlobWalRecordEnvelope,
        durable: &AdmittedWalAppendReceipt,
        key: BaselineLsmAdmittedKey,
    ) -> Option<Self> {
        let forge_store_wal::DurablePublicationScope::WalFrame(envelope_scope) =
            envelope.durable_publication().scope()
        else {
            return None;
        };
        if envelope_scope != durable.scope() {
            return None;
        }
        let artifact = baseline_lsm_record_artifact_bytes(&envelope, key);
        if !persisted_artifact_matches(
            durable.persisted_path(),
            durable.persisted_bytes(),
            &artifact,
        ) {
            return None;
        }
        Some(Self {
            envelope,
            durable_scope: durable.scope().clone(),
            key,
            persisted_path: durable.persisted_path().to_path_buf(),
            persisted_bytes: durable.persisted_bytes(),
        })
    }

    pub(super) fn readmit_persisted(
        envelope: BlobWalRecordEnvelope,
        durable_scope: WalFrameDurablePublicationScope,
        key: BaselineLsmAdmittedKey,
        persisted_path: std::path::PathBuf,
        persisted_bytes: u64,
        wal_root: &std::path::Path,
    ) -> Option<Self> {
        let forge_store_wal::DurablePublicationScope::WalFrame(envelope_scope) =
            envelope.durable_publication().scope()
        else {
            return None;
        };
        if !artifact_belongs_to_wal_root(&persisted_path, wal_root)
            || envelope_scope != &durable_scope
            || !persisted_artifact_matches(
                &persisted_path,
                persisted_bytes,
                &baseline_lsm_record_artifact_bytes(&envelope, key),
            )
        {
            return None;
        }
        Some(Self {
            envelope,
            durable_scope,
            key,
            persisted_path,
            persisted_bytes,
        })
    }

    pub(super) fn belongs_to_wal_root(&self, wal_root: &std::path::Path) -> bool {
        artifact_belongs_to_wal_root(&self.persisted_path, wal_root)
    }

    pub(crate) const fn tenant_scope(&self) -> StoreTenantScope {
        self.key.tenant_scope()
    }
    pub(crate) const fn key_scope(&self) -> StoreKeyScope {
        self.key.key_scope()
    }
    pub(crate) const fn canonical_key_bytes(&self) -> [u8; 8] {
        self.key.canonical_key_bytes()
    }
}

pub(super) fn artifact_belongs_to_wal_root(
    path: &std::path::Path,
    wal_root: &std::path::Path,
) -> bool {
    let Some(parent) = path.parent() else {
        return false;
    };
    match (
        std::fs::canonicalize(parent),
        std::fs::canonicalize(wal_root),
    ) {
        (Ok(parent), Ok(root)) => parent.starts_with(root),
        _ => false,
    }
}

/// Complete durable input set. Its fields are private, so omission is impossible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmDurableInputs {
    value: BaselineLsmAdmittedRecord,
    generation: BaselineLsmAdmittedRecord,
    tombstone: BaselineLsmAdmittedRecord,
    manifest: AdmittedCheckpointPublicationReceipt,
    output_append: AdmittedWalAppendReceipt,
    key_version: u64,
}

pub(super) const fn component_slot(kind: BlobWalRecordKind) -> Option<usize> {
    match kind {
        BlobWalRecordKind::LsmValue => Some(0),
        BlobWalRecordKind::GenerationPublication => Some(1),
        BlobWalRecordKind::LsmTombstone => Some(2),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmCompactionPlan {
    value: BaselineLsmAdmittedRecord,
    generation: BaselineLsmAdmittedRecord,
    tombstone: BaselineLsmAdmittedRecord,
    manifest_membership_digest: String,
    key_version: u64,
    membership_observation: BaselineLsmMembershipObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaselineLsmMembershipObservation {
    partition_probes: u16,
    component_probes: u16,
}

impl BaselineLsmMembershipObservation {
    fn record_partition_probe(&mut self) {
        self.partition_probes = self.partition_probes.saturating_add(1);
    }

    fn record_component_probe(&mut self) {
        self.component_probes = self.component_probes.saturating_add(1);
    }

    pub const fn partition_probes(self) -> u16 {
        self.partition_probes
    }

    pub const fn component_probes(self) -> u16 {
        self.component_probes
    }
}

impl BaselineLsmCompactionPlan {
    pub(crate) fn lower_from_persisted(
        session: &BaselineLsmWalIndexSession,
        key: BaselineLsmAdmittedKey,
    ) -> Result<Self, super::BaselineLsmExecutionAdmissionDenial> {
        let mut membership_observation = BaselineLsmMembershipObservation::default();
        membership_observation.record_partition_probe();
        let state = session
            .index
            .keys
            .get(&key)
            .ok_or(super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete)?;
        let mut selected = [None, None, None];
        for (slot, entry) in state.records.iter().enumerate() {
            membership_observation.record_component_probe();
            selected[slot] = entry
                .as_ref()
                .filter(|record| !record.retired)
                .map(|record| record.record.clone());
        }
        let [Some(value), Some(generation), Some(tombstone)] = selected else {
            return Err(super::BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete);
        };
        let records = [&value, &generation, &tombstone];
        let manifest_membership_digest = baseline_lsm_manifest_membership_digest(
            key,
            records.map(|record| record.envelope.identity()),
            &session.store_binding,
        );
        Ok(Self {
            value,
            generation,
            tombstone,
            manifest_membership_digest,
            key_version: state.version,
            membership_observation,
        })
    }

    pub const fn membership_observation(&self) -> BaselineLsmMembershipObservation {
        self.membership_observation
    }

    pub fn manifest_scope(
        &self,
        checkpoint: StoreCheckpointRecordIdentity,
        covered_lsn_start: u64,
        covered_lsn_end: u64,
    ) -> Option<CheckpointDurablePublicationScope> {
        CheckpointDurablePublicationScope::new(
            checkpoint,
            self.manifest_membership_digest.clone(),
            covered_lsn_start,
            covered_lsn_end,
        )
    }

    pub fn output_frame_digest(
        &self,
        physical: BaselineLsmPhysicalPublicationBinding,
    ) -> String {
        format!(
            "lsm-output-v1:{}:{}:{}:{:02x?}:{}",
            physical.root_scope(),
            physical.target_epoch(),
            physical.manifest_epoch(),
            self.value.key.canonical_key_bytes(),
            self.manifest_membership_digest,
        )
    }
}

impl BaselineLsmDurableInputs {
    pub(crate) fn admit(
        plan: BaselineLsmCompactionPlan,
        manifest: AdmittedCheckpointPublicationReceipt,
        output_append: AdmittedWalAppendReceipt,
    ) -> Result<Self, super::BaselineLsmExecutionAdmissionDenial> {
        if manifest.scope().manifest_digest() != plan.manifest_membership_digest
            || !persisted_artifact_matches(
                manifest.persisted_path(),
                manifest.persisted_bytes(),
                &baseline_lsm_manifest_artifact_bytes(manifest.scope()),
            )
        {
            return Err(super::BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch);
        }
        let expected_output = baseline_lsm_output_artifact_bytes(output_append.scope());
        if !persisted_artifact_matches(
            output_append.persisted_path(),
            output_append.persisted_bytes(),
            &expected_output,
        ) {
            return Err(super::BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch);
        }
        Ok(Self {
            value: plan.value,
            generation: plan.generation,
            tombstone: plan.tombstone,
            manifest,
            output_append,
            key_version: plan.key_version,
        })
    }

    pub(crate) const fn key(&self) -> BaselineLsmAdmittedKey {
        self.value.key
    }
}
