use worth_store_physical_format::{DurablePhysicalRootManifest, PersistedRecordIdentity};

use super::super::{
    access::scan_observation::scan_error, PhysicalRecordId, PhysicalRecordReader, RecordScanDenial,
    RecordScanError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalRecordScanCursor {
    store: [u8; 16],
    root_generation: u64,
    tree_identity: u64,
    format_identity: [u8; 10],
    last_record: PhysicalRecordId,
}

impl ExternalRecordScanCursor {
    pub fn encode(self) -> [u8; 66] {
        let mut bytes = [0_u8; 66];
        bytes[..16].copy_from_slice(&self.store);
        bytes[16..24].copy_from_slice(&self.root_generation.to_le_bytes());
        bytes[24..32].copy_from_slice(&self.tree_identity.to_le_bytes());
        bytes[32..42].copy_from_slice(&self.format_identity);
        bytes[42..58].copy_from_slice(&self.last_record.allocation_epoch());
        bytes[58..66].copy_from_slice(&self.last_record.ordinal().to_le_bytes());
        bytes
    }

    pub fn decode(bytes: [u8; 66]) -> Option<Self> {
        let record = PersistedRecordIdentity::new(
            bytes[42..58].try_into().ok()?,
            u64::from_le_bytes(bytes[58..66].try_into().ok()?),
        )?;
        let store = bytes[..16].try_into().ok()?;
        let root_generation = u64::from_le_bytes(bytes[16..24].try_into().ok()?);
        let tree_identity = u64::from_le_bytes(bytes[24..32].try_into().ok()?);
        let format_identity = bytes[32..42].try_into().ok()?;
        (store != [0; 16] && root_generation != 0 && tree_identity != 0).then_some(Self {
            store,
            root_generation,
            tree_identity,
            format_identity,
            last_record: PhysicalRecordId::from_persisted(record),
        })
    }
}

pub(in crate::physical_runtime::record_serving) fn readmit_cursor(
    reader: &PhysicalRecordReader<'_>,
    cursor: Option<ExternalRecordScanCursor>,
) -> Result<Option<PersistedRecordIdentity>, RecordScanError> {
    let Some(cursor) = cursor else {
        return Ok(None);
    };
    if cursor.store != reader.store_identity().bytes() {
        return Err(scan_error(RecordScanDenial::ForeignStore));
    }
    if cursor.root_generation != reader.current_root.generation() {
        return Err(scan_error(RecordScanDenial::StaleRoot));
    }
    if cursor.tree_identity != reader.current_root.tree_identity() {
        return Err(scan_error(RecordScanDenial::RoutingTreeMismatch));
    }
    if cursor.format_identity != reader.format.declaration().canonical_identity_bytes() {
        return Err(scan_error(RecordScanDenial::FormatMismatch));
    }
    Ok(Some(cursor.last_record.persisted()))
}

pub(in crate::physical_runtime::record_serving) fn cursor_for(
    reader: &PhysicalRecordReader<'_>,
    root: &DurablePhysicalRootManifest,
    record: PhysicalRecordId,
) -> ExternalRecordScanCursor {
    ExternalRecordScanCursor {
        store: reader.store_identity().bytes(),
        root_generation: root.generation(),
        tree_identity: root.tree_identity(),
        format_identity: reader.format.declaration().canonical_identity_bytes(),
        last_record: record,
    }
}
