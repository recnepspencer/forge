use crate::{
    AdmittedWalAppendReceipt, BlobWalRecordEnvelope, BlobWalRecordIdentity, BlobWalRecordKind,
    CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity,
    WalFrameDurablePublicationScope, WalSecurityMetadataCarrier,
};
use forge_store_security::{StoreKeyScope, StoreTenantScope};

pub const MAX_LSM_CANONICAL_KEY_BYTES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmMembershipKey {
    pub(crate) security_identity: forge_store_security::StoreSecurityScopeIdentity,
    pub(crate) authority_identity: forge_store_authority::StoreCurrentAuthorityIdentity,
    pub(crate) tenant_scope: StoreTenantScope,
    pub(crate) key_scope: StoreKeyScope,
    pub(crate) canonical: [u8; MAX_LSM_CANONICAL_KEY_BYTES],
    pub(crate) len: u8,
}

impl std::hash::Hash for LsmMembershipKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.authority_identity, state);
        std::hash::Hash::hash(&self.canonical(), state);
    }
}

impl LsmMembershipKey {
    pub fn admit(metadata: WalSecurityMetadataCarrier, canonical: &[u8]) -> Option<Self> {
        if canonical.is_empty() || canonical.len() > MAX_LSM_CANONICAL_KEY_BYTES {
            return None;
        }
        let mut bounded = [0; MAX_LSM_CANONICAL_KEY_BYTES];
        bounded[..canonical.len()].copy_from_slice(canonical);
        Some(Self {
            security_identity: metadata.security_identity(),
            authority_identity: metadata.authority_identity(),
            tenant_scope: metadata.tenant_scope(),
            key_scope: metadata.key_scope(),
            canonical: bounded,
            len: canonical.len() as u8,
        })
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub fn canonical(&self) -> &[u8] {
        &self.canonical[..self.len as usize]
    }

    pub const fn security_identity(self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.security_identity
    }

    pub const fn authority_identity(self) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }

