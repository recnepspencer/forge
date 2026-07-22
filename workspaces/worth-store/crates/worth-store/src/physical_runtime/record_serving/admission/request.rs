use super::super::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordResidencyPolicy {
    limits: worth_store_buffer_pool::PhysicalResidencyLimits,
}

impl PhysicalRecordResidencyPolicy {
    pub fn new(
        resident_bytes: u64,
        pinned_frames: u32,
        dirty_frames: u32,
        operation_bytes: u64,
        frame_entries: u32,
    ) -> Option<Self> {
        worth_store_buffer_pool::PhysicalResidencyLimits::new(
            resident_bytes,
            pinned_frames,
            dirty_frames,
            operation_bytes,
            frame_entries,
        )
        .map(|limits| Self { limits })
    }

    pub fn new_with_metadata_budget(
        resident_bytes: u64,
        metadata_bytes: u64,
        pinned_frames: u32,
        dirty_frames: u32,
        operation_bytes: u64,
        frame_entries: u32,
    ) -> Option<Self> {
        worth_store_buffer_pool::PhysicalResidencyLimits::new_with_metadata_budget(
            resident_bytes,
            metadata_bytes,
            pinned_frames,
            dirty_frames,
            operation_bytes,
            frame_entries,
        )
        .map(|limits| Self { limits })
    }

    pub const fn with_speculative_frame_limits(
        self,
        prefetch_frames: u32,
        read_ahead_frames: u32,
        write_back_frames: u32,
    ) -> Option<Self> {
        match self.limits.with_speculative_frame_limits(
            prefetch_frames,
            read_ahead_frames,
            write_back_frames,
        ) {
            Some(limits) => Some(Self { limits }),
            None => None,
        }
    }

    pub const fn with_pin_lease_limit(self, pin_leases: u32) -> Option<Self> {
        match self.limits.with_pin_lease_limit(pin_leases) {
            Some(limits) => Some(Self { limits }),
            None => None,
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn limits(
        self,
    ) -> worth_store_buffer_pool::PhysicalResidencyLimits {
        self.limits
    }

    pub const fn resident_bytes(self) -> u64 {
        self.limits.resident_bytes()
    }
    pub const fn pinned_frames(self) -> u32 {
        self.limits.pinned_frames()
    }
    pub const fn pin_leases(self) -> u32 {
        self.limits.pin_leases()
    }
    pub const fn metadata_bytes(self) -> u64 {
        self.limits.metadata_bytes()
    }
    pub const fn dirty_frames(self) -> u32 {
        self.limits.dirty_frames()
    }
    pub const fn operation_bytes(self) -> u64 {
        self.limits.operation_bytes()
    }
    pub const fn frame_entries(self) -> u32 {
        self.limits.frame_entries()
    }
    pub const fn prefetch_frames(self) -> u32 {
        self.limits.prefetch_frames()
    }
    pub const fn read_ahead_frames(self) -> u32 {
        self.limits.read_ahead_frames()
    }
    pub const fn write_back_frames(self) -> u32 {
        self.limits.write_back_frames()
    }

    pub(in crate::physical_runtime::record_serving) fn preflight_format(
        self,
        format: AdmittedPhysicalRecordFormat,
        _access: AdmittedRecordAccessPolicy,
    ) -> Result<(), worth_store_buffer_pool::PhysicalResidencyDenial> {
        let page_bytes = u64::from(format.declaration().page_size().bytes());
        if self.resident_bytes() < page_bytes {
            return Err(
                worth_store_buffer_pool::PhysicalResidencyDenial::FrameLargerThanResidentBudget,
            );
        }
        if self.operation_bytes() < page_bytes {
            return Err(worth_store_buffer_pool::PhysicalResidencyDenial::OperationBudgetExceeded);
        }
        Ok(())
    }
}

impl Default for PhysicalRecordResidencyPolicy {
    fn default() -> Self {
        Self::new_with_metadata_budget(
            64 * 1024 * 1024,
            2 * 1024 * 1024,
            256,
            64,
            256 * 1024 * 1024,
            4096,
        )
        .expect("default physical residency policy is nonzero")
    }
}

pub struct PhysicalRecordInitialization {
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) placement: AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) residency: PhysicalRecordResidencyPolicy,
}

impl PhysicalRecordInitialization {
    pub fn new(
        format: AdmittedPhysicalRecordFormat,
        placement: AdmittedRecordPlacementPolicy,
        access: AdmittedRecordAccessPolicy,
    ) -> Self {
        Self {
            format,
            placement,
            access,
            residency: PhysicalRecordResidencyPolicy::default(),
        }
    }

    pub const fn with_residency_policy(mut self, policy: PhysicalRecordResidencyPolicy) -> Self {
        self.residency = policy;
        self
    }
}

pub struct PhysicalRecordOpen {
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) residency: PhysicalRecordResidencyPolicy,
}

impl PhysicalRecordOpen {
    pub fn new(format: AdmittedPhysicalRecordFormat, access: AdmittedRecordAccessPolicy) -> Self {
        Self {
            format,
            access,
            residency: PhysicalRecordResidencyPolicy::default(),
        }
    }

    pub const fn with_residency_policy(mut self, policy: PhysicalRecordResidencyPolicy) -> Self {
        self.residency = policy;
        self
    }
}
