use crate::identity::ResultDigest;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionCostClass {
    ConstantDirectLookup,
    ConstantMetadataComparison,
    ConstantDeniedSurface,
}

impl IdentityEvolutionCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConstantDirectLookup => "constant_direct_lookup",
            Self::ConstantMetadataComparison => "constant_metadata_comparison",
            Self::ConstantDeniedSurface => "constant_denied_surface",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionBudgetClass {
    SingleAnchorDirectOnly,
    FixedBasisComparisonOnly,
    DenialShapingOnly,
}

impl IdentityEvolutionBudgetClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SingleAnchorDirectOnly => "single_anchor_direct_only",
            Self::FixedBasisComparisonOnly => "fixed_basis_comparison_only",
            Self::DenialShapingOnly => "denial_shaping_only",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityEvolutionPredictionDriftOutcome {
    PredictionNotExecuted,
    PredictionDeferredToLaterPhase,
    WithinBudget,
    WidthDriftDetected,
}

impl IdentityEvolutionPredictionDriftOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PredictionNotExecuted => "prediction_not_executed",
            Self::PredictionDeferredToLaterPhase => "prediction_deferred_to_later_phase",
            Self::WithinBudget => "within_budget",
            Self::WidthDriftDetected => "width_drift_detected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionPredictionReport {
    cost_class: IdentityEvolutionCostClass,
    budget_class: IdentityEvolutionBudgetClass,
    drift_outcome: IdentityEvolutionPredictionDriftOutcome,
    digest: ResultDigest,
}

impl IdentityEvolutionPredictionReport {
    pub fn cost_class(&self) -> IdentityEvolutionCostClass {
        self.cost_class
    }

    pub fn budget_class(&self) -> IdentityEvolutionBudgetClass {
        self.budget_class
    }

    pub fn drift_outcome(&self) -> IdentityEvolutionPredictionDriftOutcome {
        self.drift_outcome
    }

    pub fn digest(&self) -> &ResultDigest {
        &self.digest
    }

    pub(crate) fn zero_work(
        cost_class: IdentityEvolutionCostClass,
        budget_class: IdentityEvolutionBudgetClass,
    ) -> Self {
        let drift_outcome = IdentityEvolutionPredictionDriftOutcome::PredictionDeferredToLaterPhase;
        let digest = ResultDigest::from_parts(&[
            format!("cost_class:{}", cost_class.as_str()),
            format!("budget_class:{}", budget_class.as_str()),
            format!("drift_outcome:{}", drift_outcome.as_str()),
        ]);
        Self {
            cost_class,
            budget_class,
            drift_outcome,
            digest,
        }
    }
}
