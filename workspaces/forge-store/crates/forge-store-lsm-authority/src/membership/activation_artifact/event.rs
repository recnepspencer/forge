use super::super::artifact::{
    decode_key_scope, decode_kind, decode_tenant, key_scope_code, record_kind_code, tenant_code,
};
use super::super::model::{
    LsmCompactionMembership, LsmMembershipKey, LsmMembershipReadmissionAuthority,
};
use super::super::session::LsmMembershipDenial;
use crate::{
    BlobWalRecordIdentity, CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity,
    WalFrameDurablePublicationScope,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PersistedMembershipActivation {
    pub(crate) key: LsmMembershipKey,
    pub(crate) selected_version: u64,
    pub(crate) selected_identities: [BlobWalRecordIdentity; 3],
    pub(crate) selected_base: Option<BlobWalRecordIdentity>,
    pub(crate) output_identity: BlobWalRecordIdentity,
    pub(crate) output_scope: WalFrameDurablePublicationScope,
    pub(crate) output_path: PathBuf,
    pub(crate) output_bytes: u64,
    pub(crate) scope: CheckpointDurablePublicationScope,
}

impl PersistedMembershipActivation {
    pub(crate) fn from_publication(
        selected: &LsmCompactionMembership,
        output: &crate::AdmittedLsmReplacementOutput,
        scope: CheckpointDurablePublicationScope,
    ) -> Self {
        Self {
            key: selected.key(),
            selected_version: selected.version(),
            selected_identities: selected.identities(),
            selected_base: selected.base().map(|base| base.output()),
            output_identity: output.identity(),
            output_scope: output.scope().clone(),
            output_path: output.persisted_path().to_path_buf(),
            output_bytes: output.persisted_bytes(),
            scope,
        }
    }

    pub(crate) const fn ordering_sequence(&self) -> u64 {
        self.output_identity.sequence()
    }

    pub(super) fn encode_payload(&self) -> Result<Vec<u8>, LsmMembershipDenial> {
        let mut writer = EventWriter::default();
        writer.u8(tenant_code(self.key.tenant_scope()));
        writer.u8(key_scope_code(self.key.key_scope()));
        writer.bytes(self.key.canonical())?;
        writer.u64(self.selected_version);
        for identity in self.selected_identities {
            writer.u64(identity.sequence());
            writer.u8(record_kind_code(identity.kind()));
        }
        match self.selected_base {
            Some(identity) => {
                writer.u8(1);
                writer.u64(identity.sequence());
                writer.u8(record_kind_code(identity.kind()));
            }
            None => writer.u8(0),
        }
        writer.u64(self.output_identity.sequence());
        writer.u8(record_kind_code(self.output_identity.kind()));
        writer.u64(self.output_scope.segment_id());
        writer.u64(self.output_scope.generation());
        writer.u64(self.output_scope.lsn_start());
        writer.u64(self.output_scope.lsn_end());
        writer.bytes(self.output_scope.frame_digest().as_bytes())?;
        writer.u64(self.output_scope.expected_bytes());
        writer.path(&self.output_path)?;
        writer.u64(self.output_bytes);
        writer.u64(self.scope.checkpoint().checkpoint_epoch());
        writer.u64(self.scope.covered_lsn_start());
        writer.u64(self.scope.covered_lsn_end());
        writer.bytes(self.scope.manifest_digest().as_bytes())?;
        Ok(writer.finish())
    }

    pub(super) fn decode(
        payload: &[u8],
        authority: LsmMembershipReadmissionAuthority,
    ) -> Result<Self, LsmMembershipDenial> {
        let mut reader = EventReader::new(payload);
        let tenant = decode_tenant(reader.u8()?)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        let key_scope = decode_key_scope(reader.u8()?)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        let key = LsmMembershipKey::readmit(authority, tenant, key_scope, reader.bytes()?)
            .ok_or(LsmMembershipDenial::CanonicalKeyRequired)?;
        let selected_version = reader.u64()?;
        let selected_identities = [
            decode_identity(&mut reader)?,
            decode_identity(&mut reader)?,
            decode_identity(&mut reader)?,
        ];
        let selected_base = match reader.u8()? {
            0 => None,
            1 => Some(decode_identity(&mut reader)?),
            _ => return Err(LsmMembershipDenial::PersistedMembershipArtifactInvalid),
        };
        let output_identity = decode_identity(&mut reader)?;
        let output_scope = WalFrameDurablePublicationScope::new(
            reader.u64()?,
            reader.u64()?,
            reader.u64()?,
            reader.u64()?,
            text(reader.bytes()?)?,
            reader.u64()?,
        )
        .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        let output_path = reader.path()?;
        let output_bytes = reader.u64()?;
        let checkpoint = StoreCheckpointRecordIdentity::new(reader.u64()?);
        let covered_lsn_start = reader.u64()?;
        let covered_lsn_end = reader.u64()?;
        let manifest_digest = text(reader.bytes()?)?;
        let scope = CheckpointDurablePublicationScope::new(
            checkpoint,
            manifest_digest,
            covered_lsn_start,
            covered_lsn_end,
        )
        .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        reader.finish()?;
        Ok(Self {
            key,
            selected_version,
            selected_identities,
            selected_base,
            output_identity,
            output_scope,
            output_path,
            output_bytes,
            scope,
        })
    }
}

fn decode_identity(
    reader: &mut EventReader<'_>,
) -> Result<BlobWalRecordIdentity, LsmMembershipDenial> {
    let sequence = reader.u64()?;
    let kind =
        decode_kind(reader.u8()?).ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
    BlobWalRecordIdentity::new(sequence, kind)
        .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}

fn text(bytes: &[u8]) -> Result<String, LsmMembershipDenial> {
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)
}

#[derive(Default)]
struct EventWriter {
    bytes: Vec<u8>,
}

impl EventWriter {
    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    fn bytes(&mut self, value: &[u8]) -> Result<(), LsmMembershipDenial> {
        let len = u32::try_from(value.len())
            .map_err(|_| LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        self.bytes.extend_from_slice(&len.to_le_bytes());
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    fn path(&mut self, value: &std::path::Path) -> Result<(), LsmMembershipDenial> {
        self.bytes(value.as_os_str().to_string_lossy().as_bytes())
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

struct EventReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> EventReader<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn u8(&mut self) -> Result<u8, LsmMembershipDenial> {
        let value = *self
            .bytes
            .get(self.offset)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        self.offset += 1;
        Ok(value)
    }

    fn u64(&mut self) -> Result<u64, LsmMembershipDenial> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn bytes(&mut self) -> Result<&'a [u8], LsmMembershipDenial> {
        let len = u32::from_le_bytes(self.take(4)?.try_into().unwrap()) as usize;
        self.take(len)
    }

    fn path(&mut self) -> Result<PathBuf, LsmMembershipDenial> {
        Ok(PathBuf::from(text(self.bytes()?)?))
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], LsmMembershipDenial> {
        let end = self
            .offset
            .checked_add(len)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)?;
        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), LsmMembershipDenial> {
        (self.offset == self.bytes.len())
            .then_some(())
            .ok_or(LsmMembershipDenial::PersistedMembershipArtifactInvalid)
    }
}
