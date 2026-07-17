use std::path::PathBuf;

use super::{
    manifest_encoding::MANIFEST_MAGIC, manifest_vocabulary, DisasterRecoveryArtifactEvidence,
    DisasterRecoveryBundleDenial, DisasterRecoveryComponent, DisasterRecoveryComponentSemantics,
    DisasterRecoverySecurityBinding,
};
use crate::{ReplicaRecoveryFrontier, ReplicationLineageIdentity};

pub(super) struct DecodedDisasterRecoveryManifest {
    pub(super) source_lineage: ReplicationLineageIdentity,
    pub(super) frontier: ReplicaRecoveryFrontier,
    pub(super) security: DisasterRecoverySecurityBinding,
    pub(super) expected_rpo_lsn: u64,
    pub(super) components: Vec<DisasterRecoveryComponent>,
}

pub(super) fn decode_manifest(
    bytes: &[u8],
) -> Result<DecodedDisasterRecoveryManifest, DisasterRecoveryBundleDenial> {
    let mut cursor = ManifestCursor::new(bytes);
    if cursor.read_exact(8)? != MANIFEST_MAGIC {
        return Err(DisasterRecoveryBundleDenial::ManifestMalformed);
    }
    let source_lineage = ReplicationLineageIdentity::from_declared_lineage(cursor.read_text()?)
        .ok_or(DisasterRecoveryBundleDenial::ManifestMalformed)?;
    let frontier = ReplicaRecoveryFrontier::admit(
        cursor.read_u64()?,
        cursor.read_u64()?,
        cursor.read_u64()?,
        cursor.read_u64()?,
        cursor.read_u64()?,
    )
    .map_err(|_| DisasterRecoveryBundleDenial::ManifestMalformed)?;
    let security = DisasterRecoverySecurityBinding::from_persisted_description(
        cursor.read_identity()?,
        manifest_vocabulary::key_scope(cursor.read_u8()?)?,
        manifest_vocabulary::key_version(cursor.read_u8()?)?,
        manifest_vocabulary::tenant_scope(cursor.read_u8()?)?,
        manifest_vocabulary::authenticity(cursor.read_u8()?)?,
        manifest_vocabulary::custody(cursor.read_u8()?)?,
    )
    .ok_or(DisasterRecoveryBundleDenial::ManifestMalformed)?;
    let expected_rpo_lsn = cursor.read_u64()?;
    let component_count = usize::try_from(cursor.read_u32()?)
        .map_err(|_| DisasterRecoveryBundleDenial::ManifestTooLarge)?;
    let mut components = Vec::new();
    components
        .try_reserve_exact(component_count)
        .map_err(|_| DisasterRecoveryBundleDenial::AllocationFailed)?;
    for _ in 0..component_count {
        components.push(decode_component(&mut cursor)?);
    }
    if !cursor.is_exhausted() {
        return Err(DisasterRecoveryBundleDenial::ManifestMalformed);
    }
    Ok(DecodedDisasterRecoveryManifest {
        source_lineage,
        frontier,
        security,
        expected_rpo_lsn,
        components,
    })
}

fn decode_component(
    cursor: &mut ManifestCursor<'_>,
) -> Result<DisasterRecoveryComponent, DisasterRecoveryBundleDenial> {
    let relative_path = PathBuf::from(cursor.read_text()?);
    let evidence = DisasterRecoveryArtifactEvidence::admit(
        cursor.read_identity()?,
        cursor.read_u64()?,
        cursor.read_identity()?,
        cursor.read_identity()?,
    )?;
    DisasterRecoveryComponent::declare(relative_path, evidence, decode_semantics(cursor)?)
}

fn decode_semantics(
    cursor: &mut ManifestCursor<'_>,
) -> Result<DisasterRecoveryComponentSemantics, DisasterRecoveryBundleDenial> {
    match cursor.read_u8()? {
        1 => Ok(DisasterRecoveryComponentSemantics::Authority {
            lineage_identity: cursor.read_identity()?,
            authority_epoch: cursor.read_u64()?,
        }),
        2 => Ok(DisasterRecoveryComponentSemantics::Checkpoint {
            lineage_identity: cursor.read_identity()?,
            authority_epoch: cursor.read_u64()?,
            checkpoint_identity: cursor.read_identity()?,
            checkpoint_lsn: cursor.read_u64()?,
            blob_closure_identity: cursor.read_identity()?,
        }),
        3 => Ok(DisasterRecoveryComponentSemantics::Wal {
            lineage_identity: cursor.read_identity()?,
            authority_epoch: cursor.read_u64()?,
            start_lsn: cursor.read_u64()?,
            end_lsn_exclusive: cursor.read_u64()?,
        }),
        4 => Ok(DisasterRecoveryComponentSemantics::Page {
            checkpoint_identity: cursor.read_identity()?,
        }),
        5 => Ok(DisasterRecoveryComponentSemantics::Blob {
            blob_closure_identity: cursor.read_identity()?,
        }),
        6 => Ok(DisasterRecoveryComponentSemantics::Layout {
            checkpoint_identity: cursor.read_identity()?,
        }),
        _ => Err(DisasterRecoveryBundleDenial::ManifestMalformed),
    }
}

struct ManifestCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> ManifestCursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_exact(&mut self, length: usize) -> Result<&'a [u8], DisasterRecoveryBundleDenial> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(DisasterRecoveryBundleDenial::ManifestMalformed)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(DisasterRecoveryBundleDenial::ManifestMalformed)?;
        self.offset = end;
        Ok(value)
    }

    fn read_u8(&mut self) -> Result<u8, DisasterRecoveryBundleDenial> {
        self.read_exact(1)?
            .first()
            .copied()
            .ok_or(DisasterRecoveryBundleDenial::ManifestMalformed)
    }

    fn read_u32(&mut self) -> Result<u32, DisasterRecoveryBundleDenial> {
        let bytes: [u8; 4] = self
            .read_exact(4)?
            .try_into()
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestMalformed)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, DisasterRecoveryBundleDenial> {
        let bytes: [u8; 8] = self
            .read_exact(8)?
            .try_into()
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestMalformed)?;
        Ok(u64::from_be_bytes(bytes))
    }

    fn read_identity(&mut self) -> Result<[u8; 32], DisasterRecoveryBundleDenial> {
        self.read_exact(32)?
            .try_into()
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestMalformed)
    }

    fn read_text(&mut self) -> Result<String, DisasterRecoveryBundleDenial> {
        let length = usize::try_from(self.read_u32()?)
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestTooLarge)?;
        let bytes = self.read_exact(length)?;
        let text = std::str::from_utf8(bytes)
            .map_err(|_| DisasterRecoveryBundleDenial::ManifestMalformed)?;
        Ok(text.to_owned())
    }

    fn is_exhausted(&self) -> bool {
        self.offset == self.bytes.len()
    }
}
