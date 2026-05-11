use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LineageInvariantGroup {
    ProvenanceCompleteness,
}

impl LineageInvariantGroup {
    pub const ALL: [Self; 1] = [Self::ProvenanceCompleteness];
}
