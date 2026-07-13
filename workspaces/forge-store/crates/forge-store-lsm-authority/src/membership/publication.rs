use super::activation_artifact::{encode_activation, PersistedMembershipActivation};
use super::{
    artifact::{lsm_membership_output_bytes, persisted_artifact_matches},
    LsmCompactionMembership, LsmMembershipDenial, LsmMembershipKey,
};
use crate::{
    AdmittedCheckpointPublicationReceipt, AdmittedWalAppendReceipt, BlobWalRecordEnvelope,
    BlobWalRecordIdentity, CheckpointDurablePublicationScope, DurablePublicationDeclaration,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_PUBLISHED_MEMBERSHIP_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublishedLsmMembershipIdentity(u64);

impl PublishedLsmMembershipIdentity {
    fn issue() -> Self {
        let value = NEXT_PUBLISHED_MEMBERSHIP_IDENTITY
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .expect("published LSM membership identity space exhausted");
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedLsmReplacementOutput {
    envelope: BlobWalRecordEnvelope,
    scope: WalFrameDurablePublicationScope,
    persisted_path: std::path::PathBuf,
    persisted_bytes: u64,
    key: LsmMembershipKey,
    selected_identities: [BlobWalRecordIdentity; 3],
    membership_version: u64,
    store_binding: String,
    physical: crate::LsmPhysicalCompactionIntent,
}

pub fn admit_lsm_replacement_output(
    selected: &LsmCompactionMembership,
    durable: AdmittedWalAppendReceipt,
    physical: crate::LsmPhysicalCompactionIntent,
) -> Result<AdmittedLsmReplacementOutput, LsmMembershipDenial> {
    let identity = selected
        .expected_output_identity()
        .ok_or(LsmMembershipDenial::ReplacementOutputMismatch)?;
    let scope = durable.scope();
    let expected_digest = selected.compaction_output_digest(
        physical.root_scope(),
        physical.target_epoch(),
        physical.manifest_epoch(),
    );
    let store_scope_matches = selected.records().iter().all(|record| {
        record.durable_scope().segment_id() == scope.segment_id()
            && record.durable_scope().generation() == scope.generation()
    });
    if !store_scope_matches
        || scope.lsn_start() > identity.sequence()
        || scope.lsn_end() < identity.sequence()
        || scope.expected_bytes() != durable.persisted_bytes()
        || scope.frame_digest() != expected_digest
        || !persisted_artifact_matches(
            durable.persisted_path(),
            durable.persisted_bytes(),
            &lsm_membership_output_bytes(scope),
        )
    {
        return Err(LsmMembershipDenial::ReplacementOutputMismatch);
    }
    let envelope = BlobWalRecordEnvelope::new(
        identity,
        DurablePublicationDeclaration::wal_frame(scope.clone()),
        scope.frame_digest().to_owned(),
    )
    .map_err(|_| LsmMembershipDenial::ReplacementOutputMismatch)?;
    Ok(AdmittedLsmReplacementOutput {
        envelope,
        scope: scope.clone(),
        persisted_path: durable.persisted_path().to_path_buf(),
        persisted_bytes: durable.persisted_bytes(),
        key: selected.key(),
        selected_identities: selected.identities(),
        membership_version: selected.version(),
        store_binding: selected.store_binding().to_owned(),
        physical,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedLsmMembershipReplacement {
    checkpoint: AdmittedCheckpointPublicationReceipt,
    output: AdmittedLsmReplacementOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmMembershipActivationDeclaration {
    selected_key: LsmMembershipKey,
    selected_identities: [BlobWalRecordIdentity; 3],
    selected_base: Option<BlobWalRecordIdentity>,
    selected_version: u64,
    store_binding: String,
    output: AdmittedLsmReplacementOutput,
    scope: CheckpointDurablePublicationScope,
    bytes: Vec<u8>,
}

pub fn admit_lsm_membership_replacement(
    selected: &LsmCompactionMembership,
    activation: LsmMembershipActivationDeclaration,
    checkpoint: AdmittedCheckpointPublicationReceipt,
) -> Result<AdmittedLsmMembershipReplacement, LsmMembershipDenial> {
    if !activation.binds(selected)
        || checkpoint.scope() != activation.scope()
        || !persisted_artifact_matches(
            checkpoint.persisted_path(),
            checkpoint.persisted_bytes(),
            activation.bytes(),
        )
    {
        return Err(LsmMembershipDenial::ManifestMembershipMismatch);
    }
    Ok(AdmittedLsmMembershipReplacement {
        checkpoint,
        output: activation.output,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn prepare_lsm_membership_activation(
    selected: &LsmCompactionMembership,
    output: AdmittedLsmReplacementOutput,
    physical: &forge_store_physical_isolation::CompactionRewritePublication,
) -> Result<LsmMembershipActivationDeclaration, LsmMembershipDenial> {
    if !output.binds(selected) || !output.binds_physical(physical) {
        return Err(LsmMembershipDenial::PhysicalPublicationBindingMismatch);
    }
    let identities = selected.identities();
    let checkpoint_epoch = selected
        .base()
        .map(|base| base.activation_scope.checkpoint().checkpoint_epoch())
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(LsmMembershipDenial::ManifestMembershipMismatch)?;
    let covered_lsn_start = selected.base().map_or(identities[0].sequence(), |base| {
        base.activation_scope.covered_lsn_start()
    });
    let covered_lsn_end = output
        .identity()
        .sequence()
        .checked_add(1)
        .ok_or(LsmMembershipDenial::ManifestMembershipMismatch)?;
    let publication = physical.publication();
    let digest = format!(
        "{}{}:{}:{}:{}:{}",
        super::artifact::lsm_membership_activation_digest_prefix(
            selected.key(),
            identities,
            selected.base().map(|base| base.output()),
            output.identity(),
            selected.store_binding(),
            output.scope(),
        ),
        publication.old_root().scope(),
        publication.old_root().epoch().get(),
        publication.new_root().epoch().get(),
        publication.old_root().manifest_epoch().get(),
        publication.new_root().manifest_epoch().get(),
    );
    let scope = CheckpointDurablePublicationScope::new(
        StoreCheckpointRecordIdentity::new(checkpoint_epoch),
        digest,
        covered_lsn_start,
        covered_lsn_end,
    )
    .ok_or(LsmMembershipDenial::ManifestMembershipMismatch)?;
    let activation =
        PersistedMembershipActivation::from_publication(selected, &output, scope.clone());
    let bytes = encode_activation(&activation)?;
    Ok(LsmMembershipActivationDeclaration {
        selected_key: selected.key(),
        selected_identities: identities,
        selected_base: selected.base().map(|base| base.output()),
        selected_version: selected.version(),
        store_binding: selected.store_binding().to_owned(),
        output,
        scope,
        bytes,
    })
}

impl LsmMembershipActivationDeclaration {
    pub const fn scope(&self) -> &CheckpointDurablePublicationScope {
        &self.scope
    }

    pub fn artifact(&self) -> super::LsmMembershipArtifactDeclaration {
        super::LsmMembershipArtifactDeclaration::from_owner_bytes(self.bytes.clone())
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn binds(&self, selected: &LsmCompactionMembership) -> bool {
        self.selected_key == selected.key()
            && self.selected_identities == selected.identities()
            && self.selected_base == selected.base().map(|base| base.output())
            && self.selected_version == selected.version()
            && self.store_binding == selected.store_binding()
            && self.output.binds(selected)
    }
}

impl AdmittedLsmReplacementOutput {
    pub const fn envelope(&self) -> &BlobWalRecordEnvelope {
        &self.envelope
    }

    pub const fn identity(&self) -> BlobWalRecordIdentity {
        self.envelope.identity()
    }

    pub const fn scope(&self) -> &WalFrameDurablePublicationScope {
        &self.scope
    }

    pub fn persisted_path(&self) -> &std::path::Path {
        &self.persisted_path
    }

    pub const fn persisted_bytes(&self) -> u64 {
        self.persisted_bytes
    }

    pub(crate) fn binds(&self, selected: &LsmCompactionMembership) -> bool {
        self.key == selected.key()
            && self.selected_identities == selected.identities()
            && self.membership_version == selected.version()
            && self.store_binding == selected.store_binding()
    }

    pub(crate) fn binds_physical(
        &self,
        publication: &forge_store_physical_isolation::CompactionRewritePublication,
    ) -> bool {
        self.physical.binds(publication)
    }
}

impl AdmittedLsmMembershipReplacement {
    pub(crate) fn binds(&self, selected: &LsmCompactionMembership) -> bool {
        self.output.binds(selected)
    }

    pub const fn output(&self) -> &AdmittedLsmReplacementOutput {
        &self.output
    }

    pub fn scope(&self) -> &CheckpointDurablePublicationScope {
        self.checkpoint.scope()
    }

    pub fn persisted_path(&self) -> &std::path::Path {
        self.checkpoint.persisted_path()
    }

    pub const fn persisted_bytes(&self) -> u64 {
        self.checkpoint.persisted_bytes()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedLsmMembershipReplacement {
    identity: PublishedLsmMembershipIdentity,
    key: LsmMembershipKey,
    retired: [BlobWalRecordIdentity; 3],
    output: BlobWalRecordIdentity,
    output_scope: WalFrameDurablePublicationScope,
    activation_scope: CheckpointDurablePublicationScope,
    output_path: std::path::PathBuf,
    output_bytes: u64,
}

impl PublishedLsmMembershipReplacement {
    pub(crate) fn issued(
        key: LsmMembershipKey,
        retired: [BlobWalRecordIdentity; 3],
        output: BlobWalRecordIdentity,
        output_scope: WalFrameDurablePublicationScope,
        activation_scope: CheckpointDurablePublicationScope,
        output_path: std::path::PathBuf,
        output_bytes: u64,
    ) -> Self {
        Self {
            identity: PublishedLsmMembershipIdentity::issue(),
            key,
            retired,
            output,
            output_scope,
            activation_scope,
            output_path,
            output_bytes,
        }
    }

    pub const fn identity(&self) -> PublishedLsmMembershipIdentity {
        self.identity
    }

    pub const fn key(&self) -> LsmMembershipKey {
        self.key
    }

    pub const fn retired(&self) -> &[BlobWalRecordIdentity; 3] {
        &self.retired
    }

    pub const fn output(&self) -> BlobWalRecordIdentity {
        self.output
    }

    pub const fn persisted_bytes(&self) -> u64 {
        self.output_bytes
    }

    pub const fn activation_scope(&self) -> &CheckpointDurablePublicationScope {
        &self.activation_scope
    }

    pub(crate) fn artifact_is_current(&self) -> bool {
        persisted_artifact_matches(
            &self.output_path,
            self.output_bytes,
            &lsm_membership_output_bytes(&self.output_scope),
        )
    }
}
