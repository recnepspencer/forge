use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedInvalidationDensityPolicy {
    Sparse,
    Dense,
}

impl DerivedInvalidationDensityPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sparse => "sparse",
            Self::Dense => "dense",
        }
    }
}
