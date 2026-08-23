use sha2::{Digest, Sha256};
use worth_store_physical_format::PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN;

const CHECKPOINT_MAGIC: &[u8] = b"WCP7REC\0";
const CHECKPOINT_PREFIX_BYTES: usize = 16;
const CHECKPOINT_CRC_BYTES: usize = 4;
const TERMINAL_STATE: u8 = 3;
const PROVEN_NO_EFFECT_CLASS: u8 = 1;
const KEY_DOMAIN: &[u8] = b"store.physical.mutation.idempotency-key.v1";

/// Returns a no-effect conclusion only when a verified checkpoint contains a
/// canonical-shaped terminal binding with an explicit persisted no-effect
/// fate. Identity bytes in a WAL, receipt, or arbitrary checkpoint payload are
/// not terminal evidence.
pub(crate) fn contains_persisted_no_effect_terminal(
    files: &[(String, Vec<u8>)],
    identity: &[u8],
) -> Result<bool, String> {
    for (path, bytes) in files {
        if !bytes.starts_with(CHECKPOINT_MAGIC) {
            continue;
        }
        for record in binding_records(bytes, path)? {
            if is_proven_no_effect_terminal(record, identity) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn binding_records<'bytes>(bytes: &'bytes [u8], path: &str) -> Result<Vec<&'bytes [u8]>, String> {
    let mut offset = 0;
    let mut records = Vec::new();
    while offset < bytes.len() {
        let prefix = bytes
            .get(offset..offset + CHECKPOINT_PREFIX_BYTES)
            .ok_or_else(|| {
                format!("independent terminal oracle found a truncated prefix: {path}")
            })?;
        if &prefix[..8] != CHECKPOINT_MAGIC || prefix[8] != 1 {
            return Err(format!(
                "independent terminal oracle found an invalid record: {path}"
            ));
        }
        let payload_bytes = usize::try_from(read_u32(prefix, 12)?)
            .map_err(|_| format!("independent terminal oracle payload is too large: {path}"))?;
        let total = CHECKPOINT_PREFIX_BYTES
            .checked_add(payload_bytes)
            .and_then(|length| length.checked_add(CHECKPOINT_CRC_BYTES))
            .ok_or_else(|| format!("independent terminal oracle record overflowed: {path}"))?;
        let record = bytes.get(offset..offset + total).ok_or_else(|| {
            format!("independent terminal oracle found a truncated record: {path}")
        })?;
        let checksum_offset = CHECKPOINT_PREFIX_BYTES + payload_bytes;
        if crc32c(&record[..checksum_offset]) != read_u32(&record[checksum_offset..], 0)? {
            return Err(format!(
                "independent terminal oracle rejected a checksum: {path}"
            ));
        }
        if prefix[9] == 4 {
            records.push(&record[CHECKPOINT_PREFIX_BYTES..checksum_offset]);
        }
        offset += total;
    }
    Ok(records)
}

fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    bytes
        .get(offset..offset + 4)
        .and_then(|value| value.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| "independent terminal oracle found a truncated integer".to_owned())
}

fn crc32c(bytes: &[u8]) -> u32 {
    let mut value = !0_u32;
    for byte in bytes {
        value ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(value & 1);
            value = (value >> 1) ^ (0x82f6_3b78 & mask);
        }
    }
    !value
}

fn is_proven_no_effect_terminal(bytes: &[u8], identity: &[u8]) -> bool {
    let mut cursor = FieldCursor::new(bytes);
    if cursor.field() != Some(PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN)
        || cursor.byte() != Some(TERMINAL_STATE)
    {
        return false;
    }
    if cursor.field() != Some(identity) {
        return false;
    }
    let Some(store) = cursor.array_field(16) else {
        return false;
    };
    let Some(policy) = cursor.array_field(32) else {
        return false;
    };
    let Some(issuance) = cursor.u64() else {
        return false;
    };
    let Some(expiry) = cursor.u64() else {
        return false;
    };
    let Some(material) = cursor.array_field(32) else {
        return false;
    };
    let Some(fingerprint) = cursor.array_field(32) else {
        return false;
    };
    let Some(mutation_store) = cursor.array_field(16) else {
        return false;
    };
    let Some(runtime) = cursor.u64() else {
        return false;
    };
    let Some(lifecycle) = cursor.u64() else {
        return false;
    };
    let Some(operation) = cursor.u64() else {
        return false;
    };
    let mut key = Sha256::new();
    key.update((KEY_DOMAIN.len() as u64).to_le_bytes());
    key.update(KEY_DOMAIN);
    key.update(store);
    key.update(policy);
    key.update(issuance.to_le_bytes());
    key.update(expiry.to_le_bytes());
    key.update(material);
    let key_identity: [u8; 32] = key.finalize().into();
    if key_identity.as_slice() != identity
        || store == [0; 16]
        || policy == [0; 32]
        || material == [0; 32]
        || fingerprint == [0; 32]
        || mutation_store != store
        || issuance >= expiry
        || runtime == 0
        || lifecycle == 0
        || operation == 0
    {
        return false;
    }
    cursor.byte() == Some(PROVEN_NO_EFFECT_CLASS)
        && matches!(cursor.byte(), Some(1..=4))
        && cursor.is_empty()
}

