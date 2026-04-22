#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionCertificationComparisonRelationship {
    SemanticEquivalence,
    IntentionalDivergence,
    ExpectedRejection,
    DiagnosticsOnlyVariation,
    ResidueAbsence,
    ReplayEquivalence,
    CounterContract,
    BundleCompleteness,
}

impl BridgeSubscriptionCertificationComparisonRelationship {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticEquivalence => "semantic_equivalence",
            Self::IntentionalDivergence => "intentional_divergence",
            Self::ExpectedRejection => "expected_rejection",
            Self::DiagnosticsOnlyVariation => "diagnostics_only_variation",
            Self::ResidueAbsence => "residue_absence",
            Self::ReplayEquivalence => "replay_equivalence",
            Self::CounterContract => "counter_contract",
            Self::BundleCompleteness => "bundle_completeness",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionCertificationDivergenceAxis {
    DeclarationFamily,
    Basis,
    DeliveryFamily,
    ContinuationDecision,
    BranchScope,
    PreviewOutcome,
    StrategyLowering,
    DiagnosticsDetail,
    CounterContract,
    BundleCompleteness,
}

impl BridgeSubscriptionCertificationDivergenceAxis {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationFamily => "declaration_family",
            Self::Basis => "basis",
            Self::DeliveryFamily => "delivery_family",
            Self::ContinuationDecision => "continuation_decision",
            Self::BranchScope => "branch_scope",
            Self::PreviewOutcome => "preview_outcome",
            Self::StrategyLowering => "strategy_lowering",
            Self::DiagnosticsDetail => "diagnostics_detail",
            Self::CounterContract => "counter_contract",
            Self::BundleCompleteness => "bundle_completeness",
        }
    }
}
