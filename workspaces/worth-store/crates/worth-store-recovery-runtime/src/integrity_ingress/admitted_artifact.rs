use worth_store::physical_runtime::ObservedRecoveryArtifact;
use worth_store_physical_integrity::{PhysicalArtifactScope, PhysicalIntegrityValidationRecord};

/// Recovery-owned result retaining the exact bounded-read lifetime.
pub(crate) struct IntegrityAdmittedRecoveryArtifact<'media> {
    source: &'media ObservedRecoveryArtifact,
    scope: PhysicalArtifactScope,
    validation: PhysicalIntegrityValidationRecord,
}

impl<'media> IntegrityAdmittedRecoveryArtifact<'media> {
    pub(crate) const fn new(
        source: &'media ObservedRecoveryArtifact,
        scope: PhysicalArtifactScope,
        validation: PhysicalIntegrityValidationRecord,
    ) -> Self {
        Self {
            source,
            scope,
            validation,
        }
    }

    pub(crate) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub(crate) const fn validation(&self) -> PhysicalIntegrityValidationRecord {
        self.validation
    }

    pub(crate) fn bytes(&self) -> &'media [u8] {
        self.source
            .bytes()
            .expect("admission retained a present bounded recovery artifact")
    }
}
