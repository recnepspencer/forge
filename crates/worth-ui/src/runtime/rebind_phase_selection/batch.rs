use crate::runtime::{
    WorthUiAdmittedProjectionPlan, WorthUiAdmittedRuntimeChangeEvidence, WorthUiHeaderFramePlan,
    WorthUiPageHostPlan, WorthUiProjectionRebindPlan, WorthUiProjectionRebindPlanDenial,
    WorthUiRuntimeChangeEvidenceDigest, WorthUiRuntimeHost, WorthUiRuntimeInstanceWitness,
};

use super::{
    WorthUiRebindPhaseLane, WorthUiRebindPhaseSelectionCounters, WorthUiRebindPhaseSelectionRow,
    WorthUiRebindPhaseSelectionStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRebindPhaseSelectionBatch {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
    counters: WorthUiRebindPhaseSelectionCounters,
    rows: Vec<WorthUiRebindPhaseSelectionRow>,
    replay_digest: u64,
    header_phase_plan: WorthUiProjectionRebindPlan<WorthUiHeaderFramePlan>,
    page_host_phase_plan: WorthUiProjectionRebindPlan<WorthUiPageHostPlan>,
}

impl WorthUiRuntimeHost {
    pub fn plan_rebind_phase_selection(
        &self,
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
        header_projection: WorthUiAdmittedProjectionPlan<WorthUiHeaderFramePlan>,
        page_host_projection: WorthUiAdmittedProjectionPlan<WorthUiPageHostPlan>,
    ) -> Result<WorthUiRebindPhaseSelectionBatch, WorthUiProjectionRebindPlanDenial> {
        let header_phase_plan = self.prepare_projection_rebind(evidence, header_projection)?;
        let page_host_phase_plan =
            self.prepare_projection_rebind(evidence, page_host_projection)?;
        Ok(WorthUiRebindPhaseSelectionBatch::new(
            evidence,
            header_phase_plan,
            page_host_phase_plan,
        ))
    }
}

impl WorthUiRebindPhaseSelectionBatch {
    fn new(
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
        header_phase_plan: WorthUiProjectionRebindPlan<WorthUiHeaderFramePlan>,
        page_host_phase_plan: WorthUiProjectionRebindPlan<WorthUiPageHostPlan>,
    ) -> Self {
        let rows = vec![
            selection_row(WorthUiRebindPhaseLane::HeaderFrame, &header_phase_plan),
            selection_row(WorthUiRebindPhaseLane::PageHost, &page_host_phase_plan),
        ];
        let counters = WorthUiRebindPhaseSelectionCounters::from_rows(&rows);
        let replay_digest = replay_digest(evidence.digest(), &rows);
        Self {
            runtime_instance: evidence.runtime_instance(),
            change_evidence_digest: evidence.digest(),
            counters,
            rows,
            replay_digest,
            header_phase_plan,
            page_host_phase_plan,
        }
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn change_evidence_digest(&self) -> WorthUiRuntimeChangeEvidenceDigest {
        self.change_evidence_digest
    }

    pub fn counters(&self) -> WorthUiRebindPhaseSelectionCounters {
        self.counters
    }

    pub fn rows(&self) -> &[WorthUiRebindPhaseSelectionRow] {
        &self.rows
    }

    pub fn replay_digest(&self) -> u64 {
        self.replay_digest
    }

    pub fn into_plans(
        self,
    ) -> (
        WorthUiProjectionRebindPlan<WorthUiHeaderFramePlan>,
        WorthUiProjectionRebindPlan<WorthUiPageHostPlan>,
    ) {
        (self.header_phase_plan, self.page_host_phase_plan)
    }
}

fn selection_row<P: crate::runtime::WorthUiProjectionPlanContract>(
    lane: WorthUiRebindPhaseLane,
    plan: &WorthUiProjectionRebindPlan<P>,
) -> WorthUiRebindPhaseSelectionRow {
    match plan {
        WorthUiProjectionRebindPlan::Preserve(preserved) => WorthUiRebindPhaseSelectionRow::new(
            lane,
            match preserved.status() {
                crate::runtime::WorthUiProjectionRebindStatus::PreservedEquivalentReload => {
                    WorthUiRebindPhaseSelectionStatus::PreservedEquivalentReload
                }
                crate::runtime::WorthUiProjectionRebindStatus::PreservedDeniedReload
                | crate::runtime::WorthUiProjectionRebindStatus::DeniedReloadNotActivated => {
                    WorthUiRebindPhaseSelectionStatus::PreservedDeniedReload
                }
                crate::runtime::WorthUiProjectionRebindStatus::EquivalentAfterActivation => {
                    WorthUiRebindPhaseSelectionStatus::PreservedWithoutIntersection
                }
                crate::runtime::WorthUiProjectionRebindStatus::ReboundAfterActivation => {
                    unreachable!("preserved projection plan cannot advertise rebuild status")
                }
            },
            0,
        ),
        WorthUiProjectionRebindPlan::Rebuild(_) => WorthUiRebindPhaseSelectionRow::new(
            lane,
            WorthUiRebindPhaseSelectionStatus::RebuildScheduled,
            1,
        ),
    }
}

fn replay_digest(
    change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
    rows: &[WorthUiRebindPhaseSelectionRow],
) -> u64 {
    let mut digest = change_evidence_digest.value();
    for row in rows {
        digest = digest.rotate_left(7)
            ^ match row.lane() {
                WorthUiRebindPhaseLane::HeaderFrame => 0x4845_4144_4552,
                WorthUiRebindPhaseLane::PageHost => 0x5041_4745_484f_5354,
            };
        digest = digest.rotate_left(11)
            ^ match row.status() {
                WorthUiRebindPhaseSelectionStatus::PreservedEquivalentReload => 1,
                WorthUiRebindPhaseSelectionStatus::PreservedDeniedReload => 2,
                WorthUiRebindPhaseSelectionStatus::PreservedWithoutIntersection => 3,
                WorthUiRebindPhaseSelectionStatus::RebuildScheduled => 4,
            };
        digest ^= row.dependency_intersection_count() as u64;
    }
    digest
}
