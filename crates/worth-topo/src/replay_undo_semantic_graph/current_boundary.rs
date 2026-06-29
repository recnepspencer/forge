use worth_primitives::{truth_digest_parts, TruthDigestScope};

use forge_query::facade::ForgeQueryApplicationFacade;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use crate::derived_invalidation_authority_inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_family_catalog::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalogCloseout,
};
use crate::facade::{
    topology_runtime, TopologyRuntimeAdapters,
    lower_topology_undo_scope_product_from_traversal_views_request,
    DerivedInvalidationDensityPolicy, DerivedInvalidationLegalitySupportEvidence,
    DerivedInvalidationQuerySupportEvidence, DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
    TopologyDeclaredTouchedGraphBasisProof, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyTouchedOperatingWorld, TopologyUndoFamilyExecutionError, TopologyUndoScopeProduct, TraversalViewsRollbackRequest,
};
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
    TopologyCurrentHeadReadHandleExt, TopologyReadAnchorIdentity,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::{
    topology_operator_contribution_workflow, TopologyOperatorWorkflowHandleExt,
};
use crate::validation_authority_inventory::WorthValidationAuthorityMilestoneEightSeedSummary;
use crate::validator_invariant_catalog::{
    current_worth_topology_legality_catalog_closeout, WorthTopologyLegalitySelectionCloseout,
    WorthTopologyValidatorRoutingClosure,
};

use super::current_boundary_support::{
    first_source_identity_for_relation_kind, successor_candidate_with_retained_predecessor,
    successor_relocation_declaration,
};

#[derive(Clone, Debug)]
pub struct CurrentReplayUndoTopologyBoundary {
    touched_closure: DerivedInvalidationTouchedClosure,
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
    let declaration = current_replay_undo_declaration()?;
    let proof = declaration
        .declared_touched_basis_proof(
            "topology.rewire_loop_successor_program",
            TopologyTouchedOperatingWorld::mainline(),
        )
        .map_err(current_runtime_error)?;
    let touched_closure = DerivedInvalidationTouchedClosure::from_declared_touch(&proof);
    let legality_support = legality_support_for_touch(&proof)?;
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
            detail: format!(
                "current derived invalidation authority inventory closeout failed: {error:?}"
            ),
        })?;
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
                detail: format!("current derived invalidation family catalog failed: {error:?}"),
            })?;
    let catalog_closeout =
        DerivedInvalidationFamilyCatalogCloseout::close(catalog).map_err(|error| {
            CurrentReplayUndoTopologyBoundaryError {
                detail: format!(
                    "current derived invalidation family catalog closeout failed: {error:?}"
                ),
            }
        })?;
    let selected_plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout,
        &touched_closure,
        &DerivedInvalidationQuerySupportEvidence::missing(),
        &legality_support,
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
        detail: format!("current replay/undo topology selected plan failed: {error:?}"),
    })?;
    let invalidation_receipt = DerivedInvalidationExecutionReceipt::execute_selected_plan(
        &selected_plan,
    )
    .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
        detail: format!("current replay/undo topology execution failed: {error:?}"),
    })?;
    let graph_obligation_envelope_digest =
        current_replay_undo_graph_obligation_digest(&declaration)?;
    let operator_touched_basis_digest = proof.basis_digest().to_string();
    let boundary_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:current-replay-undo-topology-boundary:v1".to_string(),
            format!("operator-touch:{operator_touched_basis_digest}"),
            format!("touched-closure:{}", touched_closure.closure_digest()),
            format!(
                "invalidation-receipt:{}",
                invalidation_receipt.execution_receipt_digest()
            ),
            format!("graph-obligation:{graph_obligation_envelope_digest}"),
        ],
    );
    Ok(CurrentReplayUndoTopologyBoundary {
        touched_closure,
        invalidation_receipt,
        operator_touched_basis_digest,
        graph_obligation_envelope_digest,
        boundary_digest,
    })
}

