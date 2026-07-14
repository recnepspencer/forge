use super::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionBoundary,
    WorthQueryIntentAdmissionFamily,
};

pub(crate) const INTENT_ADMISSION_SUPPORT_MODULE_ROOT: &str = "intent_admission/support/mod.rs";
pub(crate) const INTENT_ADMISSION_SUPPORT_CHILD_MODULES: &[&str] = &[];
pub(crate) const INTENT_ADMISSION_SUPPORT_EXPORTED_SURFACE: &[&str] = &[
    "WorthQueryIntentAdmissionSupportPosture",
    "WorthQueryIntentAdmissionSupportDetail",
    "WorthQueryIntentAdmissionSupportRow",
    "WorthQueryIntentAdmissionSupportMatrix",
    "worth_query_intent_admission_support_matrix",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionSupportPosture {
    Admitted,
    Deferred,
    Unsupported,
}

impl WorthQueryIntentAdmissionSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred => "deferred",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionSupportDetail {
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

impl WorthQueryIntentAdmissionSupportDetail {
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
pub struct WorthQueryIntentAdmissionSupportRow {
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    posture: WorthQueryIntentAdmissionSupportPosture,
    execution_boundary: WorthQueryIntentAdmissionExecutionBoundary,
    detail: WorthQueryIntentAdmissionSupportDetail,
}

impl WorthQueryIntentAdmissionSupportRow {
    pub(crate) const fn new(
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        posture: WorthQueryIntentAdmissionSupportPosture,
        execution_boundary: WorthQueryIntentAdmissionExecutionBoundary,
        detail: WorthQueryIntentAdmissionSupportDetail,
    ) -> Self {
        Self {
            family,
            entrypoint,
            posture,
            execution_boundary,
            detail,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn posture(&self) -> WorthQueryIntentAdmissionSupportPosture {
        self.posture
    }

    pub fn execution_boundary(&self) -> WorthQueryIntentAdmissionExecutionBoundary {
        self.execution_boundary
    }

    pub fn detail(&self) -> WorthQueryIntentAdmissionSupportDetail {
        self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionSupportMatrix {
    rows: &'static [WorthQueryIntentAdmissionSupportRow],
}

impl WorthQueryIntentAdmissionSupportMatrix {
    pub(crate) const fn new(rows: &'static [WorthQueryIntentAdmissionSupportRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [WorthQueryIntentAdmissionSupportRow] {
        self.rows
    }
}

const SUPPORT_ROWS: [WorthQueryIntentAdmissionSupportRow; 14] = [
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteScalarWrite,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_backend_write_authority_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedAuthoritativeMutationFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::AuthoritativeMutationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteBatchWrite,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_backend_write_authority_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedAuthoritativeMutationBatchFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::BasisUseIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::BasisObservation,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "no-execution-handoff-basis-observation-scope",
        ),
        WorthQueryIntentAdmissionSupportDetail::ImplementedBasisObservationScope,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::ProjectionConsumptionIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "no-execution-handoff-projection-consumption-contract",
        ),
        WorthQueryIntentAdmissionSupportDetail::ImplementedProjectionConsumptionContract,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::ReadExecutionIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamily,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_query_runtime_read_execution_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedReadExecutionFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::ReadExecutionIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteReadFamilyInBasisContext,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_query_runtime_read_execution_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedReadExecutionFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::ReadExecutionIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_query_runtime_read_execution_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedLiveReadExecutionFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_query_runtime_inspection_materialization_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedUnifiedInspectionFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedMaterialization,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_query_runtime_inspection_materialization_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedDerivedMaterializationFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteDerivedInspection,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_query_runtime_inspection_materialization_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedDerivedInspectionFloor,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        WorthQueryIntentAdmissionSupportPosture::Deferred,
        WorthQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "inspection-materialization-neighbor-deferred-until-covered",
        ),
        WorthQueryIntentAdmissionSupportDetail::InspectionMaterializationNeighborDeferredUntilCovered,
    ),
    WorthQueryIntentAdmissionSupportRow::new(
        WorthQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent,
        WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting,
        WorthQueryIntentAdmissionSupportPosture::Admitted,
        WorthQueryIntentAdmissionExecutionBoundary::covered_backend_existing_truth_probe_route(),
        WorthQueryIntentAdmissionSupportDetail::ImplementedExistingTruthProbeRoutingFloor,
    ),
];

pub fn worth_query_intent_admission_support_matrix() -> WorthQueryIntentAdmissionSupportMatrix {
    WorthQueryIntentAdmissionSupportMatrix::new(&SUPPORT_ROWS)
}
