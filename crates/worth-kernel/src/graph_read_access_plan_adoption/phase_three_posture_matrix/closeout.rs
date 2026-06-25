use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout;

use super::cap_ledger::WorthGraphReadAccessPostureCapReport;
use super::counters::WorthGraphReadAccessPostureMatrixCounters;
use super::errors::WorthGraphReadAccessPostureMatrixError;
use super::phase_four_seed::WorthGraphReadAccessPhaseFourSeed;
use super::posture_resolution::WorthGraphReadRequirementPostureMap;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureMatrixCloseout {
    phase_two_closeout_digest: String,
    posture_map: WorthGraphReadRequirementPostureMap,
    cap_report: WorthGraphReadAccessPostureCapReport,
    counters: WorthGraphReadAccessPostureMatrixCounters,
    phase_four_seed: WorthGraphReadAccessPhaseFourSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_posture_matrix_closeout(
    phase_two: &WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout,
) -> Result<WorthGraphReadAccessPostureMatrixCloseout, WorthGraphReadAccessPostureMatrixError> {
    let posture_map = WorthGraphReadRequirementPostureMap::from_phase_two_closeout(phase_two)?;
    let cap_report = WorthGraphReadAccessPostureCapReport::from_posture_map(&posture_map)?;
    let counters =
        WorthGraphReadAccessPostureMatrixCounters::from_products(&posture_map, &cap_report);
    let closeout_digest = phase_three_closeout_digest(phase_two, &posture_map, &cap_report);
    let phase_four_seed = WorthGraphReadAccessPhaseFourSeed::from_phase_three_products(
        &closeout_digest,
        &posture_map,
        &cap_report,
    );

    Ok(WorthGraphReadAccessPostureMatrixCloseout {
        phase_two_closeout_digest: phase_two.closeout_digest().to_string(),
        posture_map,
        cap_report,
        counters,
        phase_four_seed,
        closeout_digest,
    })
}

impl WorthGraphReadAccessPostureMatrixCloseout {
    pub fn phase_two_closeout_digest(&self) -> &str {
        &self.phase_two_closeout_digest
    }

    pub const fn posture_map(&self) -> &WorthGraphReadRequirementPostureMap {
        &self.posture_map
    }

    pub const fn cap_report(&self) -> &WorthGraphReadAccessPostureCapReport {
        &self.cap_report
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessPostureMatrixCounters {
        &self.counters
    }

    pub const fn phase_four_seed(&self) -> &WorthGraphReadAccessPhaseFourSeed {
        &self.phase_four_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_access_plan_admission_attempts(&self) -> bool {
        true
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipts(&self) -> bool {
        false
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }

    pub const fn claims_milestone_nine_seed_export(&self) -> bool {
        false
    }
}

fn phase_three_closeout_digest(
    phase_two: &WorthGraphReadAccessPlanAdoptionPhaseTwoCloseout,
    posture_map: &WorthGraphReadRequirementPostureMap,
    cap_report: &WorthGraphReadAccessPostureCapReport,
) -> String {
    stable_digest(&[
        "worth_graph_read_access_posture_matrix_closeout_v1".to_string(),
        format!("phase_two_closeout:{}", phase_two.closeout_digest()),
        format!("posture_map:{}", posture_map.map_digest()),
        format!("cap_report:{}", cap_report.report_digest()),
    ])
}
