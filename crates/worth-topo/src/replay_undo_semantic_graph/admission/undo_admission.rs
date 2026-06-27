use schema::facade::platform::authority::replay_undo_semantic_graph_internal::{
    admit_replay_undo_stage_index_identity,
    admit_topology_derived_invalidation_prior_proof_identity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    TopologyUndoSemanticGraphAdmissionError, TopologyUndoSemanticGraphAdmissionRequest,
    TopologyUndoSemanticGraphAdmittedInput,
};
use crate::undo_family_catalog::current_topology_undo_family_catalog;

pub fn admit_topology_undo_semantic_graph_input<'a>(
    request: TopologyUndoSemanticGraphAdmissionRequest<'a>,
) -> Result<TopologyUndoSemanticGraphAdmittedInput<'a>, TopologyUndoSemanticGraphAdmissionError> {
    let catalog = current_topology_undo_family_catalog();
    let declaration = catalog.require_family(request.family_identity()).ok_or(
        TopologyUndoSemanticGraphAdmissionError::MissingUndoFamilyDeclaration {
            family_identity: request.family_identity(),
        },
    )?;
    if request.touched_closure().closure_digest()
        != request.invalidation_receipt().touched_closure_digest()
    {
        return Err(
            TopologyUndoSemanticGraphAdmissionError::InvalidationReceiptTouchedClosureMismatch {
                touched_closure_digest: request.touched_closure().closure_digest().to_string(),
                receipt_touched_closure_digest: request
                    .invalidation_receipt()
                    .touched_closure_digest()
                    .to_string(),
            },
        );
    }

    let stage_index_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:replay-undo-semantic-graph:undo-stage-index:v1".to_string(),
            format!("family:{}", declaration.identity().as_str()),
            format!(
                "selected-plan:{}",
                request.invalidation_receipt().selected_plan_digest()
            ),
            format!(
                "touched-closure:{}",
                request.touched_closure().closure_digest()
            ),
            format!(
                "invalidation-receipt:{}",
                request.invalidation_receipt().execution_receipt_digest()
            ),
        ],
    );

    Ok(TopologyUndoSemanticGraphAdmittedInput::new(
        declaration.identity(),
        request.touched_closure(),
        admit_topology_derived_invalidation_prior_proof_identity(
            request.invalidation_receipt().execution_receipt_digest(),
        ),
        admit_replay_undo_stage_index_identity(&stage_index_digest),
    ))
}
