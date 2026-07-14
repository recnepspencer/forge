use worth_store_physical_format::{
    PageGenerationCell, PhysicalFrameKind, PhysicalHeaderDecodeWitness, PhysicalPageKind,
    PhysicalReferenceValidationWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeclaredPhysicalChecksum {
    value: u64,
}

impl DeclaredPhysicalChecksum {
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn value(self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIntegrityAdmissionRequest {
    Page {
        cell: PageGenerationCell,
        header_witness: PhysicalHeaderDecodeWitness,
        expected_kind: PhysicalPageKind,
        expected_checksum: DeclaredPhysicalChecksum,
    },
    Frame {
        validation: PhysicalReferenceValidationWitness,
        header_witness: PhysicalHeaderDecodeWitness,
        expected_kind: PhysicalFrameKind,
        expected_checksum: DeclaredPhysicalChecksum,
    },
}

impl PhysicalIntegrityAdmissionRequest {
    pub const fn page(
        cell: PageGenerationCell,
        header_witness: PhysicalHeaderDecodeWitness,
        expected_kind: PhysicalPageKind,
        expected_checksum: DeclaredPhysicalChecksum,
    ) -> Self {
        Self::Page {
            cell,
            header_witness,
            expected_kind,
            expected_checksum,
        }
    }

    pub const fn frame(
        validation: PhysicalReferenceValidationWitness,
        header_witness: PhysicalHeaderDecodeWitness,
        expected_kind: PhysicalFrameKind,
        expected_checksum: DeclaredPhysicalChecksum,
    ) -> Self {
        Self::Frame {
            validation,
            header_witness,
            expected_kind,
            expected_checksum,
        }
    }
}
