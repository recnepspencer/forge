use super::super::ForgeQueryIntentAdmissionFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionSurfaceDescriptor {
    Available(&'static str),
    Deferred(&'static str),
}

impl ForgeQueryIntentAdmissionSurfaceDescriptor {
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
pub enum ForgeQueryIntentAdmissionCoveredEntrypoint {
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

impl ForgeQueryIntentAdmissionCoveredEntrypoint {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecuteIntent => "ForgeQueryRuntime::execute_intent",
            Self::ExecuteNextEffectWriteIntent => {
                "ForgeQueryRuntime::execute_next_effect_write_intent"
            }
            Self::ExecuteScalarWrite => "ForgeQueryRuntime::write",
            Self::ExecuteBatchWrite => "ForgeQueryRuntime::write_batch",
            Self::BasisObservation => {
                "basis_lifecycle::basis_lifecycle().current_head().for_observation()"
            }
            Self::ProjectionConsumption => {
                "projection_consumption::declare_projection_consumption(...)"
            }
            Self::ExecuteReadFamily => "ForgeQueryWorkspace::execute_read_family",
            Self::ExecuteReadFamilyInBasisContext => {
                "ForgeQueryWorkspace::execute_read_family_in_basis_context"
            }
            Self::ExecuteLiveRead => "ForgeQueryWorkspace::read",
            Self::ExecuteUnifiedInspection => "ForgeQueryWorkspace::inspect",
            Self::ExecuteDerivedMaterialization => "ForgeQueryWorkspace::materialize",
            Self::ExecuteDerivedInspection => "ForgeQueryWorkspace::inspect(&derived_view)",
            Self::ExecuteInspectionNeighborDeferred => {
                "ForgeQueryRuntime::inspection neighbor deferred"
            }
            Self::ExecuteExistingTruthProbeRouting => "ForgeQueryRuntime::probe_existing",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionEligibilityAuthority {
    RuntimeIntentAuthorityAdapter,
    RuntimeWriteAuthorityAdapter,
    BasisLifecycleObservationAuthority,
    ProjectionConsumptionEligibilityAuthority,
    ReadCompositionExecutionAuthority,
    InspectionMaterializationExecutionAuthority,
    DeferredInspectionMaterializationAuthority,
    LowerRuntimeCapabilityRoutingAuthority,
}

impl ForgeQueryIntentAdmissionEligibilityAuthority {
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
pub enum ForgeQueryIntentAdmissionPlanKind {
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

impl ForgeQueryIntentAdmissionPlanKind {
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
pub enum ForgeQueryIntentAdmissionExecutionHandoffInventory {
    Available(&'static str),
    NoExecutionHandoff(&'static str),
}

impl ForgeQueryIntentAdmissionExecutionHandoffInventory {
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
pub enum ForgeQueryIntentAdmissionDecisionClass {
    AdvisoryNotYetExercisedOnCoveredEntrypoint,
    ProjectionWarningBearingAdmission,
    InspectionDetailRedactionAdvisory,
    DeferredNeighborSupport,
    AdmissionOrExecutionViolation,
    NeighborUnsupportedUntilCoverage,
}

impl ForgeQueryIntentAdmissionDecisionClass {
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
pub enum ForgeQueryIntentAdmissionResultArtifact {
    ForgeQueryIntentReceipt,
    ForgeQueryEffectIntentReceipt,
    ForgeQueryWriteReceipt,
    ForgeQueryBatchWriteReceipt,
    ForgeQueryReadResult,
    ForgeQueryLiveReadResult,
    ForgeQueryUnifiedInspectionResult,
    ForgeQueryDerivedMaterializationResult,
    ForgeQueryDerivedInspectionResult,
    ForgeQueryExistingTruthProbeResult,
    ScopedObservationBasis,
    MaterializedProjectionContract,
    DeferredInspectionMaterializationArtifact,
}

impl ForgeQueryIntentAdmissionResultArtifact {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ForgeQueryIntentReceipt => "ForgeQueryIntentReceipt",
            Self::ForgeQueryEffectIntentReceipt => "ForgeQueryEffectIntentReceipt",
            Self::ForgeQueryWriteReceipt => "ForgeQueryWriteReceipt",
            Self::ForgeQueryBatchWriteReceipt => "ForgeQueryBatchWriteReceipt",
            Self::ForgeQueryReadResult => "ForgeQueryReadResult",
            Self::ForgeQueryLiveReadResult => "ForgeQueryLiveReadResult",
            Self::ForgeQueryUnifiedInspectionResult => "ForgeQueryUnifiedInspectionResult",
            Self::ForgeQueryDerivedMaterializationResult => {
                "ForgeQueryDerivedMaterializationResult"
            }
            Self::ForgeQueryDerivedInspectionResult => "ForgeQueryDerivedInspectionResult",
            Self::ForgeQueryExistingTruthProbeResult => "ForgeQueryExistingTruthProbeResult",
            Self::ScopedObservationBasis => "ScopedObservationBasis",
            Self::MaterializedProjectionContract => "MaterializedProjectionContract",
            Self::DeferredInspectionMaterializationArtifact => {
                "deferred-inspection-materialization-artifact"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionExecutionSeam {
    BackendIntentAuthorityRoute,
    BackendWriteAuthorityRoute,
    QueryRuntimeReadExecutionRoute,
    QueryRuntimeInspectionMaterializationRoute,
    BackendExistingTruthProbeRoute,
}

impl ForgeQueryIntentAdmissionExecutionSeam {
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
pub enum ForgeQueryIntentAdmissionExecutionBoundary {
    CoveredSeam(ForgeQueryIntentAdmissionExecutionSeam),
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionExecutionBoundary {
    pub const fn covered_backend_intent_authority_route() -> Self {
        Self::CoveredSeam(ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute)
    }

    pub const fn covered_backend_write_authority_route() -> Self {
        Self::CoveredSeam(ForgeQueryIntentAdmissionExecutionSeam::BackendWriteAuthorityRoute)
    }

    pub const fn covered_query_runtime_read_execution_route() -> Self {
        Self::CoveredSeam(ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeReadExecutionRoute)
    }

    pub const fn covered_query_runtime_inspection_materialization_route() -> Self {
        Self::CoveredSeam(
            ForgeQueryIntentAdmissionExecutionSeam::QueryRuntimeInspectionMaterializationRoute,
        )
    }

    pub const fn covered_backend_existing_truth_probe_route() -> Self {
        Self::CoveredSeam(ForgeQueryIntentAdmissionExecutionSeam::BackendExistingTruthProbeRoute)
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

    pub fn execution_seam(self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        match self {
            Self::CoveredSeam(seam) => Some(seam),
            Self::DeferredNeighbor(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionCoverageStatus {
    Implemented,
    PlannedNeighbor,
}

impl ForgeQueryIntentAdmissionCoverageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::PlannedNeighbor => "planned-neighbor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionCoverageRow {
    family: ForgeQueryIntentAdmissionFamily,
    entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
    execution_boundary: ForgeQueryIntentAdmissionExecutionBoundary,
    status: ForgeQueryIntentAdmissionCoverageStatus,
    eligibility_authority: ForgeQueryIntentAdmissionEligibilityAuthority,
    admitted_plan_kind: ForgeQueryIntentAdmissionPlanKind,
    admitted_execution_handoff: ForgeQueryIntentAdmissionExecutionHandoffInventory,
    advisory_decision_class: ForgeQueryIntentAdmissionDecisionClass,
    violation_decision_class: ForgeQueryIntentAdmissionDecisionClass,
    result_artifact: ForgeQueryIntentAdmissionResultArtifact,
    raw_authoring_constructor: ForgeQueryIntentAdmissionSurfaceDescriptor,
    common_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
    advanced_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
}

impl ForgeQueryIntentAdmissionCoverageRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        execution_boundary: ForgeQueryIntentAdmissionExecutionBoundary,
        status: ForgeQueryIntentAdmissionCoverageStatus,
        eligibility_authority: ForgeQueryIntentAdmissionEligibilityAuthority,
        admitted_plan_kind: ForgeQueryIntentAdmissionPlanKind,
        admitted_execution_handoff: ForgeQueryIntentAdmissionExecutionHandoffInventory,
        advisory_decision_class: ForgeQueryIntentAdmissionDecisionClass,
        violation_decision_class: ForgeQueryIntentAdmissionDecisionClass,
        result_artifact: ForgeQueryIntentAdmissionResultArtifact,
        raw_authoring_constructor: ForgeQueryIntentAdmissionSurfaceDescriptor,
        common_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
        advanced_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor,
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

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.family
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.entrypoint
    }

    pub fn execution_boundary(&self) -> ForgeQueryIntentAdmissionExecutionBoundary {
        self.execution_boundary
    }

    pub fn execution_seam(&self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        self.execution_boundary.execution_seam()
    }

    pub fn status(&self) -> ForgeQueryIntentAdmissionCoverageStatus {
        self.status
    }

    pub fn eligibility_authority(&self) -> ForgeQueryIntentAdmissionEligibilityAuthority {
        self.eligibility_authority
    }

    pub fn admitted_plan_kind(&self) -> ForgeQueryIntentAdmissionPlanKind {
        self.admitted_plan_kind
    }

    pub fn admitted_execution_handoff(&self) -> ForgeQueryIntentAdmissionExecutionHandoffInventory {
        self.admitted_execution_handoff
    }

    pub fn advisory_decision_class(&self) -> ForgeQueryIntentAdmissionDecisionClass {
        self.advisory_decision_class
    }

    pub fn violation_decision_class(&self) -> ForgeQueryIntentAdmissionDecisionClass {
        self.violation_decision_class
    }

    pub fn result_artifact(&self) -> ForgeQueryIntentAdmissionResultArtifact {
        self.result_artifact
    }

    pub fn raw_authoring_constructor(&self) -> ForgeQueryIntentAdmissionSurfaceDescriptor {
        self.raw_authoring_constructor
    }

    pub fn common_path_front_door(&self) -> ForgeQueryIntentAdmissionSurfaceDescriptor {
        self.common_path_front_door
    }

    pub fn advanced_path_front_door(&self) -> ForgeQueryIntentAdmissionSurfaceDescriptor {
        self.advanced_path_front_door
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionCoverageInventory {
    rows: &'static [ForgeQueryIntentAdmissionCoverageRow],
}

impl ForgeQueryIntentAdmissionCoverageInventory {
    pub(crate) const fn new(rows: &'static [ForgeQueryIntentAdmissionCoverageRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryIntentAdmissionCoverageRow] {
        self.rows
    }
}
