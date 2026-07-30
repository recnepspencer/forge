use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, RecordReadObservation, ServingPhysicalRuntime,
};

use crate::bounded_residency::{read_pressure::read_limits, workload};

use super::super::super::configuration::{BoundedResidencyConfiguration, STREAMING_SCRATCH_BYTES};

#[derive(Clone, Copy)]
pub(super) struct VerifiedRecordCopy {
    pub(super) observation: RecordReadObservation,
    pub(super) operations: u64,
    pub(super) bytes: u64,
    pub(super) maximum_width: u64,
}

#[derive(Default)]
pub(super) struct CopyTotals {
    operations: u64,
    bytes: u64,
    maximum_width: u64,
}

impl CopyTotals {
    pub(super) fn observe(&mut self, read: VerifiedRecordCopy) {
        self.operations = self.operations.saturating_add(read.operations);
        self.bytes = self.bytes.saturating_add(read.bytes);
        self.maximum_width = self.maximum_width.max(read.maximum_width);
    }

    pub(super) const fn operations(&self) -> u64 {
        self.operations
    }

    pub(super) const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub(super) const fn maximum_width(&self) -> u64 {
        self.maximum_width
    }
}

pub(super) fn read(
    serving: &ServingPhysicalRuntime,
    locator: ExternalPhysicalRecordLocator,
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
) -> Result<VerifiedRecordCopy, String> {
    let expected_bytes = configuration
        .record_bytes(ordinal)
        .ok_or_else(|| "C.6 record ordinal is out of range".to_owned())?;
    let mut session = serving
        .records()
        .open_external(locator, read_limits(configuration, ordinal)?)
        .map_err(|failure| format!("C.6 record open failed: {failure:?}"))?;
    let mut scratch = [0_u8; STREAMING_SCRATCH_BYTES];
    let mut offset = 0_usize;
    let mut operations = 0_u64;
    let mut maximum_width = 0_u64;
    while offset < expected_bytes {
        let count = session
            .read_next(&mut scratch)
            .map_err(|failure| format!("C.6 record read failed: {failure:?}"))?;
        if count == 0 {
            return Err("C.6 record ended before its declared payload".to_owned());
        }
        workload::verify_record_range(configuration, ordinal, offset, &scratch[..count])?;
        operations = operations.saturating_add(1);
        maximum_width = maximum_width.max(count as u64);
        offset += count;
    }
    let observation = session.observation();
    if offset != expected_bytes
        || observation.explicit_copy_count() != operations
        || observation.copied_bytes() != offset as u64
        || observation.peak_transfer_width() > configuration.resident_bytes()
    {
        return Err("C.6 record copy observation diverged from caller truth".to_owned());
    }
    Ok(VerifiedRecordCopy {
        observation,
        operations,
        bytes: offset as u64,
        maximum_width,
    })
}

pub(super) fn largest_record_bytes(configuration: BoundedResidencyConfiguration) -> u64 {
    (0..configuration.record_count())
        .filter_map(|ordinal| configuration.record_bytes(ordinal))
        .max()
        .unwrap_or(0) as u64
}
