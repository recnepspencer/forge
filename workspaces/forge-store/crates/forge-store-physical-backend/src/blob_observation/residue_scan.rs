use super::{BlobBackendResidueObservation, BlobBackendResidueObservationKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobBackendResidueScanRequest {
    kind: BlobBackendResidueObservationKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobBackendResidueScanObservation {
    kind: BlobBackendResidueObservationKind,
    observed_token: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedBlobBackendResidueScan {
    _private: (),
}

pub trait PhysicalStoreBlobResidueScanner {
    type Error;

    fn scan_blob_residue(
        &mut self,
        request: BlobBackendResidueScanRequest,
    ) -> Result<BlobBackendResidueScanObservation, Self::Error>;
}

pub struct BlobBackendResidueScanSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedBlobBackendResidueScan,
}

impl BlobBackendResidueScanRequest {
    pub const fn new(kind: BlobBackendResidueObservationKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> BlobBackendResidueObservationKind {
        self.kind
    }
}

impl BlobBackendResidueScanObservation {
    pub fn new(kind: BlobBackendResidueObservationKind, observed_token: impl Into<String>) -> Self {
        Self {
            kind,
            observed_token: observed_token.into(),
        }
    }
}

impl StoreOwnedBlobBackendResidueScan {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self { _private: () }
    }
}

impl<'backend, Backend> BlobBackendResidueScanSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedBlobBackendResidueScan,
    ) -> Self {
        Self { backend, authority }
    }

    #[allow(dead_code)]
    pub(crate) fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(backend, StoreOwnedBlobBackendResidueScan::store_owned())
    }

    pub fn execute(
        &mut self,
        request: BlobBackendResidueScanRequest,
    ) -> Result<BlobBackendResidueObservation, Backend::Error>
    where
        Backend: PhysicalStoreBlobResidueScanner,
    {
        let observation = self.backend.scan_blob_residue(request)?;
        Ok(self.authority.complete(observation))
    }
}

impl StoreOwnedBlobBackendResidueScan {
    fn complete(
        self,
        observation: BlobBackendResidueScanObservation,
    ) -> BlobBackendResidueObservation {
        BlobBackendResidueObservation::observed(observation.kind, observation.observed_token)
    }
}