struct FieldCursor<'bytes> {
    remaining: &'bytes [u8],
}

impl<'bytes> FieldCursor<'bytes> {
    const fn new(bytes: &'bytes [u8]) -> Self {
        Self { remaining: bytes }
    }

    fn field(&mut self) -> Option<&'bytes [u8]> {
        let length = usize::try_from(self.u64()?).ok()?;
        let (field, remaining) = self.remaining.split_at_checked(length)?;
        self.remaining = remaining;
        Some(field)
    }

    fn array_field(&mut self, expected: usize) -> Option<&'bytes [u8]> {
        let field = self.field()?;
        (field.len() == expected).then_some(field)
    }

    fn u64(&mut self) -> Option<u64> {
        let (value, remaining) = self.remaining.split_at_checked(8)?;
        self.remaining = remaining;
        Some(u64::from_le_bytes(value.try_into().ok()?))
    }

    fn byte(&mut self) -> Option<u8> {
        let (value, remaining) = self.remaining.split_at_checked(1)?;
        self.remaining = remaining;
        value.first().copied()
    }

    const fn is_empty(&self) -> bool {
        self.remaining.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use super::is_proven_no_effect_terminal;

    #[test]
    fn terminal_oracle_requires_the_full_persisted_fate_shape() {
        let identity = key_identity();
        assert!(!is_proven_no_effect_terminal(&identity, &identity));

        let valid = terminal_record(&identity, 1, 1);
        assert!(is_proven_no_effect_terminal(&valid, &identity));

        let mut invalid_class = valid.clone();
        let class_index = invalid_class.len() - 2;
        invalid_class[class_index] = 2;
        assert!(!is_proven_no_effect_terminal(&invalid_class, &identity));

        let mut invalid_cause = valid;
        let last = invalid_cause.len() - 1;
        invalid_cause[last] = 9;
        assert!(!is_proven_no_effect_terminal(&invalid_cause, &identity));
    }

    fn terminal_record(identity: &[u8; 32], class: u8, cause: u8) -> Vec<u8> {
        let mut bytes = Vec::new();
        field(
            &mut bytes,
            super::PHYSICAL_MUTATION_BINDING_COMPACTION_RECORD_DOMAIN,
        );
        bytes.push(super::TERMINAL_STATE);
        field(&mut bytes, identity);
        field(&mut bytes, &[1; 16]);
        field(&mut bytes, &[2; 32]);
        bytes.extend_from_slice(&1_u64.to_le_bytes());
        bytes.extend_from_slice(&2_u64.to_le_bytes());
        field(&mut bytes, &[3; 32]);
        field(&mut bytes, &[4; 32]);
        field(&mut bytes, &[1; 16]);
        bytes.extend_from_slice(&6_u64.to_le_bytes());
        bytes.extend_from_slice(&7_u64.to_le_bytes());
        bytes.extend_from_slice(&8_u64.to_le_bytes());
        bytes.push(class);
        bytes.push(cause);
        bytes
    }

    fn key_identity() -> [u8; 32] {
        let mut key = Sha256::new();
        key.update((super::KEY_DOMAIN.len() as u64).to_le_bytes());
        key.update(super::KEY_DOMAIN);
        key.update([1; 16]);
        key.update([2; 32]);
        key.update(1_u64.to_le_bytes());
        key.update(2_u64.to_le_bytes());
        key.update([3; 32]);
        key.finalize().into()
    }

    fn field(target: &mut Vec<u8>, value: &[u8]) {
        target.extend_from_slice(&(value.len() as u64).to_le_bytes());
        target.extend_from_slice(value);
    }
}
