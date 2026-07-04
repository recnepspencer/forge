use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlannerMismatchLocus {
    SelectedRoute,
    SelectedProduct,
    QuerySupportPosture,
    PublicProofProjection,
    DiagnosticProjection,
}

impl PlannerMismatchLocus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedRoute => "selected-route",
            Self::SelectedProduct => "selected-product",
            Self::QuerySupportPosture => "query-support-posture",
            Self::PublicProofProjection => "public-proof-projection",
            Self::DiagnosticProjection => "diagnostic-projection",
        }
    }
}
