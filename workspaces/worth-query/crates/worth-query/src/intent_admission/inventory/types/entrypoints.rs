#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionCoveredEntrypoint {
    ExecuteIntent,
    ExecuteNextEffectWriteIntent,
    ExecuteScalarWrite,
    ExecuteBatchWrite,
    BasisObservation,
    ProjectionConsumption,
    ExecuteReadFamily,
    ExecuteReadFamilyInBasisContext,
    ExecuteLiveRead,
    ExecuteUnifiedInspection,
    ExecuteDerivedMaterialization,
    ExecuteDerivedInspection,
    ExecuteInspectionNeighborDeferred,
    ExecuteExistingTruthProbeRouting,
}

impl WorthQueryIntentAdmissionCoveredEntrypoint {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecuteIntent => "WorthQueryRuntime::execute_intent",
            Self::ExecuteNextEffectWriteIntent => {
                "WorthQueryRuntime::execute_next_effect_write_intent"
            }
            Self::ExecuteScalarWrite => "WorthQueryRuntime::write",
            Self::ExecuteBatchWrite => "WorthQueryRuntime::write_batch",
            Self::BasisObservation => {
                "basis_lifecycle::basis_lifecycle().current_head().for_observation()"
            }
            Self::ProjectionConsumption => {
                "projection_consumption::declare_projection_consumption(...)"
            }
            Self::ExecuteReadFamily => "WorthQueryWorkspace::execute_read_family",
            Self::ExecuteReadFamilyInBasisContext => {
                "WorthQueryWorkspace::execute_read_family_in_basis_context"
            }
            Self::ExecuteLiveRead => "WorthQueryWorkspace::read",
            Self::ExecuteUnifiedInspection => "WorthQueryWorkspace::inspect",
            Self::ExecuteDerivedMaterialization => "WorthQueryWorkspace::materialize",
            Self::ExecuteDerivedInspection => "WorthQueryWorkspace::inspect(&derived_view)",
            Self::ExecuteInspectionNeighborDeferred => {
                "WorthQueryRuntime::inspection neighbor deferred"
            }
            Self::ExecuteExistingTruthProbeRouting => "WorthQueryRuntime::probe_existing",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionExecutionSeam {
    BackendIntentAuthorityRoute,
    BackendWriteAuthorityRoute,
    QueryRuntimeReadExecutionRoute,
    QueryRuntimeInspectionMaterializationRoute,
    BackendExistingTruthProbeRoute,
}

impl WorthQueryIntentAdmissionExecutionSeam {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackendIntentAuthorityRoute => "backend-intent-authority-route",
            Self::BackendWriteAuthorityRoute => "backend-write-authority-route",
            Self::QueryRuntimeReadExecutionRoute => "query-runtime-read-execution-route",
            Self::QueryRuntimeInspectionMaterializationRoute => {
                "query-runtime-inspection-materialization-route"
            }
            Self::BackendExistingTruthProbeRoute => "backend-existing-truth-probe-route",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionExecutionBoundary {
    CoveredSeam(WorthQueryIntentAdmissionExecutionSeam),
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionExecutionBoundary {
    pub const fn covered_backend_intent_authority_route() -> Self {
        Self::CoveredSeam(WorthQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute)
    }

    pub const fn covered_backend_write_authority_route() -> Self {
        Self::CoveredSeam(WorthQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute)
    }

    pub const fn covered_query_runtime_read_execution_route() -> Self {
        Self::CoveredSeam(WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute)
    }

    pub const fn covered_query_runtime_inspection_materialization_route() -> Self {
        Self::CoveredSeam(
            WorthQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
        )
    }

    pub const fn covered_backend_existing_truth_probe_route() -> Self {
        Self::CoveredSeam(WorthQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute)
    }

    pub const fn deferred_neighbor(reason: &'static str) -> Self {
        Self::DeferredNeighbor(reason)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoveredSeam(seam) => seam.as_str(),
            Self::DeferredNeighbor(reason) => reason,
        }
    }

    pub fn execution_seam(self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        match self {
            Self::CoveredSeam(seam) => Some(seam),
            Self::DeferredNeighbor(_) => None,
        }
    }
}
