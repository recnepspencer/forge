use super::detail::S8AccessShapeDetail;
use super::lane::S8AccessLaneClassification;
use crate::maintenance::S8PhysicalMutationShape;
use crate::materialization::S8LayoutCoverageWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessAuthorityPosture {
    ExactMaterialized,
    ExplicitDegradedExactScan,
    MaintenanceMutation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessStaleDisposition {
    ExactOnly,
    RebindBeforeExecution,
    ExplicitDegradedFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8ExpectedCounterClass {
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
pub struct S8AccessShapeContract {
    detail: S8AccessShapeDetail,
    lane: S8AccessLaneClassification,
    authority_posture: S8AccessAuthorityPosture,
    stale_disposition: S8AccessStaleDisposition,
    expected_counters: S8ExpectedCounterClass,
    coverage: Option<S8LayoutCoverageWitness>,
    mutation_shape: Option<S8PhysicalMutationShape>,
    budget_rows: Option<u64>,
}

impl S8AccessShapeContract {
    pub(crate) const fn exact_read(
        detail: S8AccessShapeDetail,
        lane: S8AccessLaneClassification,
        expected_counters: S8ExpectedCounterClass,
        coverage: S8LayoutCoverageWitness,
    ) -> Self {
        Self {
            detail,
            lane,
            authority_posture: S8AccessAuthorityPosture::ExactMaterialized,
            stale_disposition: S8AccessStaleDisposition::ExactOnly,
            expected_counters,
            coverage: Some(coverage),
            mutation_shape: None,
            budget_rows: None,
        }
    }

    pub(crate) const fn mutation_path(
        detail: S8AccessShapeDetail,
        lane: S8AccessLaneClassification,
        stale_disposition: S8AccessStaleDisposition,
        expected_counters: S8ExpectedCounterClass,
        mutation_shape: S8PhysicalMutationShape,
    ) -> Self {
        Self {
            detail,
            lane,
            authority_posture: S8AccessAuthorityPosture::MaintenanceMutation,
            stale_disposition,
            expected_counters,
            coverage: None,
            mutation_shape: Some(mutation_shape),
            budget_rows: None,
        }
    }

    pub(crate) const fn explicit_degraded_exact_scan(
        detail: S8AccessShapeDetail,
        lane: S8AccessLaneClassification,
        coverage: S8LayoutCoverageWitness,
        budget_rows: u64,
    ) -> Self {
        Self {
            detail,
            lane,
            authority_posture: S8AccessAuthorityPosture::ExplicitDegradedExactScan,
            stale_disposition: S8AccessStaleDisposition::ExplicitDegradedFallback,
            expected_counters: S8ExpectedCounterClass::DegradedExactScan,
            coverage: Some(coverage),
            mutation_shape: None,
            budget_rows: Some(budget_rows),
        }
    }

    pub const fn shape(self) -> super::shape::S8AccessShape {
        self.detail.shape()
    }

    pub const fn detail(self) -> S8AccessShapeDetail {
        self.detail
    }

    pub const fn lane(self) -> S8AccessLaneClassification {
        self.lane
    }

    pub const fn authority_posture(self) -> S8AccessAuthorityPosture {
        self.authority_posture
    }

    pub const fn stale_disposition(self) -> S8AccessStaleDisposition {
        self.stale_disposition
    }

    pub const fn expected_counters(self) -> S8ExpectedCounterClass {
        self.expected_counters
    }

    pub const fn coverage(self) -> Option<S8LayoutCoverageWitness> {
        self.coverage
    }

    pub const fn mutation_shape(self) -> Option<S8PhysicalMutationShape> {
        self.mutation_shape
    }

    pub const fn budget_rows(self) -> Option<u64> {
        self.budget_rows
    }
}
