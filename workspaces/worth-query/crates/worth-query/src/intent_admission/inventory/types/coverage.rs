use super::super::super::WorthQueryIntentAdmissionFamily;
use super::admission::{
    WorthQueryIntentAdmissionDecisionClass, WorthQueryIntentAdmissionEligibilityAuthority,
    WorthQueryIntentAdmissionPlanKind, WorthQueryIntentAdmissionResultArtifact,
};
use super::entrypoints::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionExecutionBoundary,
    WorthQueryIntentAdmissionExecutionSeam,
};
use super::surface::{
    WorthQueryIntentAdmissionExecutionHandoffInventory, WorthQueryIntentAdmissionSurfaceDescriptor,
};

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
