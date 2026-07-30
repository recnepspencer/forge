use worth_store::physical_runtime::{PhysicalRecordChunkBasis, PhysicalRecordChunkView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtectedPhysicalByteView<'lease> {
    bytes: &'lease [u8],
    basis: PhysicalRecordChunkBasis,
}

impl<'lease> ProtectedPhysicalByteView<'lease> {
    pub fn from_store_chunk(view: &PhysicalRecordChunkView<'lease>) -> Self {
        Self {
            bytes: view.bytes(),
            basis: view.basis(),
        }
    }

    pub const fn basis(self) -> PhysicalRecordChunkBasis {
        self.basis
    }

    pub const fn as_bytes(self) -> &'lease [u8] {
        self.bytes
    }

    pub const fn len_bytes(self) -> usize {
        self.bytes.len()
    }

    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }
}
