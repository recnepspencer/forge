use serde::{Deserialize, Serialize};

use super::{TopologyEditNamingOutcome, TopologyEditNamingRow, TopologyEditRejectionClass};

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
