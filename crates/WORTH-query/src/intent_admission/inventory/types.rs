use super::super::WorthQueryIntentAdmissionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionSurfaceDescriptor {
    Available(&'static str),
    Deferred(&'static str),
}

impl WorthQueryIntentAdmissionSurfaceDescriptor {
    pub const fn available(label: &'static str) -> Self {
        Self::Available(label)
    }

    pub const fn deferred(reason: &'static str) -> Self {
        Self::Deferred(reason)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Available(label) | Self::Deferred(label) => label,
        }
    }

    pub fn deferred_reason(self) -> Option<&'static str> {
        match self {
            Self::Available(_) => None,
            Self::Deferred(reason) => Some(reason),
        }
    }
}

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
pub enum WorthQueryIntentAdmissionEligibilityAuthority {
    RuntimeIntentAuthorityAdapter,
    RuntimeWriteAuthorityAdapter,
    BasisLifecycleObservationAuthority,
    ProjectionConsumptionEligibilityAuthority,
    ReadCompositionExecutionAuthority,
    InspectionMaterializationExecutionAuthority,
    DeferredInspectionMaterializationAuthority,
    LowerRuntimeCapabilityRoutingAuthority,
}

impl WorthQueryIntentAdmissionEligibilityAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeIntentAuthorityAdapter => "runtime-intent-authority-adapter",
            Self::RuntimeWriteAuthorityAdapter => "runtime-write-authority-adapter",
            Self::BasisLifecycleObservationAuthority => "basis-lifecycle-observation-authority",
            Self::ProjectionConsumptionEligibilityAuthority => {
                "projection-consumption-eligibility-authority"
            }
            Self::ReadCompositionExecutionAuthority => "read-composition-execution-authority",
            Self::InspectionMaterializationExecutionAuthority => {
                "inspection-materialization-execution-authority"
            }
            Self::DeferredInspectionMaterializationAuthority => {
                "deferred-inspection-materialization-authority"
            }
            Self::LowerRuntimeCapabilityRoutingAuthority => {
                "lower-runtime-capability-routing-authority"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionPlanKind {
    AuthoritativeIntentExecutionPlan,
    EffectTriggeredIntentExecutionPlan,
    AuthoritativeMutationExecutionPlan,
    AuthoritativeMutationBatchExecutionPlan,
    BasisObservationPlan,
    ProjectionConsumptionPlan,
    ReadExecutionPlan,
    UnifiedInspectionExecutionPlan,
    InspectionMaterializationExecutionPlan,
    DeferredInspectionMaterializationPlan,
    ExistingTruthProbeRoutingPlan,
}

impl WorthQueryIntentAdmissionPlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeIntentExecutionPlan => "authoritative-intent-execution-plan",
            Self::EffectTriggeredIntentExecutionPlan => "effect-triggered-intent-execution-plan",
            Self::AuthoritativeMutationExecutionPlan => "authoritative-mutation-execution-plan",
            Self::AuthoritativeMutationBatchExecutionPlan => {
                "authoritative-mutation-batch-execution-plan"
            }
            Self::BasisObservationPlan => "basis-observation-plan",
            Self::ProjectionConsumptionPlan => "projection-consumption-plan",
            Self::ReadExecutionPlan => "read-execution-plan",
            Self::UnifiedInspectionExecutionPlan => "unified-inspection-execution-plan",
            Self::InspectionMaterializationExecutionPlan => {
                "inspection-materialization-execution-plan"
            }
            Self::DeferredInspectionMaterializationPlan => {
                "deferred-inspection-materialization-plan"
            }
            Self::ExistingTruthProbeRoutingPlan => "existing-truth-probe-routing-plan",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionExecutionHandoffInventory {
    Available(&'static str),
    NoExecutionHandoff(&'static str),
}

impl WorthQueryIntentAdmissionExecutionHandoffInventory {
    pub const fn available(type_name: &'static str) -> Self {
        Self::Available(type_name)
    }

    pub const fn no_execution_handoff(reason: &'static str) -> Self {
        Self::NoExecutionHandoff(reason)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Available(label) | Self::NoExecutionHandoff(label) => label,
        }
    }

    pub fn no_execution_handoff_reason(self) -> Option<&'static str> {
        match self {
            Self::Available(_) => None,
            Self::NoExecutionHandoff(reason) => Some(reason),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionDecisionClass {
    AdvisoryNotYetExercisedOnCoveredEntrypoint,
    ProjectionWarningBearingAdmission,
    InspectionDetailRedactionAdvisory,
    DeferredNeighborSupport,
    AdmissionOrExecutionViolation,
    NeighborUnsupportedUntilCoverage,
}

impl WorthQueryIntentAdmissionDecisionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryNotYetExercisedOnCoveredEntrypoint => {
                "advisory-not-yet-exercised-on-covered-entrypoint"
            }
            Self::ProjectionWarningBearingAdmission => "projection-warning-bearing-admission",
            Self::InspectionDetailRedactionAdvisory => "inspection-detail-redaction-advisory",
            Self::DeferredNeighborSupport => "deferred-neighbor-support",
            Self::AdmissionOrExecutionViolation => "admission-or-execution-violation",
            Self::NeighborUnsupportedUntilCoverage => "neighbor-unsupported-until-coverage",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionResultArtifact {
    WorthQueryIntentReceipt,
    WorthQueryEffectIntentReceipt,
    WorthQueryWriteReceipt,
    WorthQueryBatchWriteReceipt,
    WorthQueryReadResult,
    WorthQueryLiveReadResult,
    WorthQueryUnifiedInspectionResult,
    WorthQueryDerivedMaterializationResult,
    WorthQueryDerivedInspectionResult,
    WorthQueryExistingTruthProbeResult,
    ScopedObservationBasis,
    MaterializedProjectionContract,
    DeferredInspectionMaterializationArtifact,
}

impl WorthQueryIntentAdmissionResultArtifact {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorthQueryIntentReceipt => "WorthQueryIntentReceipt",
            Self::WorthQueryEffectIntentReceipt => "WorthQueryEffectIntentReceipt",
            Self::WorthQueryWriteReceipt => "WorthQueryWriteReceipt",
            Self::WorthQueryBatchWriteReceipt => "WorthQueryBatchWriteReceipt",
            Self::WorthQueryReadResult => "WorthQueryReadResult",
            Self::WorthQueryLiveReadResult => "WorthQueryLiveReadResult",
            Self::WorthQueryUnifiedInspectionResult => "WorthQueryUnifiedInspectionResult",
            Self::WorthQueryDerivedMaterializationResult => {
                "WorthQueryDerivedMaterializationResult"
            }
            Self::WorthQueryDerivedInspectionResult => "WorthQueryDerivedInspectionResult",
            Self::WorthQueryExistingTruthProbeResult => "WorthQueryExistingTruthProbeResult",
            Self::ScopedObservationBasis => "ScopedObservationBasis",
            Self::MaterializedProjectionContract => "MaterializedProjectionContract",
            Self::DeferredInspectionMaterializationArtifact => {
                "deferred-inspection-materialization-artifact"
            }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionCoverageStatus {
    Implemented,
    PlannedNeighbor,
}

impl WorthQueryIntentAdmissionCoverageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::PlannedNeighbor => "planned-neighbor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionCoverageRow {
    family: WorthQueryIntentAdmissionFamily,
    entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
    execution_boundary: WorthQueryIntentAdmissionExecutionBoundary,
    status: WorthQueryIntentAdmissionCoverageStatus,
    eligibility_authority: WorthQueryIntentAdmissionEligibilityAuthority,
    admitted_plan_kind: WorthQueryIntentAdmissionPlanKind,
    admitted_execution_handoff: WorthQueryIntentAdmissionExecutionHandoffInventory,
    advisory_decision_class: WorthQueryIntentAdmissionDecisionClass,
    violation_decision_class: WorthQueryIntentAdmissionDecisionClass,
    result_artifact: WorthQueryIntentAdmissionResultArtifact,
    raw_authoring_constructor: WorthQueryIntentAdmissionSurfaceDescriptor,
    common_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
    advanced_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
}

impl WorthQueryIntentAdmissionCoverageRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        execution_boundary: WorthQueryIntentAdmissionExecutionBoundary,
        status: WorthQueryIntentAdmissionCoverageStatus,
        eligibility_authority: WorthQueryIntentAdmissionEligibilityAuthority,
        admitted_plan_kind: WorthQueryIntentAdmissionPlanKind,
        admitted_execution_handoff: WorthQueryIntentAdmissionExecutionHandoffInventory,
        advisory_decision_class: WorthQueryIntentAdmissionDecisionClass,
        violation_decision_class: WorthQueryIntentAdmissionDecisionClass,
        result_artifact: WorthQueryIntentAdmissionResultArtifact,
        raw_authoring_constructor: WorthQueryIntentAdmissionSurfaceDescriptor,
        common_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
        advanced_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor,
    ) -> Self {
        Self {
            family,
            entrypoint,
            execution_boundary,
            status,
            eligibility_authority,
            admitted_plan_kind,
            admitted_execution_handoff,
            advisory_decision_class,
            violation_decision_class,
            result_artifact,
            raw_authoring_constructor,
            common_path_front_door,
            advanced_path_front_door,
        }
    }

    pub fn family(&self) -> WorthQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> WorthQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn execution_boundary(&self) -> WorthQueryIntentAdmissionExecutionBoundary {
        self.execution_boundary
    }

    pub fn execution_seam(&self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        self.execution_boundary.execution_seam()
    }

    pub fn status(&self) -> WorthQueryIntentAdmissionCoverageStatus {
        self.status
    }

    pub fn eligibility_authority(&self) -> WorthQueryIntentAdmissionEligibilityAuthority {
        self.eligibility_authority
    }

    pub fn admitted_plan_kind(&self) -> WorthQueryIntentAdmissionPlanKind {
        self.admitted_plan_kind
    }

    pub fn admitted_execution_handoff(&self) -> WorthQueryIntentAdmissionExecutionHandoffInventory {
        self.admitted_execution_handoff
    }

    pub fn advisory_decision_class(&self) -> WorthQueryIntentAdmissionDecisionClass {
        self.advisory_decision_class
    }

    pub fn violation_decision_class(&self) -> WorthQueryIntentAdmissionDecisionClass {
        self.violation_decision_class
    }

    pub fn result_artifact(&self) -> WorthQueryIntentAdmissionResultArtifact {
        self.result_artifact
    }

    pub fn raw_authoring_constructor(&self) -> WorthQueryIntentAdmissionSurfaceDescriptor {
        self.raw_authoring_constructor
    }

    pub fn common_path_front_door(&self) -> WorthQueryIntentAdmissionSurfaceDescriptor {
        self.common_path_front_door
    }

    pub fn advanced_path_front_door(&self) -> WorthQueryIntentAdmissionSurfaceDescriptor {
        self.advanced_path_front_door
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionCoverageInventory {
    rows: &'static [WorthQueryIntentAdmissionCoverageRow],
}

impl WorthQueryIntentAdmissionCoverageInventory {
    pub(crate) const fn new(rows: &'static [WorthQueryIntentAdmissionCoverageRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [WorthQueryIntentAdmissionCoverageRow] {
        self.rows
    }
}
