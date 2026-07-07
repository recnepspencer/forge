use crate::{S3ReadinessDenial, S3ReadinessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2BoundedCounterRecap {
    resident_bytes: u64,
    pinned_pages: u64,
    dirty_pages: u32,
    allocation_bytes: u64,
    copied_bytes: u64,
    materialized_bytes: u64,
}

impl S2BoundedCounterRecap {
    pub fn exact(
        resident_bytes: u64,
        pinned_pages: u64,
        dirty_pages: u32,
        allocation_bytes: u64,
        copied_bytes: u64,
        materialized_bytes: u64,
    ) -> Result<Self, S3ReadinessDenial> {
        if resident_bytes == 0 || allocation_bytes == 0 {
            return Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingCounterRecap,
            ));
        }
        Ok(Self {
            resident_bytes,
            pinned_pages,
            dirty_pages,
            allocation_bytes,
            copied_bytes,
            materialized_bytes,
        })
    }

    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(self) -> u64 {
        self.pinned_pages
    }

    pub const fn dirty_pages(self) -> u32 {
        self.dirty_pages
    }

    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }

    pub const fn materialized_bytes(self) -> u64 {
        self.materialized_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S2DeniedBoundaryKind {
    OverBudgetResidency,
    PinLeak,
    DirtyOverflow,
    WholeStoreMaterialization,
    WholeObjectStreaming,
    ForgedViewAccess,
}

impl S2DeniedBoundaryKind {
    pub const ALL: [Self; 6] = [
        Self::OverBudgetResidency,
        Self::PinLeak,
        Self::DirtyOverflow,
        Self::WholeStoreMaterialization,
        Self::WholeObjectStreaming,
        Self::ForgedViewAccess,
    ];

    const fn bit(self) -> u8 {
        match self {
            Self::OverBudgetResidency => 1 << 0,
            Self::PinLeak => 1 << 1,
            Self::DirtyOverflow => 1 << 2,
            Self::WholeStoreMaterialization => 1 << 3,
            Self::WholeObjectStreaming => 1 << 4,
            Self::ForgedViewAccess => 1 << 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2DenialBehaviorRecap {
    named_denial_mask: u8,
}

impl S2DenialBehaviorRecap {
    pub fn from_named_boundaries(
        boundaries: &[S2DeniedBoundaryKind],
    ) -> Result<Self, S3ReadinessDenial> {
        let mask = boundaries
            .iter()
            .fold(0u8, |mask, boundary| mask | boundary.bit());
        let recap = Self {
            named_denial_mask: mask,
        };
        if S2DeniedBoundaryKind::ALL
            .iter()
            .all(|boundary| recap.contains(*boundary))
        {
            Ok(recap)
        } else {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingDenialBehavior,
            ))
        }
    }

    pub const fn contains(self, boundary: S2DeniedBoundaryKind) -> bool {
        self.named_denial_mask & boundary.bit() != 0
    }

    pub const fn named_denial_count(self) -> u32 {
        self.named_denial_mask.count_ones()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalAuthorityRecap {
    physical_reference_count: u32,
    header_decode_witness_count: u32,
    payload_admission_witness_count: u32,
}

impl PhysicalAuthorityRecap {
    pub fn from_s1_authority(
        physical_reference_count: u32,
        header_decode_witness_count: u32,
        payload_admission_witness_count: u32,
    ) -> Result<Self, S3ReadinessDenial> {
        if physical_reference_count == 0
            || header_decode_witness_count == 0
            || payload_admission_witness_count == 0
        {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingPhysicalAuthorityRecap,
            ))
        } else {
            Ok(Self {
                physical_reference_count,
                header_decode_witness_count,
                payload_admission_witness_count,
            })
        }
    }

    pub const fn physical_reference_count(self) -> u32 {
        self.physical_reference_count
    }

    pub const fn header_decode_witness_count(self) -> u32 {
        self.header_decode_witness_count
    }

    pub const fn payload_admission_witness_count(self) -> u32 {
        self.payload_admission_witness_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolAuthorityRecap {
    lease_pinning_proven: bool,
    resident_frame_authority_proven: bool,
    allocation_envelope_proven: bool,
    view_admission_authority_proven: bool,
}

impl BufferPoolAuthorityRecap {
    pub fn s2_authority(
        lease_pinning_proven: bool,
        resident_frame_authority_proven: bool,
        allocation_envelope_proven: bool,
        view_admission_authority_proven: bool,
    ) -> Result<Self, S3ReadinessDenial> {
        if lease_pinning_proven
            && resident_frame_authority_proven
            && allocation_envelope_proven
            && view_admission_authority_proven
        {
            Ok(Self {
                lease_pinning_proven,
                resident_frame_authority_proven,
                allocation_envelope_proven,
                view_admission_authority_proven,
            })
        } else {
            Err(S3ReadinessDenial::new(
                S3ReadinessDenialKind::MissingBufferPoolAuthorityRecap,
            ))
        }
    }

    pub const fn lease_pinning_proven(self) -> bool {
        self.lease_pinning_proven
    }

    pub const fn resident_frame_authority_proven(self) -> bool {
        self.resident_frame_authority_proven
    }

    pub const fn allocation_envelope_proven(self) -> bool {
        self.allocation_envelope_proven
    }

    pub const fn view_admission_authority_proven(self) -> bool {
        self.view_admission_authority_proven
    }
}
