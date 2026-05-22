#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionProofGrade {
    BundleCoherence,
    MilestoneCloseout,
    ProofSubstrateCloseout,
}

impl PrimitiveConstructionProofGrade {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BundleCoherence => "bundle_coherence",
            Self::MilestoneCloseout => "milestone_closeout",
            Self::ProofSubstrateCloseout => "proof_substrate_closeout",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionProofSubject {
    Motion,
    IntentArbitration,
    CompoundParity,
    PolicyPressure,
    PhaseFiveSixCloseout,
    MilestoneFourKernelCloseout,
    DigestProtocol,
    VerifiedArtifactSurface,
    TruthProjectionMatrix,
    ProofBoundaryCompileFail,
    ProofSubstrateCloseout,
}

impl PrimitiveConstructionProofSubject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Motion => "motion",
            Self::IntentArbitration => "intent_arbitration",
            Self::CompoundParity => "compound_parity",
            Self::PolicyPressure => "policy_pressure",
            Self::PhaseFiveSixCloseout => "phase_five_six_closeout",
            Self::MilestoneFourKernelCloseout => "milestone_four_kernel_closeout",
            Self::DigestProtocol => "digest_protocol",
            Self::VerifiedArtifactSurface => "verified_artifact_surface",
            Self::TruthProjectionMatrix => "truth_projection_matrix",
            Self::ProofBoundaryCompileFail => "proof_boundary_compile_fail",
            Self::ProofSubstrateCloseout => "proof_substrate_closeout",
        }
    }
}
