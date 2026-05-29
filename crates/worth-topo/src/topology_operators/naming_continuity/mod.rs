use serde::{Deserialize, Serialize};

use super::{
    TopologyEditBatch, TopologyEditNamingOutcome, TopologyEditNamingRow, TopologyEditRejectionClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingEditContinuityMatrix {
    pub rows: Vec<TopologyEditNamingRow>,
    pub preserved_count: usize,
    pub ambiguous_count: usize,
    pub rejected_count: usize,
}

impl NamingEditContinuityMatrix {
    pub fn outcome_class(&self) -> TopologyEditNamingOutcome {
        if self.rejected_count > 0 {
            TopologyEditNamingOutcome::Rejected
        } else if self.ambiguous_count > 0 {
            TopologyEditNamingOutcome::Ambiguous
        } else {
            TopologyEditNamingOutcome::Preserved
        }
    }

    pub fn rejection_class(&self) -> Option<TopologyEditRejectionClass> {
        match self.outcome_class() {
            TopologyEditNamingOutcome::Preserved => None,
            TopologyEditNamingOutcome::Ambiguous => {
                Some(TopologyEditRejectionClass::NamingContinuityAmbiguous)
            }
            TopologyEditNamingOutcome::Rejected => {
                Some(TopologyEditRejectionClass::NamingContinuityRejected)
            }
        }
    }
}

impl TopologyEditBatch {
    pub fn naming_edit_continuity_matrix(&self) -> NamingEditContinuityMatrix {
        let rows = self.naming_report().rows;
        let preserved_count = rows
            .iter()
            .filter(|row| row.outcome == TopologyEditNamingOutcome::Preserved)
            .count();
        let ambiguous_count = rows
            .iter()
            .filter(|row| row.outcome == TopologyEditNamingOutcome::Ambiguous)
            .count();
        let rejected_count = rows
            .iter()
            .filter(|row| row.outcome == TopologyEditNamingOutcome::Rejected)
            .count();
        NamingEditContinuityMatrix {
            rows,
            preserved_count,
            ambiguous_count,
            rejected_count,
        }
    }
}
