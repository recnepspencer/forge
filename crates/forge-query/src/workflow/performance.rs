#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkflowCounters {
    pub(crate) workflow_declaration_count: usize,
    pub(crate) workflow_basis_binding_count: usize,
    pub(crate) workflow_basis_binding_width: usize,
    pub(crate) workflow_authority_target_check_count: usize,
    pub(crate) workflow_denial_count: usize,
    pub(crate) workflow_broadening_denial_count: usize,
    pub(crate) workflow_executor_rediscovery_count: usize,
}

impl WorkflowCounters {
    pub fn workflow_declaration_count(&self) -> usize {
        self.workflow_declaration_count
    }

    pub fn workflow_basis_binding_count(&self) -> usize {
        self.workflow_basis_binding_count
    }

    pub fn workflow_basis_binding_width(&self) -> usize {
        self.workflow_basis_binding_width
    }

    pub fn workflow_authority_target_check_count(&self) -> usize {
        self.workflow_authority_target_check_count
    }

    pub fn workflow_denial_count(&self) -> usize {
        self.workflow_denial_count
    }

    pub fn workflow_broadening_denial_count(&self) -> usize {
        self.workflow_broadening_denial_count
    }

    pub fn workflow_executor_rediscovery_count(&self) -> usize {
        self.workflow_executor_rediscovery_count
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkflowLoweringCounters {
    pub(crate) workflow_declaration_count: usize,
    pub(crate) workflow_lowering_count: usize,
    pub(crate) workflow_mutation_lowering_count: usize,
    pub(crate) workflow_merge_lowering_count: usize,
    pub(crate) workflow_lowering_width: usize,
    pub(crate) workflow_lowering_denial_count: usize,
    pub(crate) workflow_merge_denial_count: usize,
    pub(crate) workflow_writeback_declaration_count: usize,
    pub(crate) workflow_writeback_denial_count: usize,
    pub(crate) workflow_writeback_causality_binding_count: usize,
    pub(crate) workflow_staleness_check_count: usize,
    pub(crate) workflow_stale_denial_count: usize,
    pub(crate) workflow_lowering_staleness_denial_count: usize,
    pub(crate) workflow_explicit_rebind_required_count: usize,
    pub(crate) workflow_authority_override_denial_count: usize,
    pub(crate) workflow_ambient_basis_fallback_denial_count: usize,
    pub(crate) workflow_replay_bundle_count: usize,
    pub(crate) workflow_budget_cross_count: usize,
    // Counts the number of lower-authority request artifacts minted directly
    // from admitted query workflow declarations without host rediscovery or
    // extra lowering passes. Current admitted lanes each mint exactly one.
    pub(crate) workflow_work_avoided_by_query_lowering_count: usize,
    pub(crate) workflow_executor_rediscovery_count: usize,
}

impl WorkflowLoweringCounters {
    pub fn workflow_declaration_count(&self) -> usize {
        self.workflow_declaration_count
    }

    pub fn workflow_lowering_count(&self) -> usize {
        self.workflow_lowering_count
    }

    pub fn workflow_mutation_lowering_count(&self) -> usize {
        self.workflow_mutation_lowering_count
    }

    pub fn workflow_merge_lowering_count(&self) -> usize {
        self.workflow_merge_lowering_count
    }

    pub fn workflow_lowering_width(&self) -> usize {
        self.workflow_lowering_width
    }

    pub fn workflow_lowering_denial_count(&self) -> usize {
        self.workflow_lowering_denial_count
    }

    pub fn workflow_merge_denial_count(&self) -> usize {
        self.workflow_merge_denial_count
    }

    pub fn workflow_writeback_declaration_count(&self) -> usize {
        self.workflow_writeback_declaration_count
    }

    pub fn workflow_writeback_denial_count(&self) -> usize {
        self.workflow_writeback_denial_count
    }

    pub fn workflow_writeback_causality_binding_count(&self) -> usize {
        self.workflow_writeback_causality_binding_count
    }

    pub fn workflow_staleness_check_count(&self) -> usize {
        self.workflow_staleness_check_count
    }

    pub fn workflow_stale_denial_count(&self) -> usize {
        self.workflow_stale_denial_count
    }

    pub fn workflow_lowering_staleness_denial_count(&self) -> usize {
        self.workflow_lowering_staleness_denial_count
    }

    pub fn workflow_explicit_rebind_required_count(&self) -> usize {
        self.workflow_explicit_rebind_required_count
    }

    pub fn workflow_authority_override_denial_count(&self) -> usize {
        self.workflow_authority_override_denial_count
    }

    pub fn workflow_ambient_basis_fallback_denial_count(&self) -> usize {
        self.workflow_ambient_basis_fallback_denial_count
    }

    pub fn workflow_replay_bundle_count(&self) -> usize {
        self.workflow_replay_bundle_count
    }

    pub fn workflow_budget_cross_count(&self) -> usize {
        self.workflow_budget_cross_count
    }

    pub fn workflow_work_avoided_by_query_lowering_count(&self) -> usize {
        self.workflow_work_avoided_by_query_lowering_count
    }

    pub fn workflow_executor_rediscovery_count(&self) -> usize {
        self.workflow_executor_rediscovery_count
    }

    pub(crate) fn with_replay_bundle_issued(&self) -> Self {
        Self {
            workflow_replay_bundle_count: self.workflow_replay_bundle_count + 1,
            ..self.clone()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowInspectionBudget {
    ConflictInspectionNarrow,
    PostMergeInspectionNarrow,
}

impl WorkflowInspectionBudget {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConflictInspectionNarrow => "conflict_inspection_narrow",
            Self::PostMergeInspectionNarrow => "post_merge_inspection_narrow",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorkflowBudgetOutcome {
    WithinBudget,
    ExplicitBroadeningDenied,
    ExplicitRebindRequired,
}

impl WorkflowBudgetOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::ExplicitBroadeningDenied => "explicit_broadening_denied",
            Self::ExplicitRebindRequired => "explicit_rebind_required",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowPredictionReport {
    pub(crate) predicted_declaration_width: usize,
    pub(crate) predicted_inspection_width: usize,
    pub(crate) predicted_lowering_width: usize,
    pub(crate) predicted_freshness_width: usize,
    pub(crate) predicted_denial_width: usize,
}

impl WorkflowPredictionReport {
    pub fn predicted_declaration_width(&self) -> usize {
        self.predicted_declaration_width
    }

    pub fn predicted_inspection_width(&self) -> usize {
        self.predicted_inspection_width
    }

    pub fn predicted_lowering_width(&self) -> usize {
        self.predicted_lowering_width
    }

    pub fn predicted_freshness_width(&self) -> usize {
        self.predicted_freshness_width
    }

    pub fn predicted_denial_width(&self) -> usize {
        self.predicted_denial_width
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorkflowInspectionCounters {
    pub(crate) workflow_inspection_count: usize,
    pub(crate) workflow_conflict_inspection_count: usize,
    pub(crate) workflow_post_merge_inspection_count: usize,
    pub(crate) workflow_inspection_row_width: usize,
    pub(crate) workflow_inspection_merge_class_width: usize,
    pub(crate) workflow_inspection_denial_width: usize,
    pub(crate) workflow_executor_rediscovery_count: usize,
}

impl WorkflowInspectionCounters {
    pub fn workflow_inspection_count(&self) -> usize {
        self.workflow_inspection_count
    }

    pub fn workflow_conflict_inspection_count(&self) -> usize {
        self.workflow_conflict_inspection_count
    }

    pub fn workflow_post_merge_inspection_count(&self) -> usize {
        self.workflow_post_merge_inspection_count
    }

    pub fn workflow_inspection_row_width(&self) -> usize {
        self.workflow_inspection_row_width
    }

    pub fn workflow_inspection_merge_class_width(&self) -> usize {
        self.workflow_inspection_merge_class_width
    }

    pub fn workflow_inspection_denial_width(&self) -> usize {
        self.workflow_inspection_denial_width
    }

    pub fn workflow_executor_rediscovery_count(&self) -> usize {
        self.workflow_executor_rediscovery_count
    }
}
