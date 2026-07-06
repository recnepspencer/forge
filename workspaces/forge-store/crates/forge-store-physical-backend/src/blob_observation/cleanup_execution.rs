use super::{
    ExternalPlacementCleanupReceipt, ExternalPlacementOrphanScanReceipt,
    ExternalPlacementRecoverabilityDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementCleanupRequest {
    orphan_scan: ExternalPlacementOrphanScanReceipt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementCleanupObservation {
    orphan_token: String,
    cleanup_completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalPlacementCleanupExecutionError<BackendError> {
    Backend(BackendError),
    Denied(ExternalPlacementRecoverabilityDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedExternalPlacementCleanup {
    _private: (),
}

pub trait PhysicalStoreExternalPlacementCleanupExecutor {
    type Error;

    fn cleanup_external_placement_residue(
        &mut self,
        request: ExternalPlacementCleanupRequest,
    ) -> Result<ExternalPlacementCleanupObservation, Self::Error>;
}

pub struct ExternalPlacementCleanupSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedExternalPlacementCleanup,
}

impl ExternalPlacementCleanupRequest {
    pub const fn new(orphan_scan: ExternalPlacementOrphanScanReceipt) -> Self {
        Self { orphan_scan }
    }

    pub const fn orphan_scan(&self) -> &ExternalPlacementOrphanScanReceipt {
        &self.orphan_scan
    }
}

impl ExternalPlacementCleanupObservation {
    pub fn new(orphan_token: impl Into<String>, cleanup_completed: bool) -> Self {
        Self {
            orphan_token: orphan_token.into(),
            cleanup_completed,
        }
    }
}

impl StoreOwnedExternalPlacementCleanup {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self { _private: () }
    }
}

impl<'backend, Backend> ExternalPlacementCleanupSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedExternalPlacementCleanup,
    ) -> Self {
        Self { backend, authority }
    }

    #[allow(dead_code)]
    pub(crate) fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(backend, StoreOwnedExternalPlacementCleanup::store_owned())
    }

    pub fn execute(
        &mut self,
        request: ExternalPlacementCleanupRequest,
    ) -> Result<
        ExternalPlacementCleanupReceipt,
        ExternalPlacementCleanupExecutionError<Backend::Error>,
    >
    where
        Backend: PhysicalStoreExternalPlacementCleanupExecutor,
    {
        let observation = self
            .backend
            .cleanup_external_placement_residue(request.clone())
            .map_err(ExternalPlacementCleanupExecutionError::Backend)?;
        self.authority
            .complete(request, observation)
            .map_err(ExternalPlacementCleanupExecutionError::Denied)
    }
}

impl StoreOwnedExternalPlacementCleanup {
    fn complete(
        self,
        request: ExternalPlacementCleanupRequest,
        observation: ExternalPlacementCleanupObservation,
    ) -> Result<ExternalPlacementCleanupReceipt, ExternalPlacementRecoverabilityDenial> {
        if !observation.cleanup_completed {
            return Err(ExternalPlacementRecoverabilityDenial::CleanupIncomplete);
        }
        if observation.orphan_token != request.orphan_scan.token() {
            return Err(ExternalPlacementRecoverabilityDenial::CleanupScanMismatch);
        }
        Ok(ExternalPlacementCleanupReceipt::from_store_cleanup(
            observation.orphan_token,
        ))
    }
}
