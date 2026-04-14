use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthLineageInvariantGroup {
    ProvenanceCompleteness,
}

impl WorthLineageInvariantGroup {
    pub const ALL: [Self; 1] = [Self::ProvenanceCompleteness];
}
