use worth_store_buffer_pool::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use crate::physical_runtime::{LifecycleGeneration, PhysicalRecordId, PhysicalWorkIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordPressureBasis {
    store: StableStoreIdentity,
    record: Option<PhysicalRecordId>,
    frame_coordinate: Option<RecordFrameCoordinate>,
    work_identity: Option<PhysicalWorkIdentity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordPressureEvidence {
    basis: PhysicalRecordPressureBasis,
    store_generation: LifecycleGeneration,
    scope: PhysicalOperationAllocationScope,
    dimension: PhysicalResidencyDimension,
    requested: u64,
    admitted: u64,
    limit: u64,
    retry_posture: PhysicalResidencyRetryPosture,
    effect_may_have_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResidencyRetryPosture {
    AfterLeaseRelease,
    AfterAllocationRelease,
    AfterWritebackSettlement,
    AfterGenerationReadmission,
    AfterConfigurationChange,
    Terminal,
}

impl PhysicalRecordPressureEvidence {
    pub(in crate::physical_runtime::record_serving) fn from_store_failure(
        failure: super::PhysicalRecordResidencyFailure,
        store_generation: LifecycleGeneration,
    ) -> Option<Self> {
        let pressure = failure.pressure_denial()?;
        Self::from_failure(
            failure,
            store_generation,
            PhysicalRecordPressureBasis::for_store(pressure.store()),
        )
    }

    pub(in crate::physical_runtime::record_serving) fn from_failure(
        failure: super::PhysicalRecordResidencyFailure,
        store_generation: LifecycleGeneration,
        basis: PhysicalRecordPressureBasis,
    ) -> Option<Self> {
        let pressure = failure.pressure_denial()?;
        debug_assert_eq!(pressure.store(), basis.store);
        Some(Self {
            basis,
            store_generation,
            scope: pressure.scope(),
            dimension: pressure.dimension(),
            requested: pressure.requested(),
            admitted: pressure.current(),
            limit: pressure.limit(),
            retry_posture: classify_retry(
                pressure.dimension(),
                pressure.requested(),
                pressure.limit(),
            ),
            effect_may_have_started: pressure.effect_may_have_started(),
        })
    }

    pub const fn basis(&self) -> PhysicalRecordPressureBasis {
        self.basis
    }

    pub const fn store_generation(&self) -> LifecycleGeneration {
        self.store_generation
    }

    pub const fn scope(&self) -> PhysicalOperationAllocationScope {
        self.scope
    }

    pub const fn dimension(&self) -> PhysicalResidencyDimension {
        self.dimension
    }

    pub const fn requested(&self) -> u64 {
        self.requested
    }

    pub const fn admitted(&self) -> u64 {
        self.admitted
    }

    pub const fn limit(&self) -> u64 {
        self.limit
    }

    pub const fn retry_posture(&self) -> PhysicalResidencyRetryPosture {
        self.retry_posture
    }

    pub const fn effect_may_have_started(&self) -> bool {
        self.effect_may_have_started
    }
}

impl PhysicalRecordPressureBasis {
    pub(in crate::physical_runtime::record_serving) const fn for_store(
        store: StableStoreIdentity,
    ) -> Self {
        Self {
            store,
            record: None,
            frame_coordinate: None,
            work_identity: None,
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn with_record(
        mut self,
        record: PhysicalRecordId,
    ) -> Self {
        self.record = Some(record);
        self
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn record(&self) -> Option<PhysicalRecordId> {
        self.record
    }

    pub const fn frame_coordinate(&self) -> Option<RecordFrameCoordinate> {
        self.frame_coordinate
    }

    pub const fn work_identity(&self) -> Option<PhysicalWorkIdentity> {
        self.work_identity
    }
}

impl PhysicalResidencyRetryPosture {
    pub const fn may_retry(self) -> bool {
        !matches!(self, Self::Terminal)
    }
}

const fn classify_retry(
    dimension: PhysicalResidencyDimension,
    requested: u64,
    limit: u64,
) -> PhysicalResidencyRetryPosture {
    if requested > limit {
        return PhysicalResidencyRetryPosture::AfterConfigurationChange;
    }
    match dimension {
        PhysicalResidencyDimension::ResidentBytes
        | PhysicalResidencyDimension::FrameEntries
        | PhysicalResidencyDimension::PinnedFrames
        | PhysicalResidencyDimension::PinLeases
        | PhysicalResidencyDimension::SpeculativeFrames(
            PhysicalSpeculativeWorkKind::ReadAhead | PhysicalSpeculativeWorkKind::Prefetch,
        ) => PhysicalResidencyRetryPosture::AfterLeaseRelease,
        PhysicalResidencyDimension::DirtyFrames
        | PhysicalResidencyDimension::SpeculativeFrames(PhysicalSpeculativeWorkKind::WriteBehind) => {
            PhysicalResidencyRetryPosture::AfterWritebackSettlement
        }
        PhysicalResidencyDimension::TotalBytes
        | PhysicalResidencyDimension::DirtyReplacementBytes
        | PhysicalResidencyDimension::OperationBytes
        | PhysicalResidencyDimension::OperationScope(_) => {
            PhysicalResidencyRetryPosture::AfterAllocationRelease
        }
        PhysicalResidencyDimension::MetadataBytes => {
            PhysicalResidencyRetryPosture::AfterConfigurationChange
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_hard_dimension_has_an_explicit_retry_release_condition() {
        for dimension in [
            PhysicalResidencyDimension::ResidentBytes,
            PhysicalResidencyDimension::FrameEntries,
            PhysicalResidencyDimension::PinnedFrames,
            PhysicalResidencyDimension::PinLeases,
            PhysicalResidencyDimension::SpeculativeFrames(PhysicalSpeculativeWorkKind::ReadAhead),
        ] {
            assert_eq!(
                classify_retry(dimension, 1, 8),
                PhysicalResidencyRetryPosture::AfterLeaseRelease,
            );
        }
        for dimension in [
            PhysicalResidencyDimension::TotalBytes,
            PhysicalResidencyDimension::DirtyReplacementBytes,
            PhysicalResidencyDimension::OperationBytes,
            PhysicalResidencyDimension::OperationScope(
                PhysicalOperationAllocationScope::ForegroundRead,
            ),
        ] {
            assert_eq!(
                classify_retry(dimension, 1, 8),
                PhysicalResidencyRetryPosture::AfterAllocationRelease,
            );
        }
        for dimension in [
            PhysicalResidencyDimension::DirtyFrames,
            PhysicalResidencyDimension::SpeculativeFrames(PhysicalSpeculativeWorkKind::WriteBehind),
        ] {
            assert_eq!(
                classify_retry(dimension, 1, 8),
                PhysicalResidencyRetryPosture::AfterWritebackSettlement,
            );
        }
        assert_eq!(
            classify_retry(PhysicalResidencyDimension::MetadataBytes, 1, 8),
            PhysicalResidencyRetryPosture::AfterConfigurationChange,
        );
        assert_eq!(
            classify_retry(PhysicalResidencyDimension::ResidentBytes, 9, 8),
            PhysicalResidencyRetryPosture::AfterConfigurationChange,
        );
    }

    #[test]
    fn only_terminal_posture_forbids_retry() {
        assert!(!PhysicalResidencyRetryPosture::Terminal.may_retry());
        assert!(PhysicalResidencyRetryPosture::AfterGenerationReadmission.may_retry());
    }
}
