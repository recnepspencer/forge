use worth_ui::facade::WorthUiRuntimeFactId;

use crate::reload::{ValidationHeaderRebindEvidence, ValidationPageHostRebindEvidence};
use crate::storm_proof::ValidationMixedReloadStormProjectionRoster;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormProof {
    pub(super) scenario_digest: u64,
    pub(super) posture: ValidationMixedReloadStormPosture,
    pub(super) steps: Vec<ValidationMixedReloadStormStep>,
    pub(super) projection_counters: ValidationMixedReloadStormProjectionCounters,
    pub(super) projection_roster: ValidationMixedReloadStormProjectionRoster,
    pub(super) final_active_artifact_digest: u64,
    pub(super) final_active_plan_digest: u64,
    pub(super) final_capability_snapshot_digest: u64,
    pub(super) final_authoring_snapshot_digest: Option<u64>,
    pub(super) final_last_valid_artifact_digest: u64,
    pub(super) final_last_valid_plan_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationMixedReloadStormBuildDenial {
    ScenarioNotQualified,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormPosture {
    pub(super) activated_step_count: usize,
    pub(super) equivalent_step_count: usize,
    pub(super) denied_step_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationMixedReloadStormStep {
    pub(super) family: ValidationMixedReloadStormFamily,
    pub(super) status: ValidationMixedReloadStormStatus,
    pub(super) changed_facts: Vec<WorthUiRuntimeFactId>,
    pub(super) denial_detail: Option<String>,
    pub(super) header_rebind: Option<ValidationHeaderRebindEvidence>,
    pub(super) page_host_rebind: Option<ValidationPageHostRebindEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationMixedReloadStormFamily {
    Source,
    Theme,
    Command,
    Component,
    CommandProjection,
    Appearance,
    Density,
    Input,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationMixedReloadStormStatus {
    Activated,
    EquivalentNoOp,
    Denied,
    ReadyForFrameBoundary,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationMixedReloadStormProjectionCounters {
    pub(super) inspected_projection_count: usize,
    pub(super) dependency_intersection_count: usize,
    pub(super) rebuild_attempt_count: usize,
    pub(super) preserved_frame_count: usize,
    pub(super) denied_frame_count: usize,
    pub(super) rebuilt_frame_count: usize,
}

impl ValidationMixedReloadStormProof {
    pub fn scenario_digest(&self) -> u64 {
        self.scenario_digest
    }

    pub fn posture(&self) -> ValidationMixedReloadStormPosture {
        self.posture
    }

    pub fn steps(&self) -> &[ValidationMixedReloadStormStep] {
        &self.steps
    }

    pub fn projection_counters(&self) -> ValidationMixedReloadStormProjectionCounters {
        self.projection_counters
    }

    pub fn projection_roster(&self) -> &ValidationMixedReloadStormProjectionRoster {
        &self.projection_roster
    }

    pub fn final_active_artifact_digest(&self) -> u64 {
        self.final_active_artifact_digest
    }

    pub fn final_active_plan_digest(&self) -> u64 {
        self.final_active_plan_digest
    }

    pub fn final_capability_snapshot_digest(&self) -> u64 {
        self.final_capability_snapshot_digest
    }

    pub fn final_authoring_snapshot_digest(&self) -> Option<u64> {
        self.final_authoring_snapshot_digest
    }

    pub fn final_last_valid_artifact_digest(&self) -> u64 {
        self.final_last_valid_artifact_digest
    }

    pub fn final_last_valid_plan_digest(&self) -> u64 {
        self.final_last_valid_plan_digest
    }
}

impl ValidationMixedReloadStormBuildDenial {
    pub fn reason(&self) -> &'static str {
        match self {
            ValidationMixedReloadStormBuildDenial::ScenarioNotQualified => {
                "scenario did not satisfy mixed product storm qualification"
            }
        }
    }
}

impl ValidationMixedReloadStormPosture {
    pub fn activated_step_count(self) -> usize {
        self.activated_step_count
    }

    pub fn equivalent_step_count(self) -> usize {
        self.equivalent_step_count
    }

    pub fn denied_step_count(self) -> usize {
        self.denied_step_count
    }

    pub fn is_mixed(self) -> bool {
        let nonzero = [
            self.activated_step_count,
            self.equivalent_step_count,
            self.denied_step_count,
        ]
        .into_iter()
        .filter(|count| *count > 0)
        .count();
        nonzero > 1
    }
}

impl ValidationMixedReloadStormStep {
    pub fn family(&self) -> ValidationMixedReloadStormFamily {
        self.family
    }

    pub fn status(&self) -> ValidationMixedReloadStormStatus {
        self.status
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }

    pub fn header_rebind(&self) -> Option<&ValidationHeaderRebindEvidence> {
        self.header_rebind.as_ref()
    }

    pub fn page_host_rebind(&self) -> Option<&ValidationPageHostRebindEvidence> {
        self.page_host_rebind.as_ref()
    }
}

impl ValidationMixedReloadStormProjectionCounters {
    pub fn inspected_projection_count(self) -> usize {
        self.inspected_projection_count
    }

    pub fn dependency_intersection_count(self) -> usize {
        self.dependency_intersection_count
    }

    pub fn rebuild_attempt_count(self) -> usize {
        self.rebuild_attempt_count
    }

    pub fn preserved_frame_count(self) -> usize {
        self.preserved_frame_count
    }

    pub fn denied_frame_count(self) -> usize {
        self.denied_frame_count
    }

    pub fn rebuilt_frame_count(self) -> usize {
        self.rebuilt_frame_count
    }
}
