use super::activation_artifact::{
    decode_activation, has_activation_magic, PersistedMembershipActivation,
};
use super::artifact::{checksum, decode_kind};
use super::model::{
    LsmCompactionMembership, LsmMembershipKey, LsmMembershipReadmissionAuthority,
    LsmMembershipRecord,
};
use super::session::{
    component_slot, decode_key, decode_text, manifest_matches_membership, number,
    replacement_output_matches, selected_state_matches, LsmMembershipDenial, LsmMembershipSession,
    RecordState,
};
use crate::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, DurablePublicationDeclaration,
    PublishedLsmMembershipReplacement, WalFrameDurablePublicationScope,
};
use std::path::{Path, PathBuf};

enum DurableMembershipEvent {
    Record(LsmMembershipRecord),
    Activation {
        activation: PersistedMembershipActivation,
        path: PathBuf,
        bytes: u64,
    },
}

impl DurableMembershipEvent {
    fn ordering_sequence(&self) -> u64 {
        match self {
            Self::Record(record) => record.identity().sequence(),
            Self::Activation { activation, .. } => activation.ordering_sequence(),
        }
    }
}

impl LsmMembershipSession {
    pub(super) fn rebuild_from_store(&mut self) -> Result<(), LsmMembershipDenial> {
        let artifacts = self.store.scan().map_err(|_| LsmMembershipDenial::Io)?;
        self.reopen_counters.artifacts_examined = artifacts.artifacts().len() as u64;
        self.reopen_counters.bytes_examined = artifacts.counters().bytes_read();
        let mut events = Vec::new();
        for artifact in artifacts.artifacts() {
            if let Some(record) = decode_record_artifact(
                artifact.path(),
                artifact.bytes(),
                self.readmission_authority,
            )? {
                events.push(DurableMembershipEvent::Record(record));
                continue;
            }
            if has_activation_magic(artifact.bytes()) {
                let activation = decode_activation(artifact.bytes(), self.readmission_authority)
                    .map_err(|_| LsmMembershipDenial::ManifestMembershipMismatch)?;
                events.push(DurableMembershipEvent::Activation {
                    activation,
                    path: artifact.path().to_path_buf(),
                    bytes: artifact.bytes().len() as u64,
                });
            }
        }
        events.sort_by_key(DurableMembershipEvent::ordering_sequence);
        for event in events {
            match event {
                DurableMembershipEvent::Record(record) => self.replay_record(record)?,
                DurableMembershipEvent::Activation {
                    activation,
                    path,
                    bytes,
                } => self.replay_activation(activation, path, bytes)?,
            }
            self.reopen_counters.artifacts_readmitted =
                self.reopen_counters.artifacts_readmitted.saturating_add(1);
        }
        Ok(())
    }

    fn replay_record(&mut self, record: LsmMembershipRecord) -> Result<(), LsmMembershipDenial> {
        if !self.store.admits_persisted_path(&record.persisted_path)
            || record.durable_scope.segment_id() != self.segment_id
            || record.durable_scope.generation() != self.generation
        {
            return Err(LsmMembershipDenial::DurableRecordBindingMismatch);
        }
        let slot =
            component_slot(record.kind()).ok_or(LsmMembershipDenial::UnsupportedRecordKind)?;
        let state = self.keys.entry(record.key()).or_default();
        if state.records[slot]
            .as_ref()
            .is_some_and(|entry| !entry.retired)
        {
            return Err(LsmMembershipDenial::MembershipAmbiguous);
        }
        state.records[slot] = Some(RecordState {
            record,
            retired: false,
        });
        state.version = state.version.saturating_add(1);
        Ok(())
    }

