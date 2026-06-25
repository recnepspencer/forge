#![forbid(unsafe_code)]

use forge_store_physical_format::PhysicalReference;

pub trait PhysicalStoreBackend {
    type Error;

    fn append_framed_record(&mut self, bytes: &[u8]) -> Result<PhysicalReference, Self::Error>;

    fn read_framed_record(&self, reference: PhysicalReference) -> Result<Vec<u8>, Self::Error>;
}
