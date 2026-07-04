use std::sync::OnceLock;

use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_undo_scope_identity, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoSemanticGraphStageIndexIdentity, UndoScopeIdentity, UndoScopeIdentityInput,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity,
    admit_topology_derived_invalidation_prior_proof_identity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::current_invalidation_proof::{
    current_topology_invalidation_proof, CurrentTopologyInvalidationProofError,
};
use super::current_boundary::CurrentReplayUndoTopologyBoundaryError;
use super::lowering::lower_topology_touched_subjects;
use super::scope_product::TopologyUndoScopeProductCounters;
use super::TopologyUndoScopeProduct;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::undo_family_catalog::{TopologyUndoFamilyIdentity, TopologyUndoFamilyIdentityAuthority};

#[derive(Clone, Debug)]
pub struct CurrentReplayUndoTopologyOrdinaryUndoScopeBoundary {
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: DerivedInvalidationTouchedClosure,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    semantic_graph_identity: String,
    counters: TopologyUndoScopeProductCounters,
    equivalence_basis: ReplayUndoSemanticGraphEquivalenceBasis,
    scope_identity: UndoScopeIdentity,
    boundary_digest: String,
}

pub fn current_replay_undo_topology_ordinary_undo_scope_boundary(
) -> Result<CurrentReplayUndoTopologyOrdinaryUndoScopeBoundary, CurrentReplayUndoTopologyBoundaryError>
{
    static CACHE: OnceLock<CurrentReplayUndoTopologyOrdinaryUndoScopeBoundary> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let proof = current_topology_invalidation_proof().map_err(from_invalidation_proof_error)?;
    let selected_plan = proof.selected_plan();
    let family_identity = TopologyUndoFamilyIdentityAuthority::traversal_views().identity();
    let prior_proof_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:current-replay-undo-topology-ordinary-prior-proof:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!("legality-support:{}", selected_plan.legality_support_digest()),
        ],
    );
    let prior_proof_identity =
        admit_topology_derived_invalidation_prior_proof_identity(&prior_proof_digest);
    let stage_index_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:replay-undo-semantic-graph:ordinary-undo-stage-index:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("prior-proof:{}", prior_proof_identity.digest()),
        ],
    );
    let stage_index_identity = admit_replay_undo_stage_index_identity(&stage_index_digest);
    let equivalence_basis = ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
        lower_topology_touched_subjects(proof.touched_closure().basis()),
        prior_proof_identity.clone(),
        Some(stage_index_identity.clone()),
    );
    let scope_identity = admit_undo_scope_identity(UndoScopeIdentityInput::new(
        equivalence_basis.clone(),
    ));
    let counters =
        TopologyUndoScopeProductCounters::new(equivalence_basis.touched_subjects().len());
    let semantic_graph_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:replay-undo-semantic-graph:ordinary-undo-admitted-input:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("touched-closure:{}", proof.touched_closure().closure_digest()),
            format!("prior-proof:{}", prior_proof_identity.digest()),
            format!("stage-index:{}", stage_index_identity.digest()),
        ],
    );
    let boundary_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:current-replay-undo-topology-ordinary-undo-scope-boundary:v1".to_string(),
            format!("family:{}", family_identity.as_str()),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", proof.touched_closure().closure_digest()),
            format!("prior-proof:{}", prior_proof_identity.digest()),
            format!("stage-index:{}", stage_index_identity.digest()),
            format!("scope:{}", scope_identity.digest()),
        ],
    );
    let boundary = CurrentReplayUndoTopologyOrdinaryUndoScopeBoundary {
        family_identity,
        touched_closure: proof.touched_closure().clone(),
        prior_proof_identity,
        stage_index_identity,
        semantic_graph_identity,
        counters,
        equivalence_basis,
        scope_identity,
        boundary_digest,
    };
    let _ = CACHE.set(boundary.clone());
    Ok(boundary)
}

impl CurrentReplayUndoTopologyOrdinaryUndoScopeBoundary {
    pub fn touched_closure(&self) -> &DerivedInvalidationTouchedClosure {
        &self.touched_closure
    }

    pub fn boundary_digest(&self) -> &str {
        &self.boundary_digest
    }

    pub fn lower_undo_scope_product(&self) -> TopologyUndoScopeProduct<'_> {
        TopologyUndoScopeProduct::new(
            self.family_identity,
            &self.touched_closure,
            self.prior_proof_identity.clone(),
            self.stage_index_identity.clone(),
            self.semantic_graph_identity.clone(),
            self.counters.clone(),
            self.equivalence_basis.clone(),
            self.scope_identity.clone(),
        )
    }
}

fn from_invalidation_proof_error(
    error: CurrentTopologyInvalidationProofError,
) -> CurrentReplayUndoTopologyBoundaryError {
    CurrentReplayUndoTopologyBoundaryError::new(format!(
        "current replay/undo topology invalidation proof did not assemble: {}",
        error.detail()
    ))
}
