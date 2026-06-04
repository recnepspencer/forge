#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionProofGrade {
    BundleCoherence,
    GeometryTruthHostility,
    MilestoneCloseout,
    ProofSubstrateCloseout,
}

impl PrimitiveConstructionProofGrade {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BundleCoherence => "bundle_coherence",
            Self::GeometryTruthHostility => "geometry_truth_hostility",
            Self::MilestoneCloseout => "milestone_closeout",
            Self::ProofSubstrateCloseout => "proof_substrate_closeout",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionProofSubject {
    CompoundParity,
    #[cfg(test)]
    GeometryDigestSensitivity,
    #[cfg(test)]
    CanonicalWitnessParity,
    #[cfg(test)]
    ShellWithHoleLayoutHostility,
    #[cfg(test)]
    SimplexCanonicalRatio,
    #[cfg(test)]
    PhaseFiveSixCloseout,
    #[cfg(test)]
    MilestoneFourKernelCloseout,
    #[cfg(test)]
    ProofSubstrateCloseout,
}

impl PrimitiveConstructionProofSubject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompoundParity => "compound_parity",
            #[cfg(test)]
            Self::GeometryDigestSensitivity => "geometry_digest_sensitivity",
            #[cfg(test)]
            Self::CanonicalWitnessParity => "canonical_witness_parity",
            #[cfg(test)]
            Self::ShellWithHoleLayoutHostility => "shell_with_hole_layout_hostility",
            #[cfg(test)]
            Self::SimplexCanonicalRatio => "simplex_canonical_ratio",
            #[cfg(test)]
            Self::PhaseFiveSixCloseout => "phase_five_six_closeout",
            #[cfg(test)]
            Self::MilestoneFourKernelCloseout => "milestone_four_kernel_closeout",
            #[cfg(test)]
            Self::ProofSubstrateCloseout => "proof_substrate_closeout",
        }
    }
}
