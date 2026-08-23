use std::collections::BTreeSet;
use std::path::Path;

use super::super::super::history_io;

const IDENTITY_RECEIPT_MAGIC: &[u8] = b"WORTH-C8-ISSUED-FATES-V2\n";

#[derive(Debug, Clone)]
pub(super) struct IdentityReceiptEntry {
    pub(super) material: [u8; 32],
    pub(super) idempotency: [u8; 32],
    pub(super) fate: u8,
    pub(super) record: Option<([u8; 16], u64)>,
}

#[derive(Debug, Clone)]
pub(super) struct DecodedIdentityReceipt {
    pub(super) entries: Vec<IdentityReceiptEntry>,
}

pub(super) fn decode(path: &Path, expected_count: usize) -> Result<DecodedIdentityReceipt, String> {
    let encoded = std::fs::read(path)
        .map_err(|error| format!("read C8 identity receipt {path:?}: {error}"))?;
    if !encoded.starts_with(IDENTITY_RECEIPT_MAGIC) {
        return Err("C8 identity receipt has an unknown protocol magic".to_owned());
    }
    let mut cursor = IDENTITY_RECEIPT_MAGIC.len();
    let count = history_io::read_u32(&encoded, &mut cursor)? as usize;
    if count != expected_count {
        return Err(format!(
            "C8 identity receipt count {count} does not match expected operation count {expected_count}"
        ));
    }
    let mut seen_idempotency = BTreeSet::new();
    let mut entries = Vec::with_capacity(count);
    for ordinal in 0..count {
        let material = history_io::read_array::<32>(&encoded, &mut cursor)?;
        let idempotency = history_io::read_array::<32>(&encoded, &mut cursor)?;
        let fate = history_io::read_byte(&encoded, &mut cursor)?;
        if idempotency == [0; 32] || !seen_idempotency.insert(idempotency) {
            return Err(format!(
                "C8 identity receipt has a duplicate or empty idempotency identity at {ordinal}"
            ));
        }
        let has_record = history_io::read_byte(&encoded, &mut cursor)?;
        let record = match has_record {
            1 => Some((
                history_io::read_array::<16>(&encoded, &mut cursor)?,
                history_io::read_u64(&encoded, &mut cursor)?,
            )),
            0 => None,
            _ => {
                return Err(format!(
                    "C8 identity receipt has an invalid record flag at {ordinal}"
                ));
            }
        };
        entries.push(IdentityReceiptEntry {
            material,
            idempotency,
            fate,
            record,
        });
    }
    if cursor != encoded.len() {
        return Err("C8 identity receipt has trailing bytes".to_owned());
    }
    Ok(DecodedIdentityReceipt { entries })
}
