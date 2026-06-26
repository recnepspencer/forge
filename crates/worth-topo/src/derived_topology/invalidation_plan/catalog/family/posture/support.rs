use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologySupportPosture {
    QuerySupportRequired,
    LegalitySupportRequired,
    NoExternalSupportRequired,
}

impl DerivedTopologySupportPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuerySupportRequired => "query_support_required",
            Self::LegalitySupportRequired => "legality_support_required",
            Self::NoExternalSupportRequired => "no_external_support_required",
        }
    }
}
