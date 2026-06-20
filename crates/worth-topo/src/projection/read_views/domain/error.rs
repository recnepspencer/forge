use forge_query::facade::{
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenialKind,
    ForgeQueryRuntimeError,
};

use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyReadErrorKind {
    CanonicalLoweringResolution,
    ReadFamilyExecutionDenied,
    RuntimeBoundaryAuthorityDenied,
    UnsupportedTraversalDepth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadError {
    kind: TopologyReadErrorKind,
    detail: String,
    graph_access_denial: Option<TopologyReadGraphAccessDenial>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadBudgetExceeded {
    max_inline_index_bytes: usize,
    estimated_index_bytes: usize,
    max_inline_result_bytes: usize,
    estimated_result_bytes: usize,
    max_inline_intermediate_set_size: usize,
    estimated_intermediate_set_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadGraphAccessDenial {
    denial_kind: ForgeQueryGraphReadAccessDenialKind,
    suggested_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    budget_exceeded: Option<TopologyReadBudgetExceeded>,
    executor_entry_count: Option<usize>,
    materialized_row_count: Option<usize>,
}

impl TopologyReadError {
    pub(crate) fn canonical_lowering_resolution(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyReadErrorKind::CanonicalLoweringResolution,
            detail: detail.into(),
            graph_access_denial: None,
        }
    }

    pub(crate) fn read_family_execution_denied(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyReadErrorKind::ReadFamilyExecutionDenied,
            detail: detail.into(),
            graph_access_denial: None,
        }
    }

    pub(crate) fn runtime_boundary_authority_denied(detail: impl Into<String>) -> Self {
        Self {
            kind: TopologyReadErrorKind::RuntimeBoundaryAuthorityDenied,
            detail: detail.into(),
            graph_access_denial: None,
        }
    }

    pub(crate) fn unsupported_traversal_depth(
        request_family: TopologyReadRequestFamily,
        requested_depth: usize,
        maximum_supported_depth: usize,
    ) -> Self {
        Self {
            kind: TopologyReadErrorKind::UnsupportedTraversalDepth,
            detail: format!(
                "unsupported traversal depth `{requested_depth}` for `{request_family:?}`; maximum supported depth is `{maximum_supported_depth}`"
            ),
            graph_access_denial: None,
        }
    }

    pub(crate) fn from_query_runtime_error(error: ForgeQueryRuntimeError) -> Self {
        match error {
            ForgeQueryRuntimeError::ReadCompositionDenied(denial) => {
                let graph_access_denial = denial
                    .graph_read_access_admission()
                    .and_then(|admission| admission.denial())
                    .map(|access_denial| {
                        let budget_exceeded = access_denial.budget_exceeded().map(|budget| {
                            TopologyReadBudgetExceeded {
                                max_inline_index_bytes: budget.max_inline_index_bytes(),
                                estimated_index_bytes: budget.estimated_index_bytes(),
                                max_inline_result_bytes: budget.max_inline_result_bytes(),
                                estimated_result_bytes: budget.estimated_result_bytes(),
                                max_inline_intermediate_set_size: budget
                                    .max_inline_intermediate_set_size(),
                                estimated_intermediate_set_size: budget
                                    .estimated_intermediate_set_size(),
                            }
                        });
                        let execution_counters = denial.graph_read_access_execution_counters();
                        TopologyReadGraphAccessDenial {
                            denial_kind: access_denial.kind().clone(),
                            suggested_posture: access_denial.suggested_posture().clone(),
                            budget_exceeded,
                            executor_entry_count: execution_counters
                                .map(|counters| counters.executor_entry_count()),
                            materialized_row_count: execution_counters
                                .map(|counters| counters.materialized_row_count()),
                        }
                    });
                Self {
                    kind: TopologyReadErrorKind::ReadFamilyExecutionDenied,
                    detail: format!("{denial:?}"),
                    graph_access_denial,
                }
            }
            other => Self::read_family_execution_denied(format!("{other:?}")),
        }
    }

    pub fn kind(&self) -> TopologyReadErrorKind {
        self.kind
    }

    pub fn graph_access_denial(&self) -> Option<&TopologyReadGraphAccessDenial> {
        self.graph_access_denial.as_ref()
    }
}

impl TopologyReadGraphAccessDenial {
    pub fn denial_kind(&self) -> &ForgeQueryGraphReadAccessDenialKind {
        &self.denial_kind
    }

    pub fn suggested_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.suggested_posture
    }

    pub fn budget_exceeded(&self) -> Option<&TopologyReadBudgetExceeded> {
        self.budget_exceeded.as_ref()
    }

    pub fn executor_entry_count(&self) -> Option<usize> {
        self.executor_entry_count
    }

    pub fn materialized_row_count(&self) -> Option<usize> {
        self.materialized_row_count
    }
}

impl TopologyReadBudgetExceeded {
    pub fn max_inline_index_bytes(&self) -> usize {
        self.max_inline_index_bytes
    }

    pub fn estimated_index_bytes(&self) -> usize {
        self.estimated_index_bytes
    }

    pub fn max_inline_result_bytes(&self) -> usize {
        self.max_inline_result_bytes
    }

    pub fn estimated_result_bytes(&self) -> usize {
        self.estimated_result_bytes
    }

    pub fn max_inline_intermediate_set_size(&self) -> usize {
        self.max_inline_intermediate_set_size
    }

    pub fn estimated_intermediate_set_size(&self) -> usize {
        self.estimated_intermediate_set_size
    }
}

impl std::fmt::Display for TopologyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.detail.as_str())
    }
}

impl std::error::Error for TopologyReadError {}
