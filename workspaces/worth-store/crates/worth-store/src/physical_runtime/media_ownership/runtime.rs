use worth_store_physical_backend::{
    FilesystemBackendProfile, QualifiedFilesystemMedia, QualifiedMediaCapabilities,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{runtime::PhysicalRuntimeCore, RuntimeIdentity};
use crate::physical_runtime::{AbortedRuntime, ClosedRuntime};

use super::{MediaShutdownOutcome, PhysicalMediaObserver};

/// Sole move-only C.4 owner of the original runtime core and one qualified
/// real-filesystem owner.
pub struct MediaOwnedPhysicalRuntime {
    media: QualifiedFilesystemMedia,
    core: PhysicalRuntimeCore,
}

impl MediaOwnedPhysicalRuntime {
    pub(super) const fn new(core: PhysicalRuntimeCore, media: QualifiedFilesystemMedia) -> Self {
        Self { media, core }
    }

    pub const fn runtime_identity(&self) -> RuntimeIdentity {
        self.core.runtime_identity()
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.media.store_identity()
    }

    pub fn backend_profile(&self) -> &FilesystemBackendProfile {
        self.media.profile()
    }

    pub fn capabilities(&self) -> &QualifiedMediaCapabilities {
        self.media.capabilities()
    }

    pub fn qualification_report(
        &self,
    ) -> worth_store_physical_backend::RootProfileQualificationReport {
        self.media.qualification_report()
    }

    pub fn media_counters(&self) -> worth_store_physical_backend::MediaCounterSnapshot {
        self.media.counters()
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn certification_operation_summary(
        &self,
    ) -> Result<
        crate::physical_runtime::certification::MediaOperationSummary,
        crate::physical_runtime::certification::MediaEvidenceLoweringDenial,
    > {
        crate::physical_runtime::media_evidence::MediaOperationSummary::from_qualified_media(
            &self.media,
        )
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn certification_confinement_probe(
        &self,
        authority: crate::physical_runtime::certification::CertificationMediaFaultAuthority,
        component: &str,
    ) -> Result<(), worth_store_physical_backend::NamespaceConfinementDenial> {
        self.media
            .certification_confinement_probe(authority, component)
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn certification_staging_effect_probe(
        &self,
        authority: crate::physical_runtime::certification::CertificationMediaFaultAuthority,
        component: &str,
    ) -> worth_store_physical_backend::CertificationConfinementEffect {
        self.media
            .certification_staging_effect_probe(authority, component)
    }

    pub fn observer(&self) -> PhysicalMediaObserver {
        let (lifecycle, lease) = self.core.media_observation_parts();
        PhysicalMediaObserver::new(
            self.runtime_identity(),
            self.store_identity(),
            self.media.mutation_owner(),
            self.media.profile().clone(),
            self.media.counter_observer(),
            lifecycle,
            lease,
        )
    }

    pub fn close(self) -> MediaShutdownOutcome<ClosedRuntime> {
        let Self { media, core } = self;
        let release = media.close();
        MediaShutdownOutcome::new(core.close(), release)
    }

    pub fn abort(self) -> MediaShutdownOutcome<AbortedRuntime> {
        let Self { media, core } = self;
        let release = media.close();
        MediaShutdownOutcome::new(core.abort(), release)
    }
}
