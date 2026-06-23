use worth_ui::facade::{
    WorthUiActiveRuntimeObservation, WorthUiLastValidObservation, WorthUiRuntimeFactId,
};

use crate::reload::ValidationReloadEvidenceLog;
use crate::runtime_workbench::ValidationRuntimeWorkbench;

use super::projection_roster::ValidationMixedReloadStormProjectionRoster;
use super::qualification::latest_qualified_steps;
use super::types::{
    ValidationMixedReloadStormBuildDenial, ValidationMixedReloadStormFamily,
    ValidationMixedReloadStormPosture, ValidationMixedReloadStormProjectionCounters,
    ValidationMixedReloadStormProof, ValidationMixedReloadStormStatus,
    ValidationMixedReloadStormStep,
};

impl ValidationMixedReloadStormProof {
    pub fn inspect_from_workbench(
        workbench: &ValidationRuntimeWorkbench,
        evidence_log: &ValidationReloadEvidenceLog,
    ) -> Result<Self, ValidationMixedReloadStormBuildDenial> {
        let steps = latest_qualified_steps(evidence_log.entries())?;
        let active = workbench.runtime().inspect_active();
        let last_valid = workbench.runtime().last_valid();
        Ok(Self::from_steps(active, last_valid, steps))
    }

    pub(super) fn from_steps(
        active: WorthUiActiveRuntimeObservation,
        last_valid: WorthUiLastValidObservation,
        steps: Vec<ValidationMixedReloadStormStep>,
    ) -> Self {
        let posture = ValidationMixedReloadStormPosture::from_steps(&steps);
        let projection_counters = ValidationMixedReloadStormProjectionCounters::from_steps(&steps);
        let projection_roster = ValidationMixedReloadStormProjectionRoster::from_steps(&steps);
        let scenario_digest = fold_texts(
            steps
                .iter()
                .map(ValidationMixedReloadStormStep::digest_basis),
        );
        Self {
            scenario_digest,
            posture,
            steps,
            projection_counters,
            projection_roster,
            final_active_artifact_digest: active.artifact_digest(),
            final_active_plan_digest: active.active_plan_digest(),
            final_capability_snapshot_digest: active.snapshot_digest(),
            final_authoring_snapshot_digest: active.authoring_snapshot_digest(),
            final_last_valid_artifact_digest: last_valid.artifact_digest(),
            final_last_valid_plan_digest: last_valid.active_plan_digest(),
        }
    }

    pub fn projection_frame_digest(&self) -> u64 {
        fold_texts(
            self.steps
                .iter()
                .flat_map(ValidationMixedReloadStormStep::projection_digest_rows),
        )
    }
}

impl ValidationMixedReloadStormPosture {
    fn from_steps(steps: &[ValidationMixedReloadStormStep]) -> Self {
        let mut posture = Self {
            activated_step_count: 0,
            equivalent_step_count: 0,
            denied_step_count: 0,
        };
        for step in steps {
            match step.status {
                ValidationMixedReloadStormStatus::Activated => posture.activated_step_count += 1,
                ValidationMixedReloadStormStatus::EquivalentNoOp => {
                    posture.equivalent_step_count += 1
                }
                ValidationMixedReloadStormStatus::Denied => posture.denied_step_count += 1,
                ValidationMixedReloadStormStatus::ReadyForFrameBoundary => {}
            }
        }
        posture
    }
}

impl ValidationMixedReloadStormStep {
    pub(super) fn new(
        family: ValidationMixedReloadStormFamily,
        status: ValidationMixedReloadStormStatus,
        changed_facts: Vec<WorthUiRuntimeFactId>,
        denial_detail: Option<String>,
        header_rebind: Option<crate::reload::ValidationHeaderRebindEvidence>,
        page_host_rebind: Option<crate::reload::ValidationPageHostRebindEvidence>,
    ) -> Self {
        Self {
            family,
            status,
            changed_facts,
            denial_detail,
            header_rebind,
            page_host_rebind,
        }
    }

    pub(super) fn digest_basis(&self) -> String {
        let facts = self
            .changed_facts
            .iter()
            .map(|fact| format!("{fact:?}"))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "{:?}|{:?}|{}|{}|{}",
            self.family,
            self.status,
            facts,
            self.denial_detail.as_deref().unwrap_or(""),
            self.projection_frame_digest()
        )
    }

    pub(crate) fn projection_digest_rows(&self) -> Vec<String> {
        let mut rows = Vec::new();
        if let Some(header) = &self.header_rebind {
            for row in header.rows() {
                rows.push(format!(
                    "header|{}|{:?}|{:?}|{}|{}",
                    row.projection_identity(),
                    row.projection_family(),
                    row.status(),
                    row.previous_frame_digest(),
                    row.rebound_frame_digest()
                ));
            }
        }
        if let Some(page_host) = &self.page_host_rebind {
            for row in page_host.rows() {
                rows.push(format!(
                    "page_host|{}|{:?}|{:?}|{}|{}",
                    row.projection_identity(),
                    row.projection_family(),
                    row.status(),
                    row.previous_frame_digest(),
                    row.rebound_frame_digest()
                ));
            }
        }
        rows
    }

    fn projection_frame_digest(&self) -> u64 {
        fold_texts(self.projection_digest_rows())
    }
}

impl ValidationMixedReloadStormProjectionCounters {
    fn from_steps(steps: &[ValidationMixedReloadStormStep]) -> Self {
        let mut counters = Self::default();
        for step in steps {
            if let Some(header) = step.header_rebind() {
                counters.absorb(
                    header.inspected_projection_count(),
                    header.dependency_intersection_count(),
                    header.rebuild_attempt_count(),
                    header.preserved_frame_count(),
                    header.denied_frame_count(),
                    header.rebuilt_frame_count(),
                );
            }
            if let Some(page_host) = step.page_host_rebind() {
                counters.absorb(
                    page_host.inspected_projection_count(),
                    page_host.dependency_intersection_count(),
                    page_host.rebuild_attempt_count(),
                    page_host.preserved_frame_count(),
                    page_host.denied_frame_count(),
                    page_host.rebuilt_frame_count(),
                );
            }
        }
        counters
    }

    fn absorb(
        &mut self,
        inspected: usize,
        intersections: usize,
        rebuilds: usize,
        preserved: usize,
        denied: usize,
        rebuilt: usize,
    ) {
        self.inspected_projection_count += inspected;
        self.dependency_intersection_count += intersections;
        self.rebuild_attempt_count += rebuilds;
        self.preserved_frame_count += preserved;
        self.denied_frame_count += denied;
        self.rebuilt_frame_count += rebuilt;
    }
}

fn fold_texts(texts: impl IntoIterator<Item = impl AsRef<str>>) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for text in texts {
        for byte in text.as_ref().as_bytes() {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}
