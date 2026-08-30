use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;

/// Structural work consumed while decoding untrusted record frames.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryPackageArchiveDecodeWork {
    record_frames: u32,
    logical_bytes: u64,
    nested_entries: u64,
}

impl WorthQueryPackageArchiveDecodeWork {
    pub const fn record_frames(self) -> u32 {
        self.record_frames
    }

    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }

    pub const fn nested_entries(self) -> u64 {
        self.nested_entries
    }
}

pub(super) struct RecordDecodeAttempt {
    work: WorthQueryPackageArchiveDecodeWork,
    limits: WorthQueryPackageArchiveLimits,
}

impl RecordDecodeAttempt {
    pub(super) fn begin(
        prior: WorthQueryPackageArchiveDecodeWork,
        payload_bytes: u64,
        limits: WorthQueryPackageArchiveLimits,
    ) -> Result<Self, Denial> {
        let limits = limits.narrowed();
        let record_frames = prior
            .record_frames
            .checked_add(1)
            .ok_or_else(|| Denial::new(Kind::RecordBudgetExceeded))?;
        if record_frames > limits.maximum_records() {
            return Err(Denial::new(Kind::RecordBudgetExceeded));
        }
        let logical_bytes = prior
            .logical_bytes
            .checked_add(payload_bytes)
            .ok_or_else(|| Denial::new(Kind::LogicalByteBudgetExceeded))?;
        if logical_bytes > limits.maximum_logical_bytes() {
            return Err(Denial::new(Kind::LogicalByteBudgetExceeded));
        }
        Ok(Self {
            work: WorthQueryPackageArchiveDecodeWork {
                record_frames,
                logical_bytes,
                nested_entries: prior.nested_entries,
            },
            limits,
        })
    }

    pub(super) fn claim_nested_entries(&mut self, count: u64) -> Result<(), Denial> {
        let nested_entries = self
            .work
            .nested_entries
            .checked_add(count)
            .ok_or_else(|| Denial::new(Kind::NestedEntryBudgetExceeded))?;
        if nested_entries > self.limits.maximum_nested_entries() {
            return Err(Denial::new(Kind::NestedEntryBudgetExceeded));
        }
        self.work.nested_entries = nested_entries;
        Ok(())
    }

    pub(super) fn require_nesting_depth(&self, depth: u32) -> Result<(), Denial> {
        if depth > self.limits.maximum_nesting_depth() {
            return Err(Denial::new(Kind::NestingDepthBudgetExceeded));
        }
        Ok(())
    }

    pub(super) const fn finish(self) -> WorthQueryPackageArchiveDecodeWork {
        self.work
    }
}
