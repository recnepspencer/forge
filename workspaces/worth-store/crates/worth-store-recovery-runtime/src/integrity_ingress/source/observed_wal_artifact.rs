use worth_store::physical_runtime::ObservedWalArtifact;
use worth_store_physical_integrity::{
    PhysicalArtifactScope, PhysicalByteRange, UntrustedPhysicalArtifact,
};

use super::super::RecoveryIntegrityIngressRejection;

/// One exact frame range borrowed from a C.4 bounded WAL observation.
pub(crate) struct ObservedWalFrameSource<'media> {
    observed: &'media ObservedWalArtifact,
    scope: PhysicalArtifactScope,
    relative_range: PhysicalByteRange,
}

impl<'media> ObservedWalFrameSource<'media> {
    pub(crate) const fn new(
        observed: &'media ObservedWalArtifact,
        scope: PhysicalArtifactScope,
        relative_range: PhysicalByteRange,
    ) -> Self {
        Self {
            observed,
            scope,
            relative_range,
        }
    }

    pub(in crate::integrity_ingress) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub(in crate::integrity_ingress) fn name(&self) -> &'media std::ffi::OsStr {
        self.observed.name()
    }

    pub(in crate::integrity_ingress) const fn entry_type(
        &self,
    ) -> worth_store_physical_format::store_namespace::NamespaceEntryType {
        self.observed.entry_type()
    }

    pub(in crate::integrity_ingress) fn input(
        &self,
    ) -> Result<UntrustedPhysicalArtifact<'media>, RecoveryIntegrityIngressRejection> {
        if self.relative_range != self.scope.byte_range() {
            return Err(RecoveryIntegrityIngressRejection::ScopeMismatch);
        }
        let bytes = self
            .observed
            .bytes()
            .ok_or(RecoveryIntegrityIngressRejection::MissingBoundedArtifact)?;
        let start = usize::try_from(self.relative_range.offset())
            .map_err(|_| RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation)?;
        let end = usize::try_from(self.relative_range.end_exclusive())
            .map_err(|_| RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation)?;
        let inspected = bytes
            .get(start..end)
            .ok_or(RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation)?;
        Ok(UntrustedPhysicalArtifact::from_bounded_bytes(inspected))
    }
}
