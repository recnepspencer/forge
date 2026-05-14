use super::ForgeQueryIntentAdmissionFamily;

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
    ExecuteReadNeighborDeferred,
    ExecuteInspectionNeighborDeferred,
}

impl ForgeQueryIntentAdmissionCoveredEntrypoint {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecuteIntent => "ForgeQueryRuntime::execute_intent",
            Self::ExecuteNextEffectWriteIntent => {
                "ForgeQueryRuntime::execute_next_effect_write_intent"
            }
            Self::ExecuteReadNeighborDeferred => "ForgeQueryRuntime::read neighbor deferred",
            Self::ExecuteInspectionNeighborDeferred => {
                "ForgeQueryRuntime::inspection neighbor deferred"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionEligibilityAuthority {
    RuntimeIntentAuthorityAdapter,
    DeferredReadExecutionAuthority,
    DeferredInspectionMaterializationAuthority,
}

impl ForgeQueryIntentAdmissionEligibilityAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeIntentAuthorityAdapter => "runtime-intent-authority-adapter",
            Self::DeferredReadExecutionAuthority => "deferred-read-execution-authority",
            Self::DeferredInspectionMaterializationAuthority => {
                "deferred-inspection-materialization-authority"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionPlanKind {
    AuthoritativeIntentExecutionPlan,
    EffectTriggeredIntentExecutionPlan,
    DeferredReadExecutionPlan,
    DeferredInspectionMaterializationPlan,
}

impl ForgeQueryIntentAdmissionPlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeIntentExecutionPlan => "authoritative-intent-execution-plan",
            Self::EffectTriggeredIntentExecutionPlan => "effect-triggered-intent-execution-plan",
            Self::DeferredReadExecutionPlan => "deferred-read-execution-plan",
            Self::DeferredInspectionMaterializationPlan => {
                "deferred-inspection-materialization-plan"
            }
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
    DeferredReadExecutionArtifact,
    DeferredInspectionMaterializationArtifact,
}

impl ForgeQueryIntentAdmissionResultArtifact {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ForgeQueryIntentReceipt => "ForgeQueryIntentReceipt",
            Self::ForgeQueryEffectIntentReceipt => "ForgeQueryEffectIntentReceipt",
            Self::DeferredReadExecutionArtifact => "deferred-read-execution-artifact",
            Self::DeferredInspectionMaterializationArtifact => {
                "deferred-inspection-materialization-artifact"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionExecutionSeam {
    BackendIntentAuthorityRoute,
}

impl ForgeQueryIntentAdmissionExecutionSeam {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackendIntentAuthorityRoute => "backend-intent-authority-route",
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

const INTENT_ADMISSION_ROWS: [ForgeQueryIntentAdmissionCoverageRow; 4] = [
    ForgeQueryIntentAdmissionCoverageRow::new(
        ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        ForgeQueryIntentAdmissionCoverageStatus::Implemented,
        ForgeQueryIntentAdmissionEligibilityAuthority::RuntimeIntentAuthorityAdapter,
        ForgeQueryIntentAdmissionPlanKind::AuthoritativeIntentExecutionPlan,
        ForgeQueryIntentAdmissionExecutionHandoffInventory::available(
            "ForgeQueryAdmittedIntentExecutionHandoff",
        ),
        ForgeQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint,
        ForgeQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation,
        ForgeQueryIntentAdmissionResultArtifact::ForgeQueryIntentReceipt,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.intent(declaration).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.intent(declaration).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionCoverageRow::new(
        ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent,
        ForgeQueryIntentAdmissionExecutionBoundary::covered_backend_intent_authority_route(),
        ForgeQueryIntentAdmissionCoverageStatus::Implemented,
        ForgeQueryIntentAdmissionEligibilityAuthority::RuntimeIntentAuthorityAdapter,
        ForgeQueryIntentAdmissionPlanKind::EffectTriggeredIntentExecutionPlan,
        ForgeQueryIntentAdmissionExecutionHandoffInventory::available(
            "ForgeQueryAdmittedIntentExecutionHandoff",
        ),
        ForgeQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint,
        ForgeQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation,
        ForgeQueryIntentAdmissionResultArtifact::ForgeQueryEffectIntentReceipt,
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "ForgeQueryRawIntentAdmissionRequest::effect_runtime_entrypoint(...)",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.next_effect_write_intent(&effect, version, contract).execute()",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::available(
            "runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()",
        ),
    ),
    ForgeQueryIntentAdmissionCoverageRow::new(
        ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionCoverageStatus::PlannedNeighbor,
        ForgeQueryIntentAdmissionEligibilityAuthority::DeferredReadExecutionAuthority,
        ForgeQueryIntentAdmissionPlanKind::DeferredReadExecutionPlan,
        ForgeQueryIntentAdmissionExecutionHandoffInventory::no_execution_handoff(
            "read-execution-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionDecisionClass::DeferredNeighborSupport,
        ForgeQueryIntentAdmissionDecisionClass::NeighborUnsupportedUntilCoverage,
        ForgeQueryIntentAdmissionResultArtifact::DeferredReadExecutionArtifact,
        ForgeQueryIntentAdmissionSurfaceDescriptor::deferred(
            "read-execution-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::deferred(
            "read-execution-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::deferred(
            "read-execution-neighbor-deferred-until-covered",
        ),
    ),
    ForgeQueryIntentAdmissionCoverageRow::new(
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred,
        ForgeQueryIntentAdmissionExecutionBoundary::deferred_neighbor(
            "neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionCoverageStatus::PlannedNeighbor,
        ForgeQueryIntentAdmissionEligibilityAuthority::DeferredInspectionMaterializationAuthority,
        ForgeQueryIntentAdmissionPlanKind::DeferredInspectionMaterializationPlan,
        ForgeQueryIntentAdmissionExecutionHandoffInventory::no_execution_handoff(
            "inspection-materialization-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionDecisionClass::DeferredNeighborSupport,
        ForgeQueryIntentAdmissionDecisionClass::NeighborUnsupportedUntilCoverage,
        ForgeQueryIntentAdmissionResultArtifact::DeferredInspectionMaterializationArtifact,
        ForgeQueryIntentAdmissionSurfaceDescriptor::deferred(
            "inspection-materialization-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::deferred(
            "inspection-materialization-neighbor-deferred-until-covered",
        ),
        ForgeQueryIntentAdmissionSurfaceDescriptor::deferred(
            "inspection-materialization-neighbor-deferred-until-covered",
        ),
    ),
];

pub fn forge_query_intent_admission_coverage_inventory(
) -> ForgeQueryIntentAdmissionCoverageInventory {
    ForgeQueryIntentAdmissionCoverageInventory::new(&INTENT_ADMISSION_ROWS)
}
