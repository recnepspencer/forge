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
    ImplementedBasisObservationScope,
    ImplementedProjectionConsumptionContract,
    ReadExecutionNeighborDeferredUntilCovered,
    InspectionMaterializationNeighborDeferredUntilCovered,
}

impl ForgeQueryIntentAdmissionSupportDetail {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ImplementedRuntimeIntentFloor => "implemented-runtime-intent-floor",
            Self::ImplementedBasisObservationScope => "implemented-basis-observation-scope",
            Self::ImplementedProjectionConsumptionContract => {
                "implemented-projection-consumption-contract"
            }
            Self::ReadExecutionNeighborDeferredUntilCovered => {
                "read-execution-neighbor-deferred-until-covered"
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

const SUPPORT_ROWS: [ForgeQueryIntentAdmissionSupportRow; 6] = [
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
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred,
        ForgeQueryIntentAdmissionSupportPosture::Deferred,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSupportDetail::ReadExecutionNeighborDeferredUntilCovered,
    ),
    ForgeQueryIntentAdmissionSupportRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        ForgeQueryIntentAdmissionSupportPosture::Deferred,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSupportDetail::InspectionMaterializationNeighborDeferredUntilCovered,
    ),
];

pub fn forge_query_intent_admission_support_matrix() -> ForgeQueryIntentAdmissionSupportMatrix {
    ForgeQueryIntentAdmissionSupportMatrix::new(&SUPPORT_ROWS)
}
