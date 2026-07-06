use std::sync::OnceLock;

use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity,
    admit_topology_derived_invalidation_prior_proof_identity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use forge_query::facade::ForgeQueryApplicationFacade;

use super::current_invalidation_proof::{
    current_topology_invalidation_proof, CurrentTopologyInvalidationProofError,
};
use crate::facade::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
    TopologyRewireLoopSuccessorProgramDeclaration, TopologyUndoFamilyExecutionError,
    TopologyUndoScopeProduct, TopologyUndoScopeProductCounters,
};
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
};
use crate::topology_operators::{
    topology_operator_contribution_workflow, TopologyOperatorWorkflowHandleExt,
};
use crate::undo_family_catalog::TopologyUndoFamilyIdentityAuthority;

use super::{
    lower_topology_undo_equivalence_basis_from_admitted_input,
    lower_topology_undo_scope_identity_from_admitted_input, TopologyUndoSemanticGraphAdmittedInput,
};

#[derive(Clone, Debug)]
pub struct CurrentReplayUndoTopologyUndoScopeBoundary {
    touched_closure: DerivedInvalidationTouchedClosure,
    selected_plan: DerivedInvalidationSelectedPlan,
    prior_proof_identity_digest: String,
    stage_index_identity_digest: String,
    semantic_graph_identity: String,
    scope_identity_digest: String,
    boundary_digest: String,
}

#[derive(Clone, Debug)]
pub struct CurrentReplayUndoTopologyBoundary {
    undo_scope_boundary: CurrentReplayUndoTopologyUndoScopeBoundary,
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
    static CACHE: OnceLock<CurrentReplayUndoTopologyBoundary> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let undo_scope_boundary = current_replay_undo_topology_undo_scope_boundary()?;
    let declaration = current_replay_undo_declaration()?;
    let graph_obligation_envelope_digest =
        current_replay_undo_graph_obligation_digest(&declaration)?;
    let operator_touched_basis_digest = undo_scope_boundary
        .touched_closure()
        .basis_digest()
        .to_string();
    let boundary_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:current-replay-undo-topology-boundary:v1".to_string(),
            format!("operator-touch:{operator_touched_basis_digest}"),
            format!(
                "touched-closure:{}",
                undo_scope_boundary.touched_closure().closure_digest()
            ),
            format!(
                "undo-scope-boundary:{}",
                undo_scope_boundary.boundary_digest()
            ),
            format!("graph-obligation:{graph_obligation_envelope_digest}"),
        ],
    );
    let boundary = CurrentReplayUndoTopologyBoundary {
        undo_scope_boundary,
        operator_touched_basis_digest,
        graph_obligation_envelope_digest,
        boundary_digest,
    };
    let _ = CACHE.set(boundary.clone());
    Ok(boundary)
}

pub fn current_replay_undo_topology_undo_scope_boundary(
) -> Result<CurrentReplayUndoTopologyUndoScopeBoundary, CurrentReplayUndoTopologyBoundaryError> {
    static CACHE: OnceLock<CurrentReplayUndoTopologyUndoScopeBoundary> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let proof = current_topology_invalidation_proof().map_err(from_invalidation_proof_error)?;
    let admitted_input = current_replay_undo_topology_ordinary_admitted_input(
        proof.touched_closure(),
        proof.selected_plan(),
    );
    let scope_identity = lower_topology_undo_scope_identity_from_admitted_input(&admitted_input);
    let boundary_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:current-replay-undo-topology-undo-scope-boundary:v1".to_string(),
            format!(
                "selected-plan:{}",
                proof.selected_plan().selected_plan_digest()
            ),
            format!(
                "touched-closure:{}",
                proof.touched_closure().closure_digest()
            ),
            format!(
                "prior-proof:{}",
                admitted_input.prior_proof_identity().digest()
            ),
            format!(
                "stage-index:{}",
                admitted_input.stage_index_identity().digest()
            ),
            format!("undo-scope:{}", scope_identity.digest()),
        ],
    );
    let boundary = CurrentReplayUndoTopologyUndoScopeBoundary {
        touched_closure: proof.touched_closure().clone(),
        selected_plan: proof.selected_plan().clone(),
        prior_proof_identity_digest: admitted_input.prior_proof_identity().digest().to_string(),
        stage_index_identity_digest: admitted_input.stage_index_identity().digest().to_string(),
        semantic_graph_identity: admitted_input.semantic_graph_identity().to_string(),
        scope_identity_digest: scope_identity.digest().to_string(),
        boundary_digest,
    };
    let _ = CACHE.set(boundary.clone());
    Ok(boundary)
}

