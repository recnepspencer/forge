use worth_primitives::{truth_digest_parts, TruthDigestScope};

use forge_query::facade::ForgeQueryApplicationFacade;

use super::current_invalidation_proof::{
    current_topology_invalidation_proof, CurrentTopologyInvalidationProofError,
};
use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::facade::{
    lower_topology_undo_scope_product_from_traversal_views_request,
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
    TopologyRewireLoopSuccessorProgramDeclaration, TopologyUndoFamilyExecutionError,
    TopologyUndoScopeProduct, TraversalViewsRollbackRequest,
};
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
};
use crate::topology_operators::{
    topology_operator_contribution_workflow, TopologyOperatorWorkflowHandleExt,
};

#[derive(Clone, Debug)]
pub struct CurrentReplayUndoTopologyBoundary {
    touched_closure: DerivedInvalidationTouchedClosure,
    selected_plan: DerivedInvalidationSelectedPlan,
    invalidation_receipt: DerivedInvalidationExecutionReceipt,
    operator_touched_basis_digest: String,
    graph_obligation_envelope_digest: String,
    boundary_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentReplayUndoTopologyBoundaryError {
    detail: String,
}

pub fn current_replay_undo_topology_boundary(
) -> Result<CurrentReplayUndoTopologyBoundary, CurrentReplayUndoTopologyBoundaryError> {
    let proof = current_topology_invalidation_proof().map_err(from_invalidation_proof_error)?;
    let invalidation_receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan(
        proof.selected_plan(),
    )
    .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
        detail: format!("current replay/undo topology execution failed: {error:?}"),
    })?;
    let declaration = current_replay_undo_declaration()?;
    let graph_obligation_envelope_digest =
        current_replay_undo_graph_obligation_digest(&declaration)?;
    let operator_touched_basis_digest = proof.touched_closure().basis_digest().to_string();
    let boundary_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:current-replay-undo-topology-boundary:v1".to_string(),
            format!("operator-touch:{operator_touched_basis_digest}"),
            format!(
                "touched-closure:{}",
                proof.touched_closure().closure_digest()
            ),
            format!(
                "invalidation-receipt:{}",
                invalidation_receipt.execution_receipt_digest()
            ),
            format!("graph-obligation:{graph_obligation_envelope_digest}"),
        ],
    );
    Ok(CurrentReplayUndoTopologyBoundary {
        touched_closure: proof.touched_closure().clone(),
        selected_plan: proof.selected_plan().clone(),
        invalidation_receipt,
        operator_touched_basis_digest,
        graph_obligation_envelope_digest,
        boundary_digest,
    })
}

impl CurrentReplayUndoTopologyBoundary {
    pub fn touched_closure(&self) -> &DerivedInvalidationTouchedClosure {
        &self.touched_closure
    }

    pub fn selected_plan(&self) -> &DerivedInvalidationSelectedPlan {
        &self.selected_plan
    }

    pub fn lower_undo_scope_product(
        &self,
    ) -> Result<TopologyUndoScopeProduct<'_>, TopologyUndoFamilyExecutionError> {
        lower_topology_undo_scope_product_from_traversal_views_request(
            TraversalViewsRollbackRequest::new(&self.touched_closure, &self.invalidation_receipt),
        )
    }

    pub fn boundary_digest(&self) -> &str {
        &self.boundary_digest
    }

    pub fn operator_touched_basis_digest(&self) -> &str {
        &self.operator_touched_basis_digest
    }

    pub fn graph_obligation_envelope_digest(&self) -> &str {
        &self.graph_obligation_envelope_digest
    }
}

impl CurrentReplayUndoTopologyBoundaryError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn current_runtime_error(error: impl std::fmt::Debug) -> CurrentReplayUndoTopologyBoundaryError {
    CurrentReplayUndoTopologyBoundaryError {
        detail: format!("current replay/undo topology boundary did not assemble: {error:?}"),
    }
}

fn current_replay_undo_declaration(
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, CurrentReplayUndoTopologyBoundaryError> {
    let proof = current_topology_invalidation_proof().map_err(from_invalidation_proof_error)?;
    Ok(proof.declaration().clone())
}

fn current_replay_undo_graph_obligation_digest(
    declaration: &TopologyRewireLoopSuccessorProgramDeclaration,
) -> Result<String, CurrentReplayUndoTopologyBoundaryError> {
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .map_err(current_runtime_error)?
        .admit()
        .map_err(current_runtime_error)?;
    let artifact = handle
        .orchestrate_topology_operator_with_contributions(topology_operator_contribution_workflow(
            declaration.clone(),
        ))
        .map_err(current_contribution_error)?;
    artifact
        .graph_obligation_dispatch()
        .and_then(|dispatch| dispatch.envelope_digest())
        .map(str::to_string)
        .ok_or_else(|| CurrentReplayUndoTopologyBoundaryError {
            detail: "current replay/undo topology declaration must carry graph-obligation proof"
                .to_string(),
        })
}

fn current_contribution_error<I>(
    outcome: crate::topology_operators::TopologyOperatorContributionCheckedOutcome<I>,
) -> CurrentReplayUndoTopologyBoundaryError
where
    I: forge_query::facade::ForgeQueryDeclarationInput<crate::query_domain::TopologyQueryDomain>,
{
    let detail = match outcome {
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Bound(_) => {
            "unexpected non-error contribution outcome".to_string()
        }
        forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Deferred(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Stale(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::RebindRequired(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(value)
        | forge_query::facade::ForgeQueryContributionComposedOrchestrationOutcome::Failed(value) => {
            value.reason().to_string()
        }
    };
    CurrentReplayUndoTopologyBoundaryError {
        detail: format!(
            "current replay/undo topology contribution proof did not assemble: {detail}"
        ),
    }
}

fn from_invalidation_proof_error(
    error: CurrentTopologyInvalidationProofError,
) -> CurrentReplayUndoTopologyBoundaryError {
    CurrentReplayUndoTopologyBoundaryError {
        detail: format!(
            "current replay/undo topology invalidation proof did not assemble: {}",
            error.detail()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::current_replay_undo_topology_boundary;

    #[test]
    fn current_boundary_carries_operator_artifact_proof() {
        let boundary = current_replay_undo_topology_boundary()
            .expect("current replay/undo topology boundary should assemble");

        assert!(!boundary.touched_closure().closure_digest().is_empty());
        assert!(!boundary.selected_plan().selected_plan_digest().is_empty());
        assert!(!boundary.operator_touched_basis_digest().is_empty());
        assert!(!boundary.graph_obligation_envelope_digest().is_empty());
        assert!(!boundary.boundary_digest().is_empty());
    }
}
