use super::detail::AccessShapeDetail;
use super::lane::AccessLaneClassification;
use crate::maintenance::PhysicalMutationShape;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessAuthorityPosture {
    ExactMaterialized,
    ExplicitDegradedExactScan,
    MaintenanceMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessStaleDisposition {
    ExactOnly,
    RebindBeforeExecution,
    ExplicitDegradedFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedCounterClass {
    PointLookup,
    BatchPointLookup,
    SortedBatchLookup,
    RangeLookup,
    MultiRangeLookup,
    PrefixLookup,
    GroupedPrefixLookup,
    CoalescedPageRead,
    ChunkTreeWalk,
    ManifestGraphWalk,
    BoundedScan,
    FullDeclaredScan,
    StreamingRead,
    StreamingContinuationRead,
    AppendTraversal,
    CompactionTraversal,
    RebuildTraversal,
    VerifierTraversal,
    RepairTraversal,
    QuarantineTraversal,
    DegradedExactScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessShapeContract {
    detail: AccessShapeDetail,
    lane: AccessLaneClassification,
    authority_posture: AccessAuthorityPosture,
    stale_disposition: AccessStaleDisposition,
    expected_counters: ExpectedCounterClass,
    mutation_shape: Option<PhysicalMutationShape>,
    budget_rows: Option<u64>,
}

impl AccessShapeContract {
    pub(crate) const fn exact_read_declaration(
        detail: AccessShapeDetail,
        lane: AccessLaneClassification,
        expected_counters: ExpectedCounterClass,
    ) -> Self {
        Self {
            detail,
            lane,
            authority_posture: AccessAuthorityPosture::ExactMaterialized,
            stale_disposition: AccessStaleDisposition::ExactOnly,
            expected_counters,
            mutation_shape: None,
            budget_rows: None,
        }
    }

    pub(crate) const fn mutation_path(
        detail: AccessShapeDetail,
        lane: AccessLaneClassification,
        stale_disposition: AccessStaleDisposition,
        expected_counters: ExpectedCounterClass,
        mutation_shape: PhysicalMutationShape,
    ) -> Self {
        Self {
            detail,
            lane,
            authority_posture: AccessAuthorityPosture::MaintenanceMutation,
            stale_disposition,
            expected_counters,
            mutation_shape: Some(mutation_shape),
            budget_rows: None,
        }
    }

    pub(crate) const fn explicit_degraded_exact_scan(
        detail: AccessShapeDetail,
        lane: AccessLaneClassification,
        budget_rows: u64,
    ) -> Self {
        Self {
            detail,
            lane,
            authority_posture: AccessAuthorityPosture::ExplicitDegradedExactScan,
            stale_disposition: AccessStaleDisposition::ExplicitDegradedFallback,
            expected_counters: ExpectedCounterClass::DegradedExactScan,
            mutation_shape: None,
            budget_rows: Some(budget_rows),
        }
    }

    pub const fn shape(self) -> super::shape::AccessShape {
        self.detail.shape()
    }

    pub const fn detail(self) -> AccessShapeDetail {
        self.detail
    }

    pub const fn lane(self) -> AccessLaneClassification {
        self.lane
    }

    pub const fn authority_posture(self) -> AccessAuthorityPosture {
        self.authority_posture
    }

    pub const fn stale_disposition(self) -> AccessStaleDisposition {
        self.stale_disposition
    }

    pub const fn expected_counters(self) -> ExpectedCounterClass {
        self.expected_counters
    }

    pub const fn mutation_shape(self) -> Option<PhysicalMutationShape> {
        self.mutation_shape
    }

    pub const fn budget_rows(self) -> Option<u64> {
        self.budget_rows
    }
}
