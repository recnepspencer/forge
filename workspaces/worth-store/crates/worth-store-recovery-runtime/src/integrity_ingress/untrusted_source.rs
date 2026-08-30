use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_integrity::{PhysicalArtifactScope, UntrustedPhysicalArtifact};

/// Recovery-private binding between one C.4 bounded read and its expected scope.
pub(crate) struct UntrustedRecoverySource<'media> {
    observed: &'media ObservedRecoveryArtifact,
    scope: PhysicalArtifactScope,
}

impl<'media> UntrustedRecoverySource<'media> {
    pub(crate) const fn new(
        observed: &'media ObservedRecoveryArtifact,
        scope: PhysicalArtifactScope,
    ) -> Self {
        Self { observed, scope }
    }

    pub(crate) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub(crate) fn input(&self) -> Option<UntrustedPhysicalArtifact<'media>> {
        self.observed
            .bytes()
            .map(UntrustedPhysicalArtifact::from_bounded_bytes)
    }

    pub(crate) const fn observed(&self) -> &'media ObservedRecoveryArtifact {
        self.observed
    }
}
