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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct QueryContextCounters {
    query_basis_binding_count: usize,
    historical_basis_lookup_count: usize,
    comparison_basis_lookup_count: usize,
    comparison_scope_width: usize,
    comparison_row_width: usize,
    diff_input_breadth: usize,
    unsupported_basis_denial_count: usize,
    basis_substitution_denial_count: usize,
    comparison_broadening_denial_count: usize,
    basis_binding_width: usize,
    historical_lookup_width: usize,
    denial_width: usize,
    basis_rediscovery_count: usize,
    historical_path_rediscovery_count: usize,
    comparison_family_rediscovery_count: usize,
}

impl QueryContextCounters {
    pub fn query_basis_binding_count(&self) -> usize {
        self.query_basis_binding_count
    }

    pub fn historical_basis_lookup_count(&self) -> usize {
        self.historical_basis_lookup_count
    }

    pub fn comparison_basis_lookup_count(&self) -> usize {
        self.comparison_basis_lookup_count
    }

    pub fn comparison_scope_width(&self) -> usize {
        self.comparison_scope_width
    }

    pub fn comparison_row_width(&self) -> usize {
        self.comparison_row_width
    }

    pub fn diff_input_breadth(&self) -> usize {
        self.diff_input_breadth
    }

    pub fn unsupported_basis_denial_count(&self) -> usize {
        self.unsupported_basis_denial_count
    }

    pub fn basis_substitution_denial_count(&self) -> usize {
        self.basis_substitution_denial_count
    }

    pub fn comparison_broadening_denial_count(&self) -> usize {
        self.comparison_broadening_denial_count
    }

    pub fn basis_binding_width(&self) -> usize {
        self.basis_binding_width
    }

    pub fn historical_lookup_width(&self) -> usize {
        self.historical_lookup_width
    }

    pub fn denial_width(&self) -> usize {
        self.denial_width
    }

    pub fn basis_rediscovery_count(&self) -> usize {
        self.basis_rediscovery_count
    }

    pub fn historical_path_rediscovery_count(&self) -> usize {
        self.historical_path_rediscovery_count
    }

    pub fn comparison_family_rediscovery_count(&self) -> usize {
        self.comparison_family_rediscovery_count
    }

    pub fn basis_context_binding_count(&self) -> usize {
        self.query_basis_binding_count()
    }

    pub(crate) fn for_runtime_basis_binding() -> Self {
        Self {
            query_basis_binding_count: 1,
            historical_basis_lookup_count: 0,
            comparison_basis_lookup_count: 0,
            comparison_scope_width: 0,
            comparison_row_width: 0,
            diff_input_breadth: 0,
            unsupported_basis_denial_count: 0,
            basis_substitution_denial_count: 0,
            comparison_broadening_denial_count: 0,
            basis_binding_width: 1,
            historical_lookup_width: 0,
            denial_width: 0,
            basis_rediscovery_count: 0,
            historical_path_rediscovery_count: 0,
            comparison_family_rediscovery_count: 0,
        }
    }

    pub(crate) fn for_historical_basis_binding() -> Self {
        Self {
            query_basis_binding_count: 1,
            historical_basis_lookup_count: 1,
            comparison_basis_lookup_count: 0,
            comparison_scope_width: 0,
            comparison_row_width: 0,
            diff_input_breadth: 0,
            unsupported_basis_denial_count: 0,
            basis_substitution_denial_count: 0,
            comparison_broadening_denial_count: 0,
            basis_binding_width: 1,
            historical_lookup_width: 1,
            denial_width: 0,
            basis_rediscovery_count: 0,
            historical_path_rediscovery_count: 0,
            comparison_family_rediscovery_count: 0,
        }
    }

    pub(crate) fn for_preview_basis_binding() -> Self {
        Self {
            query_basis_binding_count: 1,
            historical_basis_lookup_count: 0,
            comparison_basis_lookup_count: 0,
            comparison_scope_width: 0,
            comparison_row_width: 0,
            diff_input_breadth: 0,
            unsupported_basis_denial_count: 0,
            basis_substitution_denial_count: 0,
            comparison_broadening_denial_count: 0,
            basis_binding_width: 1,
            historical_lookup_width: 0,
            denial_width: 0,
            basis_rediscovery_count: 0,
            historical_path_rediscovery_count: 0,
            comparison_family_rediscovery_count: 0,
        }
    }

    pub(crate) fn for_denial(historical_lookup: bool, substitution: bool) -> Self {
        Self {
            query_basis_binding_count: 0,
            historical_basis_lookup_count: usize::from(historical_lookup),
            comparison_basis_lookup_count: 0,
            comparison_scope_width: 0,
            comparison_row_width: 0,
            diff_input_breadth: 0,
            unsupported_basis_denial_count: 1,
            basis_substitution_denial_count: usize::from(substitution),
            comparison_broadening_denial_count: 0,
            basis_binding_width: 0,
            historical_lookup_width: usize::from(historical_lookup),
            denial_width: 1,
            basis_rediscovery_count: 0,
            historical_path_rediscovery_count: 0,
            comparison_family_rediscovery_count: 0,
        }
    }

    pub(crate) fn for_diff(comparison_row_width: usize) -> Self {
        Self {
            query_basis_binding_count: 0,
            historical_basis_lookup_count: 0,
            comparison_basis_lookup_count: 1,
            comparison_scope_width: 2,
            comparison_row_width,
            diff_input_breadth: 2,
            unsupported_basis_denial_count: 0,
            basis_substitution_denial_count: 0,
            comparison_broadening_denial_count: 0,
            basis_binding_width: 0,
            historical_lookup_width: 0,
            denial_width: 0,
            basis_rediscovery_count: 0,
            historical_path_rediscovery_count: 0,
            comparison_family_rediscovery_count: 0,
        }
    }

    pub(crate) fn for_diff_denial(substitution: bool, broadening: bool) -> Self {
        Self {
            query_basis_binding_count: 0,
            historical_basis_lookup_count: 0,
            comparison_basis_lookup_count: 1,
            comparison_scope_width: 2,
            comparison_row_width: 0,
            diff_input_breadth: 2,
            unsupported_basis_denial_count: 1,
            basis_substitution_denial_count: usize::from(substitution),
            comparison_broadening_denial_count: usize::from(broadening),
            basis_binding_width: 0,
            historical_lookup_width: 0,
            denial_width: 1,
            basis_rediscovery_count: 0,
            historical_path_rediscovery_count: 0,
            comparison_family_rediscovery_count: 0,
        }
    }
}
