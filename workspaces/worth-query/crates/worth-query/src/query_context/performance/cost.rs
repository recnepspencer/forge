#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextCostClass {
    CurrentHeadNarrow,
    BranchHeadNarrow,
    HistoricalRetainedBounded,
    HistoricalReplayBounded,
    HistoricalReconstructionBounded,
    PreviewDerivedHistoricalBounded,
    DiffComparisonBounded,
}

impl QueryContextCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentHeadNarrow => "current_head_narrow",
            Self::BranchHeadNarrow => "branch_head_narrow",
            Self::HistoricalRetainedBounded => "historical_retained_bounded",
            Self::HistoricalReplayBounded => "historical_replay_bounded",
            Self::HistoricalReconstructionBounded => "historical_reconstruction_bounded",
            Self::PreviewDerivedHistoricalBounded => "preview_derived_historical_bounded",
            Self::DiffComparisonBounded => "diff_comparison_bounded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextBudgetClass {
    NarrowSingleBasis,
    HistoricalBounded,
    PreviewDerivedBounded,
    ComparisonBounded,
}

impl QueryContextBudgetClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NarrowSingleBasis => "narrow_single_basis",
            Self::HistoricalBounded => "historical_bounded",
            Self::PreviewDerivedBounded => "preview_derived_bounded",
            Self::ComparisonBounded => "comparison_bounded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoricalMaterializationCostClass {
    RetainedBounded,
    ReplayBounded,
    ReconstructionBounded,
}

impl HistoricalMaterializationCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RetainedBounded => "retained_bounded",
            Self::ReplayBounded => "replay_bounded",
            Self::ReconstructionBounded => "reconstruction_bounded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryContextPredictionReport {
    basis_binding_width: usize,
    historical_lookup_width: usize,
    comparison_binding_width: usize,
    comparison_row_width: usize,
    denial_width: usize,
}

impl QueryContextPredictionReport {
    pub fn basis_binding_width(&self) -> usize {
        self.basis_binding_width
    }

    pub fn historical_lookup_width(&self) -> usize {
        self.historical_lookup_width
    }

    pub fn comparison_binding_width(&self) -> usize {
        self.comparison_binding_width
    }

    pub fn comparison_row_width(&self) -> usize {
        self.comparison_row_width
    }

    pub fn denial_width(&self) -> usize {
        self.denial_width
    }

    pub(crate) fn for_runtime_binding() -> Self {
        Self {
            basis_binding_width: 1,
            historical_lookup_width: 0,
            comparison_binding_width: 0,
            comparison_row_width: 0,
            denial_width: 0,
        }
    }

    pub(crate) fn for_historical_binding() -> Self {
        Self {
            basis_binding_width: 1,
            historical_lookup_width: 1,
            comparison_binding_width: 0,
            comparison_row_width: 0,
            denial_width: 0,
        }
    }

    pub(crate) fn for_preview_binding() -> Self {
        Self {
            basis_binding_width: 1,
            historical_lookup_width: 0,
            comparison_binding_width: 0,
            comparison_row_width: 0,
            denial_width: 0,
        }
    }

    pub(crate) fn for_diff_binding(predicted_row_width: usize) -> Self {
        Self {
            basis_binding_width: 0,
            historical_lookup_width: 0,
            comparison_binding_width: 2,
            comparison_row_width: predicted_row_width,
            denial_width: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryContextPredictionDriftOutcome {
    PendingExecution,
    PendingComparison,
    WithinBudget,
    ComparisonScopeTooBroadDenied,
}

impl QueryContextPredictionDriftOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PendingExecution => "pending_execution",
            Self::PendingComparison => "pending_comparison",
            Self::WithinBudget => "within_budget",
            Self::ComparisonScopeTooBroadDenied => "comparison_scope_too_broad_denied",
        }
    }
}
