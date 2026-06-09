#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TilingEquivalenceScope {
    ExactConflictGraph,
    ConflictCore,
    MotifTerminalBehavior,
    PeriodicQuotientConstraints,
    GeneratedClosure,
    TileContactGraph,
    MetricThresholdClass,
    PeriodicColorRule,
    ProofAdmissionGap,
    CheckerInputReuse,
}

impl TilingEquivalenceScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExactConflictGraph => "exact_conflict_graph",
            Self::ConflictCore => "conflict_core",
            Self::MotifTerminalBehavior => "motif_terminal_behavior",
            Self::PeriodicQuotientConstraints => "periodic_quotient_constraints",
            Self::GeneratedClosure => "generated_closure",
            Self::TileContactGraph => "tile_contact_graph",
            Self::MetricThresholdClass => "metric_threshold_class",
            Self::PeriodicColorRule => "periodic_color_rule",
            Self::ProofAdmissionGap => "proof_admission_gap",
            Self::CheckerInputReuse => "checker_input_reuse",
        }
    }

    pub fn blocks_checker_work(self) -> bool {
        matches!(
            self,
            Self::ExactConflictGraph
                | Self::PeriodicQuotientConstraints
                | Self::GeneratedClosure
                | Self::TileContactGraph
                | Self::MetricThresholdClass
                | Self::PeriodicColorRule
                | Self::CheckerInputReuse
        )
    }

    pub fn blocks_proof_admission(self) -> bool {
        matches!(
            self,
            Self::ConflictCore | Self::MotifTerminalBehavior | Self::ProofAdmissionGap
        )
    }

    pub fn requires_reactivation_for_replan(self) -> bool {
        self.blocks_checker_work() || self.blocks_proof_admission()
    }
}
