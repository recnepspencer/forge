use std::path::Path;

use worth_store_physical_format::PersistedRecordIdentity;

const MAGIC: &[u8] = b"WORTH-C8-ISSUED-FATES-V2\n";

pub(super) struct IdentityReceipt {
    pub(super) material: [u8; 32],
    pub(super) idempotency: [u8; 32],
    pub(super) fate: u8,
    pub(super) record: Option<PersistedRecordIdentity>,
}

pub(super) fn write(path: &Path, receipts: &[IdentityReceipt]) -> Result<(), String> {
    let count = u32::try_from(receipts.len())
        .map_err(|_| "C8 identity receipt has too many issued operations".to_owned())?;
    let mut encoded = Vec::with_capacity(MAGIC.len() + 4 + receipts.len() * 90);
    encoded.extend_from_slice(MAGIC);
    encoded.extend_from_slice(&count.to_le_bytes());
    for receipt in receipts {
        encoded.extend_from_slice(&receipt.material);
        encoded.extend_from_slice(&receipt.idempotency);
        encoded.push(receipt.fate);
        match receipt.record {
            Some(record) => {
                encoded.push(1);
                encoded.extend_from_slice(&record.allocation_epoch());
                encoded.extend_from_slice(&record.ordinal().to_le_bytes());
            }
            None => encoded.push(0),
        }
    }
    std::fs::write(path, encoded).map_err(|error| format!("write C8 issued fate receipt: {error}"))
}