impl CurrentReplayUndoTopologyUndoScopeBoundary {
    pub fn touched_closure(&self) -> &DerivedInvalidationTouchedClosure {
        &self.touched_closure
    }

    pub fn selected_plan(&self) -> &DerivedInvalidationSelectedPlan {
        &self.selected_plan
    }

    pub fn prior_proof_identity_digest(&self) -> &str {
        &self.prior_proof_identity_digest
    }

    pub fn stage_index_identity_digest(&self) -> &str {
        &self.stage_index_identity_digest
    }

    pub fn semantic_graph_identity(&self) -> &str {
        &self.semantic_graph_identity
    }

    pub fn scope_identity_digest(&self) -> &str {
        &self.scope_identity_digest
    }

    pub fn boundary_digest(&self) -> &str {
        &self.boundary_digest
    }

    pub fn lower_undo_scope_product(
        &self,
    ) -> Result<TopologyUndoScopeProduct<'_>, TopologyUndoFamilyExecutionError> {
        let admitted_input = current_replay_undo_topology_ordinary_admitted_input(
            &self.touched_closure,
            &self.selected_plan,
        );
        let equivalence_basis =
            lower_topology_undo_equivalence_basis_from_admitted_input(&admitted_input);
        let scope_identity =
            lower_topology_undo_scope_identity_from_admitted_input(&admitted_input);
        let counters =
            TopologyUndoScopeProductCounters::new(equivalence_basis.touched_subjects().len());
        Ok(TopologyUndoScopeProduct::new(
            admitted_input.family_identity(),
            &self.touched_closure,
            admitted_input.prior_proof_identity().clone(),
            admitted_input.stage_index_identity().clone(),
            admitted_input.semantic_graph_identity().to_string(),
            counters,
            equivalence_basis,
            scope_identity,
        ))
    }
}

impl CurrentReplayUndoTopologyBoundary {
    pub fn touched_closure(&self) -> &DerivedInvalidationTouchedClosure {
        self.undo_scope_boundary.touched_closure()
    }

    pub fn selected_plan(&self) -> &DerivedInvalidationSelectedPlan {
        self.undo_scope_boundary.selected_plan()
    }

    pub fn lower_undo_scope_product(
        &self,
    ) -> Result<TopologyUndoScopeProduct<'_>, TopologyUndoFamilyExecutionError> {
        self.undo_scope_boundary.lower_undo_scope_product()
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

fn current_replay_undo_topology_ordinary_admitted_input<'a>(
    touched_closure: &'a DerivedInvalidationTouchedClosure,
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> TopologyUndoSemanticGraphAdmittedInput<'a> {
    let prior_proof_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:replay-undo-semantic-graph:ordinary-undo-prior-proof:v1".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", touched_closure.closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!(
                "legality-support:{}",
                selected_plan.legality_support_digest()
            ),
        ],
    );
    let prior_proof_identity =
        admit_topology_derived_invalidation_prior_proof_identity(&prior_proof_digest);
    let stage_index_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:replay-undo-semantic-graph:ordinary-undo-stage-index:v1".to_string(),
            format!(
                "family:{}",
                TopologyUndoFamilyIdentityAuthority::traversal_views()
                    .identity()
                    .as_str()
            ),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", touched_closure.closure_digest()),
            format!("prior-proof:{}", prior_proof_identity.digest()),
        ],
    );
    TopologyUndoSemanticGraphAdmittedInput::new(
        TopologyUndoFamilyIdentityAuthority::traversal_views().identity(),
        touched_closure,
        prior_proof_identity,
        admit_replay_undo_stage_index_identity(&stage_index_digest),
    )
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

    #[test]
    fn current_undo_scope_boundary_carries_undo_scope_inputs() {
        let boundary = current_replay_undo_topology_undo_scope_boundary()
            .expect("current replay/undo topology undo-scope boundary should assemble");

        assert!(!boundary.touched_closure().closure_digest().is_empty());
        assert!(!boundary.selected_plan().selected_plan_digest().is_empty());
        assert!(!boundary.prior_proof_identity_digest().is_empty());
        assert!(!boundary.stage_index_identity_digest().is_empty());
        assert!(!boundary.semantic_graph_identity().is_empty());
        assert!(!boundary.scope_identity_digest().is_empty());
        assert!(!boundary.boundary_digest().is_empty());
        assert!(boundary.lower_undo_scope_product().is_ok());
    }
}
