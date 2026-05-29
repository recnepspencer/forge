use super::super::hostile_categories::{
    MilestoneThreeHostileCertificationCategory, MilestoneThreeHostileCertificationCategoryRow,
    MilestoneThreeHostileCertificationStatus,
};

impl MilestoneThreeHostileCertificationCategoryRow {
    pub fn category(&self) -> MilestoneThreeHostileCertificationCategory {
        self.category
    }

    pub fn status(&self) -> MilestoneThreeHostileCertificationStatus {
        self.status
    }

    pub fn scenario_count(&self) -> usize {
        self.scenario_count
    }

    pub fn evidence_count(&self) -> usize {
        self.evidence_count
    }

    pub fn replay_verified_count(&self) -> usize {
        self.replay_verified_count
    }

    pub fn diagnostic_locality_count(&self) -> usize {
        self.diagnostic_locality_count
    }

    pub fn evidence_labels(&self) -> &[String] {
        &self.evidence_labels
    }

    pub fn gap_labels(&self) -> &[String] {
        &self.gap_labels
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}




