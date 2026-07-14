use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphObligationExecutionCostClass, WorthQueryGraphObligationExecutionInput,
    WorthQueryGraphObligationExecutionScope,
};

use super::state_load_counters::WorthQueryGraphObligationStateLoadCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationStateLoadPlan {
    required_state_scope_count: usize,
    traversed_edge_count: usize,
    materialized_row_count: usize,
    plan_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationStateLoadPlan {
    pub fn from_execution_input(input: &WorthQueryGraphObligationExecutionInput) -> Self {
        let budget = input.executor_contract().execution_budget();
        let required_state_scope_count = state_scope_for_cost_class(budget.cost_class())
            .max(state_scope_for_execution_scope(budget.execution_scope()));
        let traversed_edge_count = 0;
        let materialized_row_count = 0;
        let plan_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationStateLoadPlan)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("input"),
                    input.input_evidence_digest(),
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("required_state_scope_count"),
                    required_state_scope_count,
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("traversed_edge_count"),
                    traversed_edge_count,
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("materialized_row_count"),
                    materialized_row_count,
                )
                .seal();
        Self {
            required_state_scope_count,
            traversed_edge_count,
            materialized_row_count,
            plan_digest,
        }
    }

    pub fn required_state_scope_count(&self) -> usize {
        self.required_state_scope_count
    }

    pub fn plan_digest(&self) -> &str {
        self.plan_digest.as_str()
    }

    pub fn counters_before_state_load(&self) -> WorthQueryGraphObligationStateLoadCounters {
        WorthQueryGraphObligationStateLoadCounters::new(
            self.required_state_scope_count,
            self.traversed_edge_count,
            self.materialized_row_count,
        )
    }
}

fn state_scope_for_cost_class(cost_class: WorthQueryGraphObligationExecutionCostClass) -> usize {
    match cost_class {
        WorthQueryGraphObligationExecutionCostClass::SelectionOnly => 0,
        WorthQueryGraphObligationExecutionCostClass::SparseTopology
        | WorthQueryGraphObligationExecutionCostClass::PolicyBasis => 1,
        WorthQueryGraphObligationExecutionCostClass::DenseTopology
        | WorthQueryGraphObligationExecutionCostClass::ConstructionContext => 2,
    }
}

fn state_scope_for_execution_scope(
    execution_scope: WorthQueryGraphObligationExecutionScope,
) -> usize {
    match execution_scope {
        WorthQueryGraphObligationExecutionScope::SelectionOnly => 0,
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind
        | WorthQueryGraphObligationExecutionScope::TouchedCollection
        | WorthQueryGraphObligationExecutionScope::TouchedAspect
        | WorthQueryGraphObligationExecutionScope::PolicyScope => 1,
        WorthQueryGraphObligationExecutionScope::CandidateTopologyComponent
        | WorthQueryGraphObligationExecutionScope::ConstructionFamily => 2,
    }
}
