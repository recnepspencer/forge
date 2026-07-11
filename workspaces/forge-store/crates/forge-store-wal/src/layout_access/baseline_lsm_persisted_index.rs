use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::persisted_codec::{
    decode_key, decode_kind, hex, key_scope_code, number, tenant_code, text,
};
use super::{component_slot, BaselineLsmAdmittedKey, BaselineLsmAdmittedRecord};
use crate::layout_access::baseline_lsm_counter_observation::BaselineLsmExecutionAdmissionDenial;
use crate::{
    AdmittedCheckpointPublicationReceipt, BlobWalRecordEnvelope, BlobWalRecordIdentity,
    DurablePublicationDeclaration, WalFrameDurablePublicationScope,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BaselineLsmPersistedRecord {
    pub(super) record: BaselineLsmAdmittedRecord,
    pub(super) retired: bool,
}

#[derive(Debug, Default)]
pub(super) struct BaselineLsmPersistedKeyState {
    pub(super) records: [Option<BaselineLsmPersistedRecord>; 3],
    pub(super) version: u64,
}

#[derive(Debug, Default)]
pub(super) struct BaselineLsmPersistedIndex {
    pub(super) keys: HashMap<BaselineLsmAdmittedKey, BaselineLsmPersistedKeyState>,
}

/// WAL-owned persistent membership session. The in-memory map is only a
/// key-local projection rebuilt from the append-only owner journal.
#[derive(Debug)]
pub struct BaselineLsmWalIndexSession {
    pub(super) index: BaselineLsmPersistedIndex,
    pub(super) artifact_root: PathBuf,
    pub(super) store_binding: String,
    store_segment_id: u64,
    store_generation: u64,
    journal_path: PathBuf,
}

impl BaselineLsmWalIndexSession {
    pub(crate) fn open(
        root: &Path,
        artifact_root: &Path,
        store_segment_id: u64,
        store_generation: u64,
    ) -> Result<Self, BaselineLsmExecutionAdmissionDenial> {
        std::fs::create_dir_all(root)
            .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
        let journal_path = root.join("baseline-lsm-membership.journal");
        if !journal_path.exists() {
            std::fs::File::create(&journal_path)
                .and_then(|file| file.sync_all())
                .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
        }
        let mut session = Self {
            index: BaselineLsmPersistedIndex::default(),
            artifact_root: artifact_root.to_path_buf(),
            store_binding: super::store_binding::store_binding(
                artifact_root,
                store_segment_id,
                store_generation,
            )?,
            store_segment_id,
            store_generation,
            journal_path,
        };
        session.replay_journal()?;
        Ok(session)
    }

    pub(crate) fn persist(
        &mut self,
        record: BaselineLsmAdmittedRecord,
    ) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
        let slot = component_slot(record.envelope.identity().kind())
            .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete)?;
        if record.durable_scope.segment_id() != self.store_segment_id
            || record.durable_scope.generation() != self.store_generation
        {
            return Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch);
        }
        if !record.belongs_to_wal_root(&self.artifact_root) {
            return Err(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch);
        }
        let state = self.index.keys.entry(record.key).or_default();
        if state.records[slot]
            .as_ref()
            .is_some_and(|entry| !entry.retired)
        {
            return Err(BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous);
        }
        append_and_sync(&self.journal_path, &encode_add(&record))?;
        state.records[slot] = Some(BaselineLsmPersistedRecord {
            record,
            retired: false,
        });
        state.version = state.version.saturating_add(1);
        Ok(())
    }

    pub(super) fn retire(
        &mut self,
        key: BaselineLsmAdmittedKey,
        key_version: u64,
        expected: [BlobWalRecordIdentity; 3],
        manifest: &AdmittedCheckpointPublicationReceipt,
    ) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
        let state = self
            .index
            .keys
            .get_mut(&key)
            .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedMembershipStale)?;
        if state.version != key_version
            || state.records.iter().zip(expected).any(|(entry, identity)| {
                entry.as_ref().is_none_or(|entry| {
                    entry.retired || entry.record.envelope.identity() != identity
                })
            })
        {
            return Err(BaselineLsmExecutionAdmissionDenial::PersistedMembershipStale);
        }
        append_and_sync(&self.journal_path, &encode_retire(key, manifest))?;
        for entry in &mut state.records {
            entry
                .as_mut()
                .expect("validated complete membership")
                .retired = true;
        }
        state.version = state.version.saturating_add(1);
        Ok(())
    }

    fn replay_journal(&mut self) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
        let journal = std::fs::read_to_string(&self.journal_path)
            .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
        for line in journal.lines() {
            if !line.is_empty() {
                replay_line(
                    &mut self.index,
                    &self.artifact_root,
                    &self.store_binding,
                    self.store_segment_id,
                    self.store_generation,
                    line,
                )?;
            }
        }
        Ok(())
    }
}

fn append_and_sync(path: &Path, line: &str) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    writeln!(file, "{line}").map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    file.sync_all()
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)
}

fn encode_add(record: &BaselineLsmAdmittedRecord) -> String {
    format!(
        "A {} {}",
        hex(record
            .persisted_path
            .as_os_str()
            .to_string_lossy()
            .as_bytes()),
        record.persisted_bytes,
    )
}

