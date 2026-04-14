use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthLineageAspect {
    Provenance,
}

impl WorthLineageAspect {
    pub const ALL: [Self; 1] = [Self::Provenance];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provenance => "lineage",
        }
    }
}
