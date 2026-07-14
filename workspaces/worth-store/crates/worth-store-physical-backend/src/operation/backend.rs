use crate::PhysicalReference;

/// Executes ordinary framed-record operations against a physical store backend.
pub trait PhysicalStoreBackend {
    type Error;

    fn append_framed_record(&mut self, bytes: &[u8]) -> Result<PhysicalReference, Self::Error>;

    fn read_framed_record(&self, reference: PhysicalReference) -> Result<Vec<u8>, Self::Error>;
}
