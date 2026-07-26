use std::num::{NonZeroU32, NonZeroU64};

use worth_proof::TransitionOutcome;

use super::{
    AdmittedPhysicalRecordResidencyPolicy, PhysicalOperationAllocationScope,
    PhysicalRecordResidencyPolicyOutcome, PhysicalSpeculativeWorkKind,
};
use crate::physical_runtime::record_serving::AdmittedPhysicalRecordFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordResidencyPolicy;

impl PhysicalRecordResidencyPolicy {
    pub fn builder() -> PhysicalRecordResidencyPolicyBuilder {
        PhysicalRecordResidencyPolicyBuilder {
            declaration: worth_store_buffer_pool::PhysicalResidencyLimits::builder(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordResidencyPolicyBuilder {
    declaration: worth_store_buffer_pool::PhysicalResidencyLimitsBuilder,
}

impl PhysicalRecordResidencyPolicyBuilder {
    pub const fn total_bytes(mut self, bytes: NonZeroU64) -> Self {
        self.declaration = self.declaration.total_bytes(bytes);
        self
    }

    pub const fn resident_bytes(mut self, bytes: NonZeroU64) -> Self {
        self.declaration = self.declaration.resident_bytes(bytes);
        self
    }

    pub const fn metadata_bytes(mut self, bytes: NonZeroU64) -> Self {
        self.declaration = self.declaration.metadata_bytes(bytes);
        self
    }

    pub const fn frame_entries(mut self, entries: NonZeroU32) -> Self {
        self.declaration = self.declaration.frame_entries(entries);
        self
    }

    pub const fn pinned_frames(mut self, frames: NonZeroU32) -> Self {
        self.declaration = self.declaration.pinned_frames(frames);
        self
    }

    pub const fn pin_leases(mut self, leases: NonZeroU32) -> Self {
        self.declaration = self.declaration.pin_leases(leases);
        self
    }

    pub const fn dirty_frames(mut self, frames: NonZeroU32) -> Self {
        self.declaration = self.declaration.dirty_frames(frames);
        self
    }

    pub const fn dirty_replacement_bytes(mut self, bytes: NonZeroU64) -> Self {
        self.declaration = self.declaration.dirty_replacement_bytes(bytes);
        self
    }

    pub const fn operation_bytes(mut self, bytes: NonZeroU64) -> Self {
        self.declaration = self.declaration.operation_bytes(bytes);
        self
    }

    pub const fn scope_bytes(
        mut self,
        scope: PhysicalOperationAllocationScope,
        bytes: NonZeroU64,
    ) -> Self {
        self.declaration = self.declaration.scope_bytes(scope, bytes);
        self
    }

    pub const fn speculative_frames(
        mut self,
        kind: PhysicalSpeculativeWorkKind,
        frames: NonZeroU32,
    ) -> Self {
        self.declaration = self.declaration.speculative_frames(kind, frames);
        self
    }

    pub fn admit(
        self,
        format: AdmittedPhysicalRecordFormat,
    ) -> PhysicalRecordResidencyPolicyOutcome {
        let page_bytes = NonZeroU64::new(u64::from(format.declaration().page_size().bytes()))
            .expect("an admitted physical format has a nonzero page size");
        match self.declaration.admit(page_bytes) {
            Ok(limits) => {
                TransitionOutcome::success(AdmittedPhysicalRecordResidencyPolicy { limits })
            }
            Err(denial) => TransitionOutcome::denied(denial.into()),
        }
    }
}
