use super::{
    BlobPhysicalManifestValidation, ExternalPlacementRecoverabilityDenial,
    ExternalPlacementRecoveryProbe,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementRecoveryProbeRequest {
    manifest: BlobPhysicalManifestValidation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalPlacementRecoveryProbeObservation {
    placement_digest: String,
    recovery_probe_completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalPlacementRecoveryProbeExecutionError<BackendError> {
    Backend(BackendError),
    Denied(ExternalPlacementRecoverabilityDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreOwnedExternalPlacementRecoveryProbe {
    _private: (),
}

pub trait PhysicalStoreExternalPlacementRecoveryProber {
    type Error;

    fn probe_external_placement_recovery(
        &mut self,
        request: ExternalPlacementRecoveryProbeRequest,
    ) -> Result<ExternalPlacementRecoveryProbeObservation, Self::Error>;
}

pub struct ExternalPlacementRecoveryProbeSession<'backend, Backend> {
    backend: &'backend mut Backend,
    authority: StoreOwnedExternalPlacementRecoveryProbe,
}

impl ExternalPlacementRecoveryProbeRequest {
    pub const fn new(manifest: BlobPhysicalManifestValidation) -> Self {
        Self { manifest }
    }

    pub const fn manifest(&self) -> &BlobPhysicalManifestValidation {
        &self.manifest
    }
}

impl ExternalPlacementRecoveryProbeObservation {
    pub fn new(placement_digest: impl Into<String>, recovery_probe_completed: bool) -> Self {
        Self {
            placement_digest: placement_digest.into(),
            recovery_probe_completed,
        }
    }
}

impl StoreOwnedExternalPlacementRecoveryProbe {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self { _private: () }
    }
}

impl<'backend, Backend> ExternalPlacementRecoveryProbeSession<'backend, Backend> {
    pub fn for_store_backend(
        backend: &'backend mut Backend,
        authority: StoreOwnedExternalPlacementRecoveryProbe,
    ) -> Self {
        Self { backend, authority }
    }

    pub fn for_owned_backend(backend: &'backend mut Backend) -> Self {
        Self::for_store_backend(
            backend,
            StoreOwnedExternalPlacementRecoveryProbe::store_owned(),
        )
    }

    pub fn execute(
        &mut self,
        request: ExternalPlacementRecoveryProbeRequest,
    ) -> Result<
        ExternalPlacementRecoveryProbe,
        ExternalPlacementRecoveryProbeExecutionError<Backend::Error>,
    >
    where
        Backend: PhysicalStoreExternalPlacementRecoveryProber,
    {
        let observation = self
            .backend
            .probe_external_placement_recovery(request.clone())
            .map_err(ExternalPlacementRecoveryProbeExecutionError::Backend)?;
        self.authority
            .complete(request, observation)
            .map_err(ExternalPlacementRecoveryProbeExecutionError::Denied)
    }
}

impl StoreOwnedExternalPlacementRecoveryProbe {
    fn complete(
        self,
        request: ExternalPlacementRecoveryProbeRequest,
        observation: ExternalPlacementRecoveryProbeObservation,
    ) -> Result<ExternalPlacementRecoveryProbe, ExternalPlacementRecoverabilityDenial> {
        if !observation.recovery_probe_completed {
            return Err(ExternalPlacementRecoverabilityDenial::RecoveryProbeIncomplete);
        }
        if observation.placement_digest != request.manifest.placement_digest() {
            return Err(ExternalPlacementRecoverabilityDenial::ManifestProbeMismatch);
        }
        Ok(ExternalPlacementRecoveryProbe::from_store_recovery_probe(
            request.manifest,
        ))
    }
}
