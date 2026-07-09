use crate::PhysicalReference;
use worth_store_physical_format::PhysicalAlignmentClass;

use super::AccessPolicyBufferLifecycle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectIoAlignmentRequirement {
    reference: PhysicalReference,
    lifecycle: AccessPolicyBufferLifecycle,
    byte_length: u32,
    page_alignment: PhysicalAlignmentClass,
    extent_alignment: PhysicalAlignmentClass,
    page_aligned: bool,
    sector_aligned: bool,
    buffer_lifetime_stable: bool,
    _seal: DirectIoAlignmentSeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DirectIoAlignmentSeal;

impl DirectIoAlignmentRequirement {
    pub(crate) const fn page_and_sector(
        reference: PhysicalReference,
        lifecycle: AccessPolicyBufferLifecycle,
        byte_length: u32,
        page_alignment: PhysicalAlignmentClass,
        extent_alignment: PhysicalAlignmentClass,
    ) -> Self {
        Self {
            reference,
            lifecycle,
            byte_length,
            page_alignment,
            extent_alignment,
            page_aligned: true,
            sector_aligned: true,
            buffer_lifetime_stable: true,
            _seal: DirectIoAlignmentSeal,
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn unaligned(
        reference: PhysicalReference,
        lifecycle: AccessPolicyBufferLifecycle,
    ) -> Self {
        Self {
            reference,
            lifecycle,
            byte_length: 1,
            page_alignment: PhysicalAlignmentClass::page_start_4k(),
            extent_alignment: PhysicalAlignmentClass::extent_start_4k(),
            page_aligned: false,
            sector_aligned: true,
            buffer_lifetime_stable: true,
            _seal: DirectIoAlignmentSeal,
        }
    }

    pub fn is_satisfied_for(
        self,
        reference: PhysicalReference,
        lifecycle: AccessPolicyBufferLifecycle,
    ) -> bool {
        self.reference == reference
            && self.lifecycle.kind() == lifecycle.kind()
            && self.byte_length > 0
            && self.byte_length % self.page_alignment.bytes() as u32 == 0
            && self.page_aligned
            && self.sector_aligned
            && self.buffer_lifetime_stable
    }
}
