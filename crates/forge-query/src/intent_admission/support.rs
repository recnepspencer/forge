use super::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionBoundary,
    ForgeQueryIntentAdmissionFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionSupportPosture {
    Admitted,
    Deferred,
    Unsupported,
}

impl ForgeQueryIntentAdmissionSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionSupportDetail {
    ImplementedRuntimeIntentFloor,
    ImplementedAuthoritativeMutationFloor,
    ImplementedAuthoritativeMutationBatchFloor,
    ImplementedReadExecutionFloor,
    ImplementedLiveReadExecutionFloor,
    ImplementedUnifiedInspectionFloor,
    ImplementedDerivedMaterializationFloor,
    ImplementedDerivedInspectionFloor,
    ImplementedExistingTruthProbeRoutingFloor,
    ImplementedBasisObservationScope,
    ImplementedProjectionConsumptionContract,
    InspectionMaterializationNeighborDeferredUntilCovered,
}

impl ForgeQueryIntentAdmissionSupportDetail {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImplementedRuntimeIntentFloor => "implemented-runtime-intent-floor",
            Self::ImplementedAuthoritativeMutationFloor => {
                "implemented-authoritative-mutation-floor"
            }
            Self::ImplementedAuthoritativeMutationBatchFloor => {
                "implemented-authoritative-mutation-batch-floor"
            }
            Self::ImplementedReadExecutionFloor => "implemented-read-execution-floor",
            Self::ImplementedLiveReadExecutionFloor => "implemented-live-read-execution-floor",
            Self::ImplementedUnifiedInspectionFloor => "implemented-unified-inspection-floor",
            Self::ImplementedDerivedMaterializationFloor => {
                "implemented-derived-materialization-floor"
            }
            Self::ImplementedDerivedInspectionFloor => "implemented-derived-inspection-floor",
            Self::ImplementedExistingTruthProbeRoutingFloor => {
                "implemented-existing-truth-probe-routing-floor"
            }
            Self::ImplementedBasisObservationScope => "implemented-basis-observation-scope",
            Self::ImplementedProjectionConsumptionContract => {
                "implemented-projection-consumption-contract"
            }
            Self::InspectionMaterializationNeighborDeferredUntilCovered => {
                "inspection-materialization-neighbor-deferred-until-covered"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionSupportRow {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    posture: ForgeQueryIntentAdmissionSupportPosture,
    execution_boundary: ForgeQueryIntentAdmissionExecutionBoundary,
    detail: ForgeQueryIntentAdmissionSupportDetail,
}

impl ForgeQueryIntentAdmissionSupportRow {
    pub(crate) const fn new(
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        posture: ForgeQueryIntentAdmissionSupportPosture,
        execution_boundary: ForgeQueryIntentAdmissionExecutionBoundary,
        detail: ForgeQueryIntentAdmissionSupportDetail,
    ) -> Self {
        Self {
            family,
            entrypoint,
            posture,
            execution_boundary,
            detail,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn posture(&self) -> ForgeQueryIntentAdmissionSupportPosture {
        self.posture
    }

    pub fn execution_boundary(&self) -> ForgeQueryIntentAdmissionExecutionBoundary {
        self.execution_boundary
    }

    pub fn detail(&self) -> ForgeQueryIntentAdmissionSupportDetail {
        self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionSupportMatrix {
    rows: &'static [ForgeQueryIntentAdmissionSupportRow],
}

impl ForgeQueryIntentAdmissionSupportMatrix {
    pub(crate) const fn new(rows: &'static [ForgeQueryIntentAdmissionSupportRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryIntentAdmissionSupportRow] {
        self.rows
    }
}

const SUPPORT_ROWS: [ForgeQueryIntentAdmissionSupportRow; 14] = [
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_write_authority_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedAuthoritativeMutationFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_write_authority_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedAuthoritativeMutationBatchFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::BasisUseIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "no-execution-handoff-basis-observation-scope",
        ),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedBasisObservationScope,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "no-execution-handoff-projection-consumption-contract",
        ),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedProjectionConsumptionContract,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_query_runtime_read_execution_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedReadExecutionFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_query_runtime_read_execution_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedReadExecutionFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_query_runtime_read_execution_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedLiveReadExecutionFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_query_runtime_inspection_materialization_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedUnifiedInspectionFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_query_runtime_inspection_materialization_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedDerivedMaterializationFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_query_runtime_inspection_materialization_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedDerivedInspectionFloor,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        ForgeQueryIntentAdmissionSupportPosture::Deferred,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "inspection-materialization-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSupportDetail::InspectionMaterializationNeighborDeferredUntilCovered,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting,
        ForgeQueryIntentAdmissionSupportPosture::Admitted,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_existing_truth_probe_route(),
        ForgeQueryIntentAdmissionSupportDetail::ImplementedExistingTruthProbeRoutingFloor,
    ),
];

pub fn forge_query_intent_admission_support_matrix() -> ForgeQueryIntentAdmissionSupportMatrix {
    ForgeQueryIntentAdmissionSupportMatrix::new(&SUPPORT_ROWS)
}
