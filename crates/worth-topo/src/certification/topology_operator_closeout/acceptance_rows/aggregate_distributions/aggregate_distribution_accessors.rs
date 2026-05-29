use crate::topology_operators::{
    TopologyEditFamily, TopologyEditNamingOutcome, TopologyEditRejectionClass,
};

use super::super::super::report::{
    MilestoneThreeHostileFamilyCoverageRow, MilestoneThreeHostileNamingDistributionRow,
    MilestoneThreeHostileRejectionDistributionRow, MilestoneThreeHostileScenario,
};

impl MilestoneThreeHostileFamilyCoverageRow {
    pub fn family(&self) -> TopologyEditFamily {
        self.family
    }

    pub fn scenario_count(&self) -> usize {
        self.scenario_count
    }

    pub fn scenarios(&self) -> &[MilestoneThreeHostileScenario] {
        self.scenarios.as_slice()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeHostileRejectionDistributionRow {
    pub fn rejection_class(&self) -> TopologyEditRejectionClass {
        self.rejection_class
    }

    pub fn case_count(&self) -> usize {
        self.case_count
    }

    pub fn scenarios(&self) -> &[MilestoneThreeHostileScenario] {
        self.scenarios.as_slice()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}

impl MilestoneThreeHostileNamingDistributionRow {
    pub fn continuity_outcome_class(&self) -> TopologyEditNamingOutcome {
        self.continuity_outcome_class
    }

    pub fn case_count(&self) -> usize {
        self.case_count
    }

    pub fn scenarios(&self) -> &[MilestoneThreeHostileScenario] {
        self.scenarios.as_slice()
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}




