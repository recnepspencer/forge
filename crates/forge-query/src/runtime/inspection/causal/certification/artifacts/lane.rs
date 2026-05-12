#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionCertificationLane {
    ChangedResult,
    PolicyRedacted,
    DeniedResult,
    MissingEvidence,
    PublicBoundary,
    RepresentativeMatrix,
    ScaleMaterialization,
    ProofShape,
    BridgeReadmission,
    ArtifactSerialization,
}

impl CausalInspectionCertificationLane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChangedResult => "changed_result",
            Self::PolicyRedacted => "policy_redacted",
            Self::DeniedResult => "denied_result",
            Self::MissingEvidence => "missing_evidence",
            Self::PublicBoundary => "public_boundary",
            Self::RepresentativeMatrix => "representative_matrix",
            Self::ScaleMaterialization => "scale_materialization",
            Self::ProofShape => "proof_shape",
            Self::BridgeReadmission => "bridge_readmission",
            Self::ArtifactSerialization => "artifact_serialization",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CausalInspectionScaleFixtureSize {
    Small,
    Medium,
    Large,
}

impl CausalInspectionScaleFixtureSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
