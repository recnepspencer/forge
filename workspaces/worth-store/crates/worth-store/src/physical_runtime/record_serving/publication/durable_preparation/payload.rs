use sha2::{Digest, Sha256};

use super::super::{
    batch::{RecordAppendBatch, RecordAppendInput},
    streaming::{RecordStreamFailure, RecordStreamFailureKind},
};

const PAYLOAD_DOMAIN: &[u8] = b"store.physical.record-append.payload.v1";

pub(in crate::physical_runtime::record_serving) struct CanonicalRecordAppendPayload {
    pub(in crate::physical_runtime::record_serving) batch: RecordAppendBatch,
    pub(in crate::physical_runtime::record_serving) digest: [u8; 32],
    pub(in crate::physical_runtime::record_serving) record_count: u32,
    pub(in crate::physical_runtime::record_serving) payload_bytes: u64,
}

pub(in crate::physical_runtime::record_serving) enum CanonicalPayloadPreparationError {
    RecordSlots { required_records: u32 },
    PayloadBytes { required_bytes: u64 },
    Failed(RecordStreamFailure),
}

pub(in crate::physical_runtime::record_serving) fn prepare_canonical_payload(
    batch: RecordAppendBatch,
) -> Result<CanonicalRecordAppendPayload, CanonicalPayloadPreparationError> {
    let record_count = batch.records.len() as u32;
    let mut digest = Sha256::new();
    write_field(&mut digest, PAYLOAD_DOMAIN);
    write_field(&mut digest, &record_count.to_le_bytes());
    let mut payload_bytes = 0_u64;
    let mut records = Vec::new();
    records
        .try_reserve_exact(batch.records.len())
        .map_err(|_| CanonicalPayloadPreparationError::RecordSlots {
            required_records: record_count,
        })?;
    for input in batch.records {
        let bytes = materialize(input, payload_bytes)?;
        payload_bytes = payload_bytes.saturating_add(bytes.len() as u64);
        write_field(&mut digest, &(bytes.len() as u64).to_le_bytes());
        write_field(&mut digest, &bytes);
        records.push(RecordAppendInput::Bytes(bytes));
    }
    Ok(CanonicalRecordAppendPayload {
        batch: RecordAppendBatch { records },
        digest: digest.finalize().into(),
        record_count,
        payload_bytes,
    })
}

fn materialize(
    input: RecordAppendInput,
    completed_bytes: u64,
) -> Result<Vec<u8>, CanonicalPayloadPreparationError> {
    match input {
        RecordAppendInput::Bytes(bytes) => Ok(bytes),
        RecordAppendInput::Source {
            source,
            declared_length,
        } => materialize_source(source, declared_length, completed_bytes),
    }
}

fn materialize_source(
    mut source: Box<dyn super::super::streaming::RecordWriteSource>,
    declared_length: u64,
    completed_bytes: u64,
) -> Result<Vec<u8>, CanonicalPayloadPreparationError> {
    let length = usize::try_from(declared_length).map_err(|_| {
        CanonicalPayloadPreparationError::PayloadBytes {
            required_bytes: declared_length,
        }
    })?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(length).map_err(|_| {
        CanonicalPayloadPreparationError::PayloadBytes {
            required_bytes: declared_length,
        }
    })?;
    bytes.resize(length, 0);
    let completed = read_declared_bytes(&mut *source, &mut bytes, completed_bytes)?;
    reject_excess_bytes(&mut *source, completed_bytes, completed)?;
    Ok(bytes)
}

fn read_declared_bytes(
    source: &mut dyn super::super::streaming::RecordWriteSource,
    bytes: &mut [u8],
    completed_bytes: u64,
) -> Result<usize, CanonicalPayloadPreparationError> {
    let mut offset = 0_usize;
    while offset < bytes.len() {
        let count = source.read_next(&mut bytes[offset..]).map_err(|_| {
            stream_failure(
                RecordStreamFailureKind::ProducerRejected,
                completed_bytes,
                offset,
            )
        })?;
        if count == 0 {
            return Err(stream_failure(
                RecordStreamFailureKind::SourceEndedEarly,
                completed_bytes,
                offset,
            ));
        }
        if count > bytes.len() - offset {
            return Err(stream_failure(
                RecordStreamFailureKind::InvalidTransferCount,
                completed_bytes,
                offset,
            ));
        }
        offset += count;
    }
    Ok(offset)
}

fn reject_excess_bytes(
    source: &mut dyn super::super::streaming::RecordWriteSource,
    completed_bytes: u64,
    current_record_bytes: usize,
) -> Result<(), CanonicalPayloadPreparationError> {
    let mut excess = [0_u8; 1];
    let count = source.read_next(&mut excess).map_err(|_| {
        stream_failure(
            RecordStreamFailureKind::ProducerRejected,
            completed_bytes,
            current_record_bytes,
        )
    })?;
    let kind = if count > excess.len() {
        Some(RecordStreamFailureKind::InvalidTransferCount)
    } else if count != 0 {
        Some(RecordStreamFailureKind::SourceExceededDeclaredLength)
    } else {
        None
    };
    match kind {
        Some(kind) => Err(stream_failure(kind, completed_bytes, current_record_bytes)),
        None => Ok(()),
    }
}

fn stream_failure(
    kind: RecordStreamFailureKind,
    completed_bytes: u64,
    current_record_bytes: usize,
) -> CanonicalPayloadPreparationError {
    CanonicalPayloadPreparationError::Failed(RecordStreamFailure::during_read(
        kind,
        completed_bytes.saturating_add(current_record_bytes as u64),
    ))
}

fn write_field(digest: &mut Sha256, bytes: &[u8]) {
    digest.update((bytes.len() as u64).to_le_bytes());
    digest.update(bytes);
}
