use crate::binary_encoding::BinaryEncodingMeasure;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RecordPayloadEncodingWork {
    payload_bytes: u64,
    nested_entries: u64,
}

impl RecordPayloadEncodingWork {
    pub(super) fn from_measure(
        measure: &BinaryEncodingMeasure,
        limits: WorthQueryPackageArchiveLimits,
    ) -> Result<Self, Denial> {
        let work = Self {
            payload_bytes: measure.bytes(),
            nested_entries: measure.nested_entries(),
        };
        if work.nested_entries > limits.narrowed().maximum_nested_entries() {
            return Err(Denial::new(Kind::NestedEntryBudgetExceeded));
        }
        Ok(work)
    }

    pub(super) const fn without_nested_entries(payload_bytes: u64) -> Self {
        Self {
            payload_bytes,
            nested_entries: 0,
        }
    }

    pub(super) const fn payload_bytes(self) -> u64 {
        self.payload_bytes
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct RecordEncodingWork {
    record_frames: u32,
    logical_bytes: u64,
    nested_entries: u64,
}

impl RecordEncodingWork {
    pub(super) fn admit(
        self,
        payload: RecordPayloadEncodingWork,
        limits: WorthQueryPackageArchiveLimits,
    ) -> Result<Self, Denial> {
        let limits = limits.narrowed();
        let record_frames = self
            .record_frames
            .checked_add(1)
            .ok_or_else(|| Denial::new(Kind::RecordBudgetExceeded))?;
        if record_frames > limits.maximum_records() {
            return Err(Denial::new(Kind::RecordBudgetExceeded));
        }
        let logical_bytes = self
            .logical_bytes
            .checked_add(payload.payload_bytes)
            .ok_or_else(|| Denial::new(Kind::LogicalByteBudgetExceeded))?;
        if logical_bytes > limits.maximum_logical_bytes() {
            return Err(Denial::new(Kind::LogicalByteBudgetExceeded));
        }
        let nested_entries = self
            .nested_entries
            .checked_add(payload.nested_entries)
            .ok_or_else(|| Denial::new(Kind::NestedEntryBudgetExceeded))?;
        if nested_entries > limits.maximum_nested_entries() {
            return Err(Denial::new(Kind::NestedEntryBudgetExceeded));
        }
        Ok(Self {
            record_frames,
            logical_bytes,
            nested_entries,
        })
    }

    pub(crate) const fn record_frames(self) -> u32 {
        self.record_frames
    }
}
