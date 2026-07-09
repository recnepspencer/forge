use crate::failure::{StoreError, StoreErrorKind};
use serde::Serialize;
use sha2::{Digest, Sha256};

const FRAME_MAGIC: [u8; 8] = *b"FGSTRMF1";
const FRAME_TERMINATOR: [u8; 8] = *b"FGSEND01";
const HEADER_LEN: usize = 8 + 1 + 4 + 8 + 32;
const TERMINATOR_LEN: usize = 8;

pub(crate) const CURRENT_DURABLE_MEDIA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DurableMediaFamily {
    WalRecord = 1,
}

impl DurableMediaFamily {
    fn from_u8(value: u8) -> Result<Self, StoreError> {
        match value {
            1 => Ok(Self::WalRecord),
            _ => Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!("unknown durable media family tag {value}"),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RawDurableBytes {
    bytes: Vec<u8>,
}

impl RawDurableBytes {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FramedDurableRecord {
    bytes: Vec<u8>,
    family: DurableMediaFamily,
    version: u32,
    payload_len: usize,
}

impl FramedDurableRecord {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn to_raw_bytes(&self) -> RawDurableBytes {
        RawDurableBytes::new(self.bytes.clone())
    }

    pub(crate) fn payload_len(&self) -> usize {
        self.payload_len
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IntegrityValidatedDurableRecord {
    framed: FramedDurableRecord,
    payload_bytes: Vec<u8>,
}

impl IntegrityValidatedDurableRecord {
    pub(crate) fn family(&self) -> DurableMediaFamily {
        self.framed.family
    }

    pub(crate) fn version(&self) -> u32 {
        self.framed.version
    }

    pub(crate) fn payload_bytes(&self) -> &[u8] {
        &self.payload_bytes
    }

    pub(crate) fn framed_record(&self) -> &FramedDurableRecord {
        &self.framed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TailValidationOutcome {
    Clean,
    TruncatedTail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TailValidationReport {
    outcome: TailValidationOutcome,
    valid_record_count: usize,
    trailing_byte_count: usize,
}

impl TailValidationReport {
    pub(crate) fn outcome(&self) -> TailValidationOutcome {
        self.outcome
    }

    pub(crate) fn valid_record_count(&self) -> usize {
        self.valid_record_count
    }

    #[cfg(test)]
    pub(crate) fn trailing_byte_count(&self) -> usize {
        self.trailing_byte_count
    }
}

pub(crate) fn frame_payload<T: Serialize>(
    family: DurableMediaFamily,
    payload: &T,
) -> Result<FramedDurableRecord, StoreError> {
    let payload_bytes = serde_json::to_vec(payload)?;
    let digest = digest_bytes(&payload_bytes);
    let payload_len = payload_bytes.len();

    let mut bytes = Vec::with_capacity(HEADER_LEN + payload_len + TERMINATOR_LEN);
    bytes.extend_from_slice(&FRAME_MAGIC);
    bytes.push(family as u8);
    bytes.extend_from_slice(&CURRENT_DURABLE_MEDIA_VERSION.to_le_bytes());
    bytes.extend_from_slice(&(payload_len as u64).to_le_bytes());
    bytes.extend_from_slice(&digest);
    bytes.extend_from_slice(&payload_bytes);
    bytes.extend_from_slice(&FRAME_TERMINATOR);

    Ok(FramedDurableRecord {
        bytes,
        family,
        version: CURRENT_DURABLE_MEDIA_VERSION,
        payload_len,
    })
}

pub(crate) fn validate_raw_record(
    raw: RawDurableBytes,
) -> Result<IntegrityValidatedDurableRecord, StoreError> {
    let framed = parse_framed_record(raw.as_bytes())?;
    let payload_start = HEADER_LEN;
    let payload_end = payload_start + framed.payload_len;
    let payload_bytes = framed.bytes[payload_start..payload_end].to_vec();
    Ok(IntegrityValidatedDurableRecord {
        framed,
        payload_bytes,
    })
}

pub(crate) fn scan_tail(bytes: &[u8]) -> Result<TailValidationReport, StoreError> {
    let mut offset = 0_usize;
    let mut valid_record_count = 0_usize;

    while offset < bytes.len() {
        let remaining = bytes.len() - offset;
        if remaining < HEADER_LEN + TERMINATOR_LEN {
            return Ok(TailValidationReport {
                outcome: TailValidationOutcome::TruncatedTail,
                valid_record_count,
                trailing_byte_count: remaining,
            });
        }

        if bytes[offset..offset + 8] != FRAME_MAGIC {
            return Err(StoreError::new(
                StoreErrorKind::DurableRecordFramingInvalid,
                format!("invalid durable frame magic at offset {offset}"),
            ));
        }

        let payload_len = u64::from_le_bytes(
            bytes[offset + 13..offset + 21]
                .try_into()
                .expect("slice length"),
        ) as usize;
        let total_len = HEADER_LEN + payload_len + TERMINATOR_LEN;
        if remaining < total_len {
            return Ok(TailValidationReport {
                outcome: TailValidationOutcome::TruncatedTail,
                valid_record_count,
                trailing_byte_count: remaining,
            });
        }

        let raw = RawDurableBytes::new(bytes[offset..offset + total_len].to_vec());
        validate_raw_record(raw)?;
        valid_record_count += 1;
        offset += total_len;
    }

    Ok(TailValidationReport {
        outcome: TailValidationOutcome::Clean,
        valid_record_count,
        trailing_byte_count: 0,
    })
}

fn parse_framed_record(bytes: &[u8]) -> Result<FramedDurableRecord, StoreError> {
    if bytes.len() < HEADER_LEN + TERMINATOR_LEN {
        return Err(StoreError::new(
            StoreErrorKind::DurableTailTruncated,
            format!(
                "durable record shorter than minimum frame size: {} bytes",
                bytes.len()
            ),
        ));
    }
    if bytes[0..8] != FRAME_MAGIC {
        return Err(StoreError::new(
            StoreErrorKind::DurableRecordFramingInvalid,
            "durable record missing expected frame magic",
        ));
    }

    let family = DurableMediaFamily::from_u8(bytes[8])?;
    let version = u32::from_le_bytes(bytes[9..13].try_into().expect("slice length"));
    if version != CURRENT_DURABLE_MEDIA_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::DurableFamilyVersionUnsupported,
            format!("unsupported durable media version {version}"),
        ));
    }

    let payload_len = u64::from_le_bytes(bytes[13..21].try_into().expect("slice length")) as usize;
    let total_len = HEADER_LEN + payload_len + TERMINATOR_LEN;
    if bytes.len() != total_len {
        return Err(StoreError::new(
            if bytes.len() < total_len {
                StoreErrorKind::DurableTailTruncated
            } else {
                StoreErrorKind::DurableRecordFramingInvalid
            },
            format!(
                "durable record length {} does not match framed total length {}",
                bytes.len(),
                total_len
            ),
        ));
    }

    let payload_start = HEADER_LEN;
    let payload_end = payload_start + payload_len;
    let payload_bytes = &bytes[payload_start..payload_end];
    let expected_digest: [u8; 32] = bytes[21..53].try_into().expect("slice length");
    let actual_digest = digest_bytes(payload_bytes);
    if actual_digest != expected_digest {
        return Err(StoreError::new(
            StoreErrorKind::DurableTornWriteDetected,
            "durable frame payload digest mismatch",
        ));
    }

    if bytes[payload_end..payload_end + TERMINATOR_LEN] != FRAME_TERMINATOR {
        return Err(StoreError::new(
            StoreErrorKind::DurableRecordFramingInvalid,
            "durable frame missing terminal marker",
        ));
    }

    Ok(FramedDurableRecord {
        bytes: bytes.to_vec(),
        family,
        version,
        payload_len,
    })
}

fn digest_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}
