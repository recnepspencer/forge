use serde::{Deserialize, Serialize};

use super::{
    TopologyMutationNamingOutcome, TopologyMutationNamingRow, TopologyMutationRejectionClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamingMutationContinuityMatrix {
    pub rows: Vec<TopologyMutationNamingRow>,
    pub preserved_count: usize,
    pub ambiguous_count: usize,
    pub rejected_count: usize,
}

impl NamingMutationContinuityMatrix {
    pub fn outcome_class(&self) -> TopologyMutationNamingOutcome {
        if self.rejected_count > 0 {
            TopologyMutationNamingOutcome::Rejected
        } else if self.ambiguous_count > 0 {
            TopologyMutationNamingOutcome::Ambiguous
        } else {
            TopologyMutationNamingOutcome::Preserved
        }
    }

    pub fn rejection_class(&self) -> Option<TopologyMutationRejectionClass> {
        match self.outcome_class() {
            TopologyMutationNamingOutcome::Preserved => None,
            TopologyMutationNamingOutcome::Ambiguous => {
                Some(TopologyMutationRejectionClass::NamingContinuityAmbiguous)
            }
            TopologyMutationNamingOutcome::Rejected => {
                Some(TopologyMutationRejectionClass::NamingContinuityRejected)
            }
        }
    }
}
<<<<<<< HEAD

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
=======
>>>>>>> origin/master
