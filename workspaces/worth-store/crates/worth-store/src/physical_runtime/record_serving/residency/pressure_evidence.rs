use worth_store_buffer_pool::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalSpeculativeWorkKind,
};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use crate::physical_runtime::{LifecycleGeneration, PhysicalRecordId, PhysicalWorkIdentity};

/// The Store, record, frame, or work identity affected by physical pressure.
///
/// This basis is descriptive only. It cannot allocate memory, retry work, or
/// control the buffer pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordPressureBasis {
    store: StableStoreIdentity,
    record: Option<PhysicalRecordId>,
    frame_coordinate: Option<RecordFrameCoordinate>,
    work_identity: Option<PhysicalWorkIdentity>,
}

/// Exact pre-effect or post-effect physical-pressure evidence.
///
/// Inspect the dimension, requested/current/limit values, retry posture, and
/// `effect_may_have_started` before deciding how the application should react.
/// This value is not a retry token or allocation grant.
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

/// The state change required before retry may become useful.
///
/// A nonterminal posture describes a necessary condition, not permission or a
/// guarantee that a later retry will succeed.
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

    /// Returns the physical identity affected by the denial.
    pub const fn basis(&self) -> PhysicalRecordPressureBasis {
        self.basis
    }

    /// Returns the serving lifecycle generation that observed the denial.
    pub const fn store_generation(&self) -> LifecycleGeneration {
        self.store_generation
    }

    /// Returns the operation scope whose allocation was denied.
    pub const fn scope(&self) -> PhysicalOperationAllocationScope {
        self.scope
    }

    /// Returns the exhausted or invalid residency dimension.
    pub const fn dimension(&self) -> PhysicalResidencyDimension {
        self.dimension
    }

    /// Returns the units requested by the denied operation.
    pub const fn requested(&self) -> u64 {
        self.requested
    }

    /// Returns the units active when the denial was observed.
    pub const fn admitted(&self) -> u64 {
        self.admitted
    }

    /// Returns the admitted limit for this dimension.
    pub const fn limit(&self) -> u64 {
        self.limit
    }

    /// Returns the state change required before retry may be useful.
    pub const fn retry_posture(&self) -> PhysicalResidencyRetryPosture {
        self.retry_posture
    }

    /// Reports whether an external effect may already have started.
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

    pub(in crate::physical_runtime::record_serving) const fn with_frame_coordinate(
        mut self,
        frame: RecordFrameCoordinate,
    ) -> Self {
        self.frame_coordinate = Some(frame);
        self
    }

    /// Returns the stable physical Store identity.
    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    /// Returns the affected record when the denial is record-specific.
    pub const fn record(&self) -> Option<PhysicalRecordId> {
        self.record
    }

    /// Returns the affected frame when one is known.
    pub const fn frame_coordinate(&self) -> Option<RecordFrameCoordinate> {
        self.frame_coordinate
    }

    /// Returns the affected physical-work identity when one is known.
    pub const fn work_identity(&self) -> Option<PhysicalWorkIdentity> {
        self.work_identity
    }
}

impl PhysicalResidencyRetryPosture {
    /// Returns whether this posture leaves a possible future retry.
    ///
    /// The caller must still wait for the named condition and perform a fresh
    /// admission; `true` is not retry authority.
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
