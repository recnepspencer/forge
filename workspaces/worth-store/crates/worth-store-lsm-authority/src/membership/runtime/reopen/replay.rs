use super::super::super::activation_artifact::{
    decode_activation, has_activation_magic, PersistedMembershipActivation,
};
use super::super::super::durable_artifact::{
    checksum, decode_key_scope, decode_kind, decode_tenant, unhex,
};
use super::super::super::model::{
    LsmCompactionMembership, LsmMembershipKey, LsmMembershipReadmissionAuthority,
    LsmMembershipRecord,
};
use super::super::persistence::component_slot;
use super::super::replacement::{
    manifest_matches_membership, replacement_output_matches, selected_state_matches,
};
use super::super::state::{LsmMembershipDenial, LsmMembershipSession, RecordState};
use crate::{
    BlobWalRecordEnvelope, BlobWalRecordIdentity, LogSequenceNumber, PublicationDeclaration,
    PublishedLsmMembershipIdentity, PublishedLsmMembershipReplacement, WalFramePublicationScope,
    WalLsnRange, WalSegmentGeneration, WalSegmentId,
};
use std::path::{Path, PathBuf};

enum DurableMembershipEvent {
    Record(LsmMembershipRecord),
    Activation {
        activation: PersistedMembershipActivation,
        identity: PublishedLsmMembershipIdentity,
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

pub(super) fn rebuild_from_store(
    session: &mut LsmMembershipSession,
) -> Result<(), LsmMembershipDenial> {
    const MAX_MEMBERSHIP_ARTIFACT_BYTES: u64 = 4 * 1024 * 1024;

    let artifacts = session
        .store
        .scan()
        .map_err(super::operation::map_store_denial)?;
    session.reopen_counters.artifacts_examined = artifacts.artifacts().len() as u64;
    session.reopen_counters.bytes_examined = artifacts.counters().bytes_read();
    let mut events = Vec::new();
    for artifact in artifacts.artifacts() {
        let read = artifact
            .read_bounded(MAX_MEMBERSHIP_ARTIFACT_BYTES)
            .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        session.reopen_counters.bytes_examined = session
            .reopen_counters
            .bytes_examined
            .saturating_add(read.bytes_read());
        let bytes = read.bytes();
        if let Some(record) = decode_record_artifact(
            artifact.path(),
            artifact.offset(),
            bytes,
            session.readmission_authority,
        )? {
            events.push(DurableMembershipEvent::Record(record));
            continue;
        }
        if has_activation_magic(bytes) {
            let activation = decode_activation(bytes, session.readmission_authority)
                .map_err(|_| LsmMembershipDenial::ManifestMembershipMismatch)?;
            events.push(DurableMembershipEvent::Activation {
                activation,
                identity: PublishedLsmMembershipIdentity::from_activation_bytes(bytes),
                path: artifact.path().to_path_buf(),
                bytes: read.bytes_read(),
            });
        }
    }
    events.sort_by_key(DurableMembershipEvent::ordering_sequence);
    for event in events {
        match event {
            DurableMembershipEvent::Record(record) => replay_record(session, record)?,
            DurableMembershipEvent::Activation {
                activation,
                identity,
                path,
                bytes,
            } => replay_activation(session, activation, identity, path, bytes)?,
        }
        session.reopen_counters.artifacts_readmitted = session
            .reopen_counters
            .artifacts_readmitted
            .saturating_add(1);
    }
    Ok(())
}

fn replay_record(
    session: &mut LsmMembershipSession,
    record: LsmMembershipRecord,
) -> Result<(), LsmMembershipDenial> {
    if !session.store.admits_path(&record.persisted_path)
        || record.durable_scope.segment_id() != session.segment_id
        || record.durable_scope.generation() != session.generation
    {
        return Err(LsmMembershipDenial::DurableRecordBindingMismatch);
    }
    let slot = component_slot(record.kind()).ok_or(LsmMembershipDenial::UnsupportedRecordKind)?;
    let state = session.keys.entry(record.key()).or_default();
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
    session: &mut LsmMembershipSession,
    activation: PersistedMembershipActivation,
    identity: PublishedLsmMembershipIdentity,
    path: PathBuf,
    persisted_bytes: u64,
) -> Result<(), LsmMembershipDenial> {
    let selected = selected_membership_for_activation(session, &activation)?;
    if !session.store.admits_path(&path)
        || !manifest_matches_membership(
            &selected,
            activation.output_identity,
            &activation.output_scope,
            &activation.scope,
            &path,
            persisted_bytes,
        )
    {
        return Err(LsmMembershipDenial::ManifestMembershipMismatch);
    }
    if !replacement_output_matches(
        &selected,
        activation.output_identity,
        &activation.output_scope,
        &activation.output_path,
        activation.output_offset,
        activation.output_bytes,
    ) {
        return Err(LsmMembershipDenial::ReplacementOutputMismatch);
    }
    let key = activation.key;
    let state = session
        .keys
        .get_mut(&key)
        .ok_or(LsmMembershipDenial::MembershipStale)?;
    for entry in &mut state.records {
        entry
            .as_mut()
            .ok_or(LsmMembershipDenial::MembershipIncomplete)?
            .retired = true;
    }
    state.published_replacement = Some(PublishedLsmMembershipReplacement::issued(
        identity,
        key,
        selected.identity_set(),
        activation.output_identity,
        activation.output_scope,
        activation.scope,
        super::super::replacement::PublishedLsmMembershipOutputArtifact::new(
            activation.output_path,
            activation.output_offset,
            activation.output_bytes,
        ),
    ));
    state.version = state.version.saturating_add(1);
    Ok(())
}

fn selected_membership_for_activation(
    session: &LsmMembershipSession,
    activation: &PersistedMembershipActivation,
) -> Result<LsmCompactionMembership, LsmMembershipDenial> {
    let state = session
        .keys
        .get(&activation.key)
        .ok_or(LsmMembershipDenial::MembershipStale)?;
    if !selected_state_matches(
        state,
        activation.selected_identities,
        activation.selected_base,
        activation.selected_version,
    ) {
        return Err(LsmMembershipDenial::ManifestMembershipMismatch);
    }
    let value = state.records[0]
        .as_ref()
        .ok_or(LsmMembershipDenial::ValueRecordRequired)?
        .record
        .clone();
    let generation = state.records[1]
        .as_ref()
        .ok_or(LsmMembershipDenial::GenerationRecordRequired)?
        .record
        .clone();
    let tombstone = state.records[2]
        .as_ref()
        .ok_or(LsmMembershipDenial::TombstoneRecordRequired)?
        .record
        .clone();
    let record_set = crate::membership::LsmCompactionRecordSet::issue(
        activation.key,
        value,
        generation,
        tombstone,
    )?;
    Ok(LsmCompactionMembership {
        key: activation.key,
        record_set,
        base: state.published_replacement.clone(),
        version: activation.selected_version,
        store_binding: session.store_binding.clone(),
        partition_probes: 1,
        component_probes: 3,
    })
}

fn decode_record_artifact(
    path: &Path,
    offset: u64,
    artifact: &[u8],
    authority: LsmMembershipReadmissionAuthority,
) -> Result<Option<LsmMembershipRecord>, LsmMembershipDenial> {
    if !artifact.starts_with(b"worth-store:wal-lsm-membership:v1 ") {
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
    let scope = decode_record_publication_scope(&row)?;
    let envelope = BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(number(row[4])?, kind)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
        PublicationDeclaration::wal_frame(scope.clone()),
        decode_text(row[12])?,
    )
    .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    Ok(Some(LsmMembershipRecord {
        envelope,
        durable_scope: scope,
        key,
        persisted_path: path.to_path_buf(),
        persisted_offset: offset,
        persisted_bytes: artifact.len() as u64,
    }))
}

fn decode_record_publication_scope(
    row: &[&str],
) -> Result<WalFramePublicationScope, LsmMembershipDenial> {
    let segment = WalSegmentId::new(number(row[6])?)
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    let generation = WalSegmentGeneration::new(number(row[7])?)
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    let lsn_range = WalLsnRange::new(
        LogSequenceNumber::new(number(row[8])?),
        LogSequenceNumber::new(number(row[9])?),
    )
    .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    WalFramePublicationScope::new(
        segment,
        generation,
        lsn_range,
        decode_text(row[10])?,
        number(row[11])?,
    )
    .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}

fn decode_key(
    authority: LsmMembershipReadmissionAuthority,
    tenant: &str,
    scope: &str,
    canonical: &str,
) -> Result<LsmMembershipKey, LsmMembershipDenial> {
    LsmMembershipKey::readmit(
        authority,
        decode_tenant(number(tenant)?)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
        decode_key_scope(number(scope)?)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
        &unhex(canonical).ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?,
    )
    .ok_or(LsmMembershipDenial::CanonicalKeyRequired)
}

fn decode_text(value: &str) -> Result<String, LsmMembershipDenial> {
    String::from_utf8(unhex(value).ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?)
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}

fn number<T: std::str::FromStr>(value: &str) -> Result<T, LsmMembershipDenial> {
    value
        .parse()
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}