fn encode_retire(
    key: BaselineLsmAdmittedKey,
    manifest: &AdmittedCheckpointPublicationReceipt,
) -> String {
    format!(
        "R {} {} {} {} {}",
        tenant_code(key.tenant_scope()),
        key_scope_code(key.key_scope()),
        hex(&key.canonical_key_bytes()),
        hex(manifest
            .persisted_path()
            .as_os_str()
            .to_string_lossy()
            .as_bytes()),
        manifest.persisted_bytes(),
    )
}

fn replay_line(
    index: &mut BaselineLsmPersistedIndex,
    wal_root: &Path,
    store_binding: &str,
    store_segment_id: u64,
    store_generation: u64,
    line: &str,
) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
    let fields: Vec<_> = line.split_ascii_whitespace().collect();
    match fields.first().copied() {
        Some("A") if fields.len() == 3 => {
            replay_add(index, wal_root, store_segment_id, store_generation, &fields)
        }
        Some("R") if fields.len() == 6 => replay_retire(index, wal_root, store_binding, &fields),
        _ => Err(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo),
    }
}

fn replay_add(
    index: &mut BaselineLsmPersistedIndex,
    wal_root: &Path,
    store_segment_id: u64,
    store_generation: u64,
    fields: &[&str],
) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
    let persisted_path = PathBuf::from(text(fields[1])?);
    let persisted_bytes = number(fields[2])?;
    let artifact = std::fs::read(&persisted_path)
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
    if artifact.len() as u64 != persisted_bytes {
        return Err(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch);
    }
    let canonical_len = artifact
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(artifact.len());
    let canonical = std::str::from_utf8(&artifact[..canonical_len])
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
    let (body, checksum) = canonical
        .rsplit_once(' ')
        .ok_or(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
    let observed_checksum = u64::from_str_radix(checksum, 16)
        .map_err(|_| BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
    if observed_checksum != super::artifact_binding::record_artifact_checksum(body.as_bytes()) {
        return Err(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch);
    }
    let fields: Vec<_> = body.split_ascii_whitespace().collect();
    if fields.len() != 13 || fields[0] != "forge-store:baseline-lsm-record:v2" {
        return Err(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch);
    }
    let key = decode_key(fields[1], fields[2], fields[3])?;
    let sequence = number(fields[4])?;
    let kind = decode_kind(number::<u8>(fields[5])?)?;
    let scope = WalFrameDurablePublicationScope::new(
        number(fields[6])?,
        number(fields[7])?,
        number(fields[8])?,
        number(fields[9])?,
        text(fields[10])?,
        number(fields[11])?,
    )
    .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    if scope.segment_id() != store_segment_id || scope.generation() != store_generation {
        return Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch);
    }
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind)
            .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?,
        DurablePublicationDeclaration::wal_frame(scope.clone()),
        text(fields[12])?,
    )
    .map_err(|_| BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    let record = BaselineLsmAdmittedRecord::readmit_persisted(
        envelope,
        scope,
        key,
        persisted_path,
        persisted_bytes,
        wal_root,
    )
    .ok_or(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)?;
    let slot = component_slot(kind)
        .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete)?;
    let state = index.keys.entry(key).or_default();
    if state.records[slot]
        .as_ref()
        .is_some_and(|entry| !entry.retired)
    {
        return Err(BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous);
    }
    state.records[slot] = Some(BaselineLsmPersistedRecord {
        record,
        retired: false,
    });
    state.version = state.version.saturating_add(1);
    Ok(())
}

fn replay_retire(
    index: &mut BaselineLsmPersistedIndex,
    wal_root: &Path,
    store_binding: &str,
    fields: &[&str],
) -> Result<(), BaselineLsmExecutionAdmissionDenial> {
    let key = decode_key(fields[1], fields[2], fields[3])?;
    let manifest_path = PathBuf::from(text(fields[4])?);
    let manifest_bytes = number(fields[5])?;
    let state = index
        .keys
        .get_mut(&key)
        .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
    let identities = state.records.each_ref().map(|entry| {
        entry
            .as_ref()
            .filter(|entry| !entry.retired)
            .map(|entry| entry.record.envelope.identity())
    });
    let [Some(value), Some(generation), Some(tombstone)] = identities else {
        return Err(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo);
    };
    let expected_digest = super::baseline_lsm_manifest_membership_digest(
        key,
        [value, generation, tombstone],
        store_binding,
    );
    let manifest =
        super::store_binding::manifest_membership(&manifest_path, manifest_bytes, wal_root)?;
    let covered_start = state
        .records
        .iter()
        .flatten()
        .map(|entry| entry.record.durable_scope.lsn_start())
        .min()
        .ok_or(BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch)?;
    let covered_end = state
        .records
        .iter()
        .flatten()
        .map(|entry| entry.record.durable_scope.lsn_end())
        .max()
        .ok_or(BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch)?;
    if manifest.checkpoint_epoch == 0
        || manifest.digest != expected_digest
        || manifest.covered_lsn_start > covered_start
        || manifest.covered_lsn_end < covered_end
    {
        return Err(BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch);
    }
    for entry in state.records.iter_mut() {
        let entry = entry
            .as_mut()
            .filter(|entry| !entry.retired)
            .ok_or(BaselineLsmExecutionAdmissionDenial::PersistedIndexIo)?;
        entry.retired = true;
    }
    state.version = state.version.saturating_add(1);
    Ok(())
}