    pub(super) fn readmit(
        authority: LsmMembershipReadmissionAuthority,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        canonical: &[u8],
    ) -> Option<Self> {
        if authority.security_identity.tenant_scope() != tenant_scope
            || authority.security_identity.key_scope() != key_scope
        {
            return None;
        }
        if canonical.is_empty() || canonical.len() > MAX_LSM_CANONICAL_KEY_BYTES {
            return None;
        }
        let mut bounded = [0; MAX_LSM_CANONICAL_KEY_BYTES];
        bounded[..canonical.len()].copy_from_slice(canonical);
        Some(Self {
            security_identity: authority.security_identity,
            authority_identity: authority.authority_identity,
            tenant_scope,
            key_scope,
            canonical: bounded,
            len: canonical.len() as u8,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LsmMembershipReadmissionAuthority {
    security_identity: forge_store_security::StoreSecurityScopeIdentity,
    authority_identity: forge_store_authority::StoreCurrentAuthorityIdentity,
}

impl LsmMembershipReadmissionAuthority {
    pub(super) fn from_current_scope(
        witnesses: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    ) -> Self {
        Self {
            security_identity: witnesses.key_scope().identity(),
            authority_identity: witnesses.authority_identity(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmMembershipRecord {
    pub(crate) envelope: BlobWalRecordEnvelope,
    pub(crate) durable_scope: WalFrameDurablePublicationScope,
    pub(crate) key: LsmMembershipKey,
    pub(crate) persisted_path: std::path::PathBuf,
    pub(crate) persisted_bytes: u64,
}

impl LsmMembershipRecord {
    pub fn admit(
        envelope: BlobWalRecordEnvelope,
        durable: &AdmittedWalAppendReceipt,
        key: LsmMembershipKey,
    ) -> Option<Self> {
        let crate::DurablePublicationScope::WalFrame(scope) =
            envelope.durable_publication().scope()
        else {
            return None;
        };
        if scope != durable.scope()
            || !super::durable_artifact::persisted_artifact_matches(
                durable.persisted_path(),
                durable.persisted_bytes(),
                &super::durable_artifact::lsm_membership_record_bytes(&envelope, key),
            )
        {
            return None;
        }
        let durable_scope = scope.clone();
        Some(Self {
            envelope,
            durable_scope,
            key,
            persisted_path: durable.persisted_path().to_path_buf(),
            persisted_bytes: durable.persisted_bytes(),
        })
    }

    pub const fn key(&self) -> LsmMembershipKey {
        self.key
    }

    pub const fn key_ref(&self) -> &LsmMembershipKey {
        &self.key
    }

    pub const fn identity(&self) -> BlobWalRecordIdentity {
        self.envelope.identity()
    }

    pub const fn kind(&self) -> BlobWalRecordKind {
        self.envelope.identity().kind()
    }

    pub const fn envelope(&self) -> &BlobWalRecordEnvelope {
        &self.envelope
    }

    pub const fn durable_scope(&self) -> &WalFrameDurablePublicationScope {
        &self.durable_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmCompactionMembership {
    pub(crate) key: LsmMembershipKey,
    pub(crate) record_set: super::LsmCompactionRecordSet,
    pub(crate) base: Option<super::PublishedLsmMembershipReplacement>,
    pub(crate) version: u64,
    pub(crate) store_binding: String,
    pub(crate) partition_probes: u16,
    pub(crate) component_probes: u16,
}

impl LsmCompactionMembership {
    pub const fn key(&self) -> LsmMembershipKey {
        self.key
    }

    pub const fn key_ref(&self) -> &LsmMembershipKey {
        &self.key
    }

    pub const fn record_set(&self) -> &super::LsmCompactionRecordSet {
        &self.record_set
    }

    pub const fn base(&self) -> Option<&super::PublishedLsmMembershipReplacement> {
        self.base.as_ref()
    }

    pub fn identities(&self) -> [BlobWalRecordIdentity; 3] {
        self.record_set.identities()
    }

    pub fn identity_set(&self) -> super::LsmCompactionRecordIdentitySet {
        self.record_set.identity_set()
    }

    pub fn revalidate_artifacts(&self) -> Result<(), super::LsmMembershipDenial> {
        if self.record_set.iter().any(|record| {
            !super::durable_artifact::persisted_artifact_matches(
                &record.persisted_path,
                record.persisted_bytes,
                &super::durable_artifact::lsm_membership_record_bytes(&record.envelope, record.key),
            )
        }) || self
            .base
            .as_ref()
            .is_some_and(|base| !base.artifact_is_current())
        {
            return Err(super::LsmMembershipDenial::DurableRecordBindingMismatch);
        }
        Ok(())
    }

    pub const fn version(&self) -> u64 {
        self.version
    }

    pub fn store_binding(&self) -> &str {
        &self.store_binding
    }

    pub const fn partition_probes(&self) -> u16 {
        self.partition_probes
    }

    pub const fn component_probes(&self) -> u16 {
        self.component_probes
    }

    pub fn manifest_scope(
        &self,
        checkpoint: StoreCheckpointRecordIdentity,
        covered_lsn_start: u64,
        covered_lsn_end: u64,
    ) -> Option<CheckpointDurablePublicationScope> {
        let output = self.expected_output_identity()?;
        CheckpointDurablePublicationScope::new(
            checkpoint,
            super::durable_artifact::lsm_membership_replacement_digest(
                self.key,
                self.identities(),
                self.base.as_ref().map(|base| base.output()),
                output,
                &self.store_binding,
            ),
            covered_lsn_start,
            covered_lsn_end,
        )
    }

    pub fn replacement_manifest_digest(&self) -> String {
        super::durable_artifact::lsm_membership_replacement_digest(
            self.key,
            self.identities(),
            self.base.as_ref().map(|base| base.output()),
            self.expected_output_identity()
                .expect("admitted membership sequences leave output generation space"),
            &self.store_binding,
        )
    }

    pub fn expected_output_identity(&self) -> Option<BlobWalRecordIdentity> {
        BlobWalRecordIdentity::new(
            self.record_set
                .tombstone()
                .identity()
                .sequence()
                .checked_add(1)?,
            BlobWalRecordKind::GenerationPublication,
        )
    }

    pub fn compaction_output_digest(
        &self,
        root_scope: u64,
        target_epoch: u64,
        manifest_epoch: u64,
    ) -> String {
        format!(
            "lsm-output-v1:{root_scope}:{target_epoch}:{manifest_epoch}:{:02x?}:{}",
            self.key.canonical(),
            super::durable_artifact::lsm_membership_digest(
                self.key,
                self.identities(),
                self.base.as_ref().map(|base| base.output()),
                &self.store_binding,
            ),
        )
    }
}
