use super::model::{
    LsmCompactionMembership, LsmMembershipKey, LsmMembershipReadmissionAuthority,
    LsmMembershipRecord,
};
use crate::{
    AdmittedLsmMembershipReplacement, AdmittedWalAppendReceipt, AdmittedWalArtifactStore,
    BlobWalRecordIdentity, PublishedLsmMembershipReplacement, WalFrameDurablePublicationScope,
};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmMembershipDenial {
    CanonicalKeyRequired,
    DurableRecordBindingMismatch,
    StoreBindingMismatch,
    UnsupportedRecordKind,
    MembershipAmbiguous,
    MembershipIncomplete,
    MembershipStale,
    ManifestMembershipMismatch,
    ReplacementOutputMismatch,
    PhysicalPublicationBindingMismatch,
    PersistedMembershipArtifactInvalid,
    Io,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmMembershipReplayPosture {
    DurableArtifactsReadmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LsmMembershipReopenCounters {
    pub(super) artifacts_examined: u64,
    pub(super) artifacts_readmitted: u64,
    pub(super) bytes_examined: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecordState {
    pub(super) record: LsmMembershipRecord,
    pub(super) retired: bool,
}

#[derive(Debug, Default)]
pub(super) struct KeyState {
    pub(super) records: [Option<RecordState>; 3],
    pub(super) version: u64,
    pub(super) published_replacement: Option<PublishedLsmMembershipReplacement>,
}

#[derive(Debug)]
pub struct LsmMembershipSession {
    pub(super) keys: HashMap<LsmMembershipKey, KeyState>,
    pub(super) store: AdmittedWalArtifactStore,
    pub(super) store_binding: String,
    pub(super) readmission_authority: LsmMembershipReadmissionAuthority,
    pub(super) segment_id: u64,
    pub(super) generation: u64,
    pub(super) replay_posture: LsmMembershipReplayPosture,
    pub(super) reopen_counters: LsmMembershipReopenCounters,
}

impl LsmMembershipSession {
    pub fn open(
        anchor: &AdmittedWalAppendReceipt,
        current_scope: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    ) -> Result<Self, LsmMembershipDenial> {
        let store = AdmittedWalArtifactStore::open(anchor)
            .map_err(|_| LsmMembershipDenial::StoreBindingMismatch)?;
        let readmission_authority =
            LsmMembershipReadmissionAuthority::from_current_scope(current_scope);
        let mut session = Self {
            keys: HashMap::new(),
            store_binding: store.identity().stable_binding(),
            store,
            readmission_authority,
            segment_id: anchor.scope().segment_id(),
            generation: anchor.scope().generation(),
            replay_posture: LsmMembershipReplayPosture::DurableArtifactsReadmitted,
            reopen_counters: LsmMembershipReopenCounters::default(),
        };
        session.rebuild_from_store()?;
        Ok(session)
    }

    pub const fn replay_posture(&self) -> LsmMembershipReplayPosture {
        self.replay_posture
    }

    pub const fn reopen_counters(&self) -> LsmMembershipReopenCounters {
        self.reopen_counters
    }

    pub fn persist(&mut self, record: LsmMembershipRecord) -> Result<(), LsmMembershipDenial> {
        let slot =
            component_slot(record.kind()).ok_or(LsmMembershipDenial::UnsupportedRecordKind)?;
        if record.durable_scope().segment_id() != self.segment_id
            || record.durable_scope().generation() != self.generation
            || !self.store.admits_persisted_path(&record.persisted_path)
        {
            return Err(LsmMembershipDenial::StoreBindingMismatch);
        }
        if !super::artifact::persisted_artifact_matches(
            &record.persisted_path,
            record.persisted_bytes,
            &super::artifact::lsm_membership_record_bytes(&record.envelope, record.key),
        ) {
            return Err(LsmMembershipDenial::DurableRecordBindingMismatch);
        }
        let state = self.keys.entry(record.key()).or_default();
        if let Some(existing) = state.records[slot].as_ref().filter(|entry| !entry.retired) {
            return same_persisted_record(&existing.record, &record)
                .then_some(())
                .ok_or(LsmMembershipDenial::MembershipAmbiguous);
        }
        state.records[slot] = Some(RecordState {
            record,
            retired: false,
        });
        state.version = state.version.saturating_add(1);
        Ok(())
    }

    pub fn select_compaction(
        &self,
        key: LsmMembershipKey,
    ) -> Result<LsmCompactionMembership, LsmMembershipDenial> {
        let state = self
            .keys
            .get(&key)
            .ok_or(LsmMembershipDenial::MembershipIncomplete)?;
        if state
            .published_replacement
            .as_ref()
            .is_some_and(|base| !base.artifact_is_current())
        {
            return Err(LsmMembershipDenial::ReplacementOutputMismatch);
        }
        let selected = state.records.each_ref().map(|entry| {
            entry
                .as_ref()
                .filter(|entry| !entry.retired)
                .map(|entry| entry.record.clone())
        });
        let [Some(value), Some(generation), Some(tombstone)] = selected else {
            return Err(LsmMembershipDenial::MembershipIncomplete);
        };
        Ok(LsmCompactionMembership {
            key,
            records: [value, generation, tombstone],
            base: state.published_replacement.clone(),
            version: state.version,
            store_binding: self.store_binding.clone(),
            partition_probes: 1,
            component_probes: 3,
        })
    }

    pub fn replace(
        &mut self,
        selected: &LsmCompactionMembership,
        replacement: &AdmittedLsmMembershipReplacement,
    ) -> Result<PublishedLsmMembershipReplacement, LsmMembershipDenial> {
        let expected = selected.identities();
        let state = self
            .keys
            .get_mut(&selected.key())
            .ok_or(LsmMembershipDenial::MembershipStale)?;
        if !replacement.binds(selected)
            || selected.store_binding() != self.store_binding
            || !selected_state_matches(
                state,
                expected,
                selected.base().map(|base| base.output()),
                selected.version(),
            )
        {
            return Err(LsmMembershipDenial::MembershipStale);
        }
        if !manifest_matches_membership(
            selected,
            replacement.output().identity(),
            replacement.output().scope(),
            replacement.scope(),
            replacement.persisted_path(),
            replacement.persisted_bytes(),
        ) {
            return Err(LsmMembershipDenial::ManifestMembershipMismatch);
        }
        if !replacement_output_matches(
            selected,
            replacement.output().identity(),
            replacement.output().scope(),
            replacement.output().persisted_path(),
            replacement.output().persisted_bytes(),
        ) {
            return Err(LsmMembershipDenial::ReplacementOutputMismatch);
        }
        for entry in &mut state.records {
            entry.as_mut().expect("complete membership checked").retired = true;
        }
        let published = PublishedLsmMembershipReplacement::issued(
            selected.key(),
            expected,
            replacement.output().identity(),
            replacement.output().scope().clone(),
            replacement.scope().clone(),
            replacement.output().persisted_path().to_path_buf(),
            replacement.output().persisted_bytes(),
        );
        state.published_replacement = Some(published.clone());
        state.version = state.version.saturating_add(1);
        Ok(published)
    }

    pub fn published_replacement(
        &self,
        key: LsmMembershipKey,
    ) -> Result<PublishedLsmMembershipReplacement, LsmMembershipDenial> {
        self.keys
            .get(&key)
            .and_then(|state| state.published_replacement.clone())
            .ok_or(LsmMembershipDenial::MembershipIncomplete)
    }
}

fn same_persisted_record(left: &LsmMembershipRecord, right: &LsmMembershipRecord) -> bool {
    left.envelope == right.envelope
        && left.durable_scope == right.durable_scope
        && left.key == right.key
        && left.persisted_bytes == right.persisted_bytes
        && std::fs::canonicalize(&left.persisted_path).ok()
            == std::fs::canonicalize(&right.persisted_path).ok()
}

impl LsmMembershipReopenCounters {
    pub const fn artifacts_examined(self) -> u64 {
        self.artifacts_examined
    }

    pub const fn artifacts_readmitted(self) -> u64 {
        self.artifacts_readmitted
    }

    pub const fn bytes_examined(self) -> u64 {
        self.bytes_examined
    }
}

pub(super) fn component_slot(kind: crate::BlobWalRecordKind) -> Option<usize> {
    match kind {
        crate::BlobWalRecordKind::LsmValue => Some(0),
        crate::BlobWalRecordKind::GenerationPublication => Some(1),
        crate::BlobWalRecordKind::LsmTombstone => Some(2),
        _ => None,
    }
}

fn active_identities(state: &KeyState) -> Option<[BlobWalRecordIdentity; 3]> {
    let [value, generation, tombstone] = state.records.each_ref().map(|entry| {
        entry
            .as_ref()
            .filter(|record| !record.retired)
            .map(|record| record.record.identity())
    });
    Some([value?, generation?, tombstone?])
}

pub(super) fn selected_state_matches(
    state: &KeyState,
    identities: [BlobWalRecordIdentity; 3],
    base: Option<BlobWalRecordIdentity>,
    version: u64,
) -> bool {
    state.version == version
        && active_identities(state) == Some(identities)
        && state
            .published_replacement
            .as_ref()
            .map(|published| published.output())
            == base
}

pub(super) fn manifest_matches_membership(
    selected: &LsmCompactionMembership,
    output: BlobWalRecordIdentity,
    output_scope: &WalFrameDurablePublicationScope,
    scope: &crate::CheckpointDurablePublicationScope,
    path: &Path,
    bytes: u64,
) -> bool {
    let expected = super::artifact::lsm_membership_activation_digest_prefix(
        selected.key(),
        selected.identities(),
        selected.base().map(|base| base.output()),
        output,
        selected.store_binding(),
        output_scope,
    );
    activation_scope_matches(selected, output, scope)
        && scope
            .manifest_digest()
            .strip_prefix(&expected)
            .is_some_and(|physical| !physical.is_empty())
        && path.is_file()
        && std::fs::metadata(path).is_ok_and(|metadata| metadata.len() == bytes)
}

fn activation_scope_matches(
    selected: &LsmCompactionMembership,
    output: BlobWalRecordIdentity,
    scope: &crate::CheckpointDurablePublicationScope,
) -> bool {
    let expected_checkpoint = selected
        .base()
        .map(|base| base.activation_scope().checkpoint().checkpoint_epoch())
        .unwrap_or(0)
        .checked_add(1);
    let expected_start = selected
        .base()
        .map_or(selected.identities()[0].sequence(), |base| {
            base.activation_scope().covered_lsn_start()
        });
    expected_checkpoint == Some(scope.checkpoint().checkpoint_epoch())
        && scope.covered_lsn_start() == expected_start
        && output
            .sequence()
            .checked_add(1)
            .is_some_and(|end| scope.covered_lsn_end() == end)
}

pub(super) fn replacement_output_matches(
    selected: &LsmCompactionMembership,
    identity: BlobWalRecordIdentity,
    scope: &WalFrameDurablePublicationScope,
    path: &Path,
    bytes: u64,
) -> bool {
    selected.expected_output_identity() == Some(identity)
        && selected.records().iter().all(|record| {
            record.durable_scope().segment_id() == scope.segment_id()
                && record.durable_scope().generation() == scope.generation()
        })
        && scope.lsn_start() <= identity.sequence()
        && scope.lsn_end() >= identity.sequence()
        && scope.expected_bytes() == bytes
        && super::artifact::persisted_artifact_matches(
            path,
            bytes,
            &super::artifact::lsm_membership_output_bytes(scope),
        )
}

pub(super) fn decode_key(
    authority: LsmMembershipReadmissionAuthority,
    tenant: &str,
    scope: &str,
    canonical: &str,
) -> Result<LsmMembershipKey, LsmMembershipDenial> {
    LsmMembershipKey::readmit(
        authority,
        super::artifact::decode_tenant(number(tenant)?)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
        super::artifact::decode_key_scope(number(scope)?)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
        &super::artifact::unhex(canonical)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
    )
    .ok_or(LsmMembershipDenial::CanonicalKeyRequired)
}

pub(super) fn decode_text(value: &str) -> Result<String, LsmMembershipDenial> {
    String::from_utf8(
        super::artifact::unhex(value)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
    )
    .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}

pub(super) fn number<T: std::str::FromStr>(value: &str) -> Result<T, LsmMembershipDenial> {
    value
        .parse()
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}