impl CurrentReplayUndoTopologyBoundary {
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

fn execute_current_replay_undo_topology_artifact(
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, CurrentReplayUndoTopologyBoundaryError> {
    let mut runtime = crate::validation::reference_integrity::build_milestone_one_runtime().map_err(
        |error| CurrentReplayUndoTopologyBoundaryError {
            detail: format!("current replay/undo topology runtime did not build: {error:?}"),
        },
    )?;
    let _verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "phase13.current-replay-undo-topology-boundary",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 5 },
    )
    .map_err(current_runtime_error)?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "phase13.current-replay-undo-topology-boundary")
            .map_err(current_runtime_error)?;
    let surfaces = declare_topology_query_surfaces(&mut workspace).map_err(current_runtime_error)?;
    let moved_half_edge_identity = first_source_identity_for_relation_kind(
        &workspace.read::<serde_json::Value>(surfaces.relations()),
        TopologyRelationKind::HalfEdgeNext,
    )?;
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .map_err(current_runtime_error)?
        .admit()
        .map_err(current_runtime_error)?;
    let local_rewire = handle
        .topology_reads(&mut workspace)
        .local_rewire_neighborhood(
            &TopologyReadAnchorIdentity::from_runtime_row_label(moved_half_edge_identity),
            6,
        )
        .map_err(current_runtime_error)?;
    let chosen_successor_identity = successor_candidate_with_retained_predecessor(
        &local_rewire,
        3,
        "phase 13 current replay/undo topology boundary",
    )?;
    successor_relocation_declaration(&local_rewire, &chosen_successor_identity)
}

fn current_runtime_error(
    error: impl std::fmt::Debug,
) -> CurrentReplayUndoTopologyBoundaryError {
    CurrentReplayUndoTopologyBoundaryError {
        detail: format!("current replay/undo topology boundary did not assemble: {error:?}"),
    }
}

fn legality_support_for_touch(
    proof: &TopologyDeclaredTouchedGraphBasisProof,
) -> Result<DerivedInvalidationLegalitySupportEvidence, CurrentReplayUndoTopologyBoundaryError> {
    let milestone_eight_summary =
        WorthValidationAuthorityMilestoneEightSeedSummary::current_imported_public_closeout();
    let routing_closure =
        WorthTopologyValidatorRoutingClosure::from_declared_touch(proof, &milestone_eight_summary)
            .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
                detail: format!("current replay/undo topology routing closure failed: {error:?}"),
            })?;
    let catalog_closeout = current_worth_topology_legality_catalog_closeout().map_err(|error| {
        CurrentReplayUndoTopologyBoundaryError {
            detail: format!("current topology legality catalog closeout failed: {error:?}"),
        }
    })?;
    let selection_closeout =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &catalog_closeout,
            &routing_closure,
        )
        .map_err(|error| CurrentReplayUndoTopologyBoundaryError {
            detail: format!("current replay/undo topology legality selection failed: {error:?}"),
        })?;
    Ok(
        DerivedInvalidationLegalitySupportEvidence::from_selected_legality_plan(
            selection_closeout.selected_plan(),
        ),
    )
}

fn current_replay_undo_declaration(
) -> Result<TopologyRewireLoopSuccessorProgramDeclaration, CurrentReplayUndoTopologyBoundaryError> {
    execute_current_replay_undo_topology_artifact()
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
        .orchestrate_topology_operator_with_contributions(
            topology_operator_contribution_workflow(declaration.clone()),
        )
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

#[cfg(test)]
mod tests {
    use super::current_replay_undo_topology_boundary;

    #[test]
    fn current_boundary_carries_operator_artifact_proof() {
        let boundary = current_replay_undo_topology_boundary()
            .expect("current replay/undo topology boundary should assemble");

        assert!(!boundary.operator_touched_basis_digest().is_empty());
        assert!(!boundary.graph_obligation_envelope_digest().is_empty());
        assert!(!boundary.boundary_digest().is_empty());
    }
}
