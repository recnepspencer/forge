use crate::{
    ExtentGenerationCell, PhysicalBinaryEncodingWitness, PhysicalHeaderAuthority,
    SlotGenerationCell,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalOpenRequest {
    headers: PhysicalHeaderAuthority,
}

impl PlatformPhysicalOpenRequest {
    pub const fn new(headers: PhysicalHeaderAuthority) -> Self {
        Self { headers }
    }

    pub fn s1_canonical() -> Self {
        Self::new(PhysicalHeaderAuthority::s1(
            PhysicalBinaryEncodingWitness::s1_canonical()
                .expect("canonical S.1 binary format is admitted"),
        ))
    }

    pub const fn headers(&self) -> &PhysicalHeaderAuthority {
        &self.headers
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRecordTarget {
    PageSlot(SlotGenerationCell),
    Extent(ExtentGenerationCell),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalAppendRequest<'a> {
    target: PlatformPhysicalRecordTarget,
    payload: &'a [u8],
}

impl<'a> PlatformPhysicalAppendRequest<'a> {
    pub const fn page_slot(slot_cell: SlotGenerationCell, payload: &'a [u8]) -> Self {
        Self {
            target: PlatformPhysicalRecordTarget::PageSlot(slot_cell),
            payload,
        }
    }

    pub const fn extent(extent_cell: ExtentGenerationCell, payload: &'a [u8]) -> Self {
        Self {
            target: PlatformPhysicalRecordTarget::Extent(extent_cell),
            payload,
        }
    }

    pub const fn target(self) -> PlatformPhysicalRecordTarget {
        self.target
    }

    pub const fn payload(self) -> &'a [u8] {
        self.payload
    }
}
