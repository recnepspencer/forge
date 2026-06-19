use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphObligationExecutionCostClass, ForgeQueryGraphObligationExecutionInput,
    ForgeQueryGraphObligationExecutionScope,
};

use super::state_load_counters::ForgeQueryGraphObligationStateLoadCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationStateLoadPlan {
    required_state_scope_count: usize,
    traversed_edge_count: usize,
    materialized_row_count: usize,
    plan_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationStateLoadPlan {
    pub fn from_execution_input(input: &ForgeQueryGraphObligationExecutionInput) -> Self {
        let budget = input.executor_contract().execution_budget();
        let required_state_scope_count = state_scope_for_cost_class(budget.cost_class())
            .max(state_scope_for_execution_scope(budget.execution_scope()));
        let traversed_edge_count = 0;
        let materialized_row_count = 0;
        let plan_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationStateLoadPlan)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("input"),
                    input.input_evidence_digest(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("required_state_scope_count"),
                    required_state_scope_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("traversed_edge_count"),
                    traversed_edge_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("materialized_row_count"),
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

    pub fn counters_before_state_load(&self) -> ForgeQueryGraphObligationStateLoadCounters {
        ForgeQueryGraphObligationStateLoadCounters::new(
            self.required_state_scope_count,
            self.traversed_edge_count,
            self.materialized_row_count,
        )
    }
}

fn state_scope_for_cost_class(cost_class: ForgeQueryGraphObligationExecutionCostClass) -> usize {
    match cost_class {
        ForgeQueryGraphObligationExecutionCostClass::SelectionOnly => 0,
        ForgeQueryGraphObligationExecutionCostClass::SparseTopology
        | ForgeQueryGraphObligationExecutionCostClass::PolicyBasis => 1,
        ForgeQueryGraphObligationExecutionCostClass::DenseTopology
        | ForgeQueryGraphObligationExecutionCostClass::ConstructionContext => 2,
    }
}

fn state_scope_for_execution_scope(
    execution_scope: ForgeQueryGraphObligationExecutionScope,
) -> usize {
    match execution_scope {
        ForgeQueryGraphObligationExecutionScope::SelectionOnly => 0,
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind
        | ForgeQueryGraphObligationExecutionScope::TouchedCollection
        | ForgeQueryGraphObligationExecutionScope::TouchedAspectPath
        | ForgeQueryGraphObligationExecutionScope::PolicyScope => 1,
        ForgeQueryGraphObligationExecutionScope::CandidateTopologyComponent
        | ForgeQueryGraphObligationExecutionScope::ConstructionFamily => 2,
    }
}
