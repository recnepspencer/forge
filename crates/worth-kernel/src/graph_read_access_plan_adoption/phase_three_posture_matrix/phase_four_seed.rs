use super::cap_ledger::{WorthGraphReadAccessPostureCapReport, WorthGraphReadAccessPostureCapRow};
use super::posture_resolution::{
    WorthGraphReadAccessResolvedPosture, WorthGraphReadRequirementPostureMap,
};
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPhaseFourSeed {
    phase_three_closeout_digest_seed: String,
    posture_map_digest: String,
    cap_report_digest: String,
    resolved_postures: Vec<WorthGraphReadAccessResolvedPosture>,
    cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
    resolved_posture_count: usize,
    cap_family_count: usize,
    seed_digest: String,
}

impl WorthGraphReadAccessPhaseFourSeed {
    pub(crate) fn from_phase_three_products(
        phase_three_closeout_digest_seed: &str,
        posture_map: &WorthGraphReadRequirementPostureMap,
        cap_report: &WorthGraphReadAccessPostureCapReport,
    ) -> Self {
        let resolved_postures = posture_map.resolved_postures().to_vec();
        let cap_rows = cap_report.ledger().rows().to_vec();
        let seed_digest = stable_digest(&[
            "worth_graph_read_access_phase_four_seed_v1".to_string(),
            format!("phase_three_seed:{phase_three_closeout_digest_seed}"),
            format!("posture_map:{}", posture_map.map_digest()),
            format!("cap_report:{}", cap_report.report_digest()),
            format!("resolved_posture_count:{}", resolved_postures.len()),
            format!("cap_family_count:{}", cap_rows.len()),
        ]);
        Self {
            phase_three_closeout_digest_seed: phase_three_closeout_digest_seed.to_string(),
            posture_map_digest: posture_map.map_digest().to_string(),
            cap_report_digest: cap_report.report_digest().to_string(),
            resolved_postures,
            cap_rows,
            resolved_posture_count: posture_map.resolved_postures().len(),
            cap_family_count: cap_report.ledger().rows().len(),
            seed_digest,
        }
    }

    pub fn phase_three_closeout_digest_seed(&self) -> &str {
        &self.phase_three_closeout_digest_seed
    }

    pub fn posture_map_digest(&self) -> &str {
        &self.posture_map_digest
    }

    pub fn cap_report_digest(&self) -> &str {
        &self.cap_report_digest
    }

    pub fn resolved_postures(&self) -> &[WorthGraphReadAccessResolvedPosture] {
        &self.resolved_postures
    }

    pub fn cap_rows(&self) -> &[WorthGraphReadAccessPostureCapRow] {
        &self.cap_rows
    }

    pub const fn resolved_posture_count(&self) -> usize {
        self.resolved_posture_count
    }

    pub const fn cap_family_count(&self) -> usize {
        self.cap_family_count
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipt(&self) -> bool {
        false
    }
}
