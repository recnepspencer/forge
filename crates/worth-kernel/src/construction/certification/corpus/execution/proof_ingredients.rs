use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::outcome::PrimitiveConstructionPreparedOutcome;
use crate::construction::parity::{
    prepare_primitive_construction_branch_local_parity_report,
    prepare_primitive_construction_replay_parity_report, PrimitiveConstructionReplayParityReport,
};
use crate::construction::request::PrimitiveConstructionRequest;
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisError;

#[derive(Clone, Debug)]
pub(crate) struct PrimitiveConstructionCorpusExecutionProofIngredients {
    request: PrimitiveConstructionRequest,
    replay: PrimitiveConstructionReplayParityReport,
    replay_digest: String,
    branch_digest: String,
}

impl PrimitiveConstructionCorpusExecutionProofIngredients {
    pub(crate) fn request(&self) -> &PrimitiveConstructionRequest {
        &self.request
    }

    pub(crate) fn direct_outcome(&self) -> &PrimitiveConstructionPreparedOutcome {
        self.replay.direct_outcome()
    }

    pub(crate) fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub(crate) fn branch_digest(&self) -> &str {
        &self.branch_digest
    }
}

pub(crate) fn prepare_corpus_execution_proof_ingredients<E>(
    workspace: &mut ForgeQueryWorkspace,
    intent: PrimitiveConstructionIntent,
    replay_drift_error: impl FnOnce() -> E,
    branch_drift_error: impl FnOnce() -> E,
) -> Result<PrimitiveConstructionCorpusExecutionProofIngredients, E>
where
    E: From<PrimitiveConstructionRuntimeBasisError>,
{
    let replay = prepare_primitive_construction_replay_parity_report(intent.clone());
    if !replay.parity_verified() {
        return Err(replay_drift_error());
    }

    let branch =
        prepare_primitive_construction_branch_local_parity_report(workspace, intent.clone())
            .map_err(E::from)?;
    if !branch.parity_verified() {
        return Err(branch_drift_error());
    }

    let request = intent.request().clone();
    let replay_digest = replay.replay_outcome().outcome_digest().to_string();
    let branch_digest = branch
        .branch_preview_runtime_report()
        .outcome()
        .outcome_digest()
        .to_string();

    Ok(PrimitiveConstructionCorpusExecutionProofIngredients {
        request,
        replay,
        replay_digest,
        branch_digest,
    })
}
