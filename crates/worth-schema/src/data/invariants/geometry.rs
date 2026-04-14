use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WorthGeometryInvariantGroup {
    BindingCoverage,
    CarrierCompatibility,
    UvAnchoringContinuity,
    ApproximationBounded,
    ToleranceRegimeValidity,
    ProvenanceCompleteness,
    PrecisionEscalationDeclared,
    FallbackDispositionDeclared,
    FallbackProofSufficiency,
}

impl WorthGeometryInvariantGroup {
    pub const ALL: [Self; 9] = [
        Self::BindingCoverage,
        Self::CarrierCompatibility,
        Self::UvAnchoringContinuity,
        Self::ApproximationBounded,
        Self::ToleranceRegimeValidity,
        Self::ProvenanceCompleteness,
        Self::PrecisionEscalationDeclared,
        Self::FallbackDispositionDeclared,
        Self::FallbackProofSufficiency,
    ];
}