    fn replay_activation(
        &mut self,
        activation: PersistedMembershipActivation,
        path: PathBuf,
        persisted_bytes: u64,
    ) -> Result<(), LsmMembershipDenial> {
        let PersistedMembershipActivation {
            key,
            selected_version,
            selected_identities,
            selected_base,
            output_identity,
            output_scope,
            output_path,
            output_bytes,
            scope,
        } = activation;
        self.replay_replace(
            key,
            selected_version,
            selected_identities,
            selected_base,
            output_identity,
            output_scope,
            output_path,
            output_bytes,
            scope,
            path,
            persisted_bytes,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn replay_replace(
        &mut self,
        key: LsmMembershipKey,
        selected_version: u64,
        selected_identities: [BlobWalRecordIdentity; 3],
        selected_base: Option<BlobWalRecordIdentity>,
        output_identity: BlobWalRecordIdentity,
        output_scope: WalFrameDurablePublicationScope,
        output_path: PathBuf,
        output_bytes: u64,
        scope: crate::CheckpointDurablePublicationScope,
        path: PathBuf,
        bytes: u64,
    ) -> Result<(), LsmMembershipDenial> {
        let state = self
            .keys
            .get_mut(&key)
            .ok_or(LsmMembershipDenial::MembershipStale)?;
        if !selected_state_matches(state, selected_identities, selected_base, selected_version) {
            return Err(LsmMembershipDenial::ManifestMembershipMismatch);
        }
        let selected = LsmCompactionMembership {
            key,
            records: state.records.each_ref().map(|entry| {
                entry
                    .as_ref()
                    .expect("selected state checked")
                    .record
                    .clone()
            }),
            base: state.published_replacement.clone(),
            version: selected_version,
            store_binding: self.store_binding.clone(),
            partition_probes: 1,
            component_probes: 3,
        };
        if !self.store.admits_persisted_path(&path)
            || !manifest_matches_membership(
                &selected,
                output_identity,
                &output_scope,
                &scope,
                &path,
                bytes,
            )
        {
            return Err(LsmMembershipDenial::ManifestMembershipMismatch);
        }
        if !replacement_output_matches(
            &selected,
            output_identity,
            &output_scope,
            &output_path,
            output_bytes,
        ) {
            return Err(LsmMembershipDenial::ReplacementOutputMismatch);
        }
        for entry in &mut state.records {
            entry
                .as_mut()
                .ok_or(LsmMembershipDenial::MembershipIncomplete)?
                .retired = true;
        }
        state.published_replacement = Some(PublishedLsmMembershipReplacement::issued(
            key,
            selected_identities,
            output_identity,
            output_scope,
            scope,
            output_path,
            output_bytes,
        ));
        state.version = state.version.saturating_add(1);
        Ok(())
    }
}

fn decode_record_artifact(
    path: &Path,
    artifact: &[u8],
    authority: LsmMembershipReadmissionAuthority,
) -> Result<Option<LsmMembershipRecord>, LsmMembershipDenial> {
    if !artifact.starts_with(b"forge-store:wal-lsm-membership:v1 ") {
        return Ok(None);
    }
    let end = artifact
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(artifact.len());
    let canonical = std::str::from_utf8(&artifact[..end])
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    let (body, observed_checksum) = canonical
        .rsplit_once(' ')
        .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    if u64::from_str_radix(observed_checksum, 16).ok() != Some(checksum(body.as_bytes())) {
        return Err(LsmMembershipDenial::DurableRecordBindingMismatch);
    }
    let row: Vec<_> = body.split_ascii_whitespace().collect();
    if row.len() != 13 {
        return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid);
    }
    let key = decode_key(authority, row[1], row[2], row[3])?;
    let kind = decode_kind(number(row[5])?)
        .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    let scope = WalFrameDurablePublicationScope::new(
        number(row[6])?,
        number(row[7])?,
        number(row[8])?,
        number(row[9])?,
        decode_text(row[10])?,
        number(row[11])?,
    )
    .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(number(row[4])?, kind)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
        DurablePublicationDeclaration::wal_frame(scope.clone()),
        decode_text(row[12])?,
    )
    .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    Ok(Some(LsmMembershipRecord {
        envelope,
        durable_scope: scope,
        key,
        persisted_path: path.to_path_buf(),
        persisted_bytes: artifact.len() as u64,
    }))
}
