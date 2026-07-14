use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::core::RuntimeCore;

pub(crate) fn committed_truth_digest_for_runtime(
    runtime: &RuntimeCore,
) -> Result<String, WorthSignalJsError> {
    let branch = runtime.current_branch();
    let proof = runtime.branch_state_proof(branch.id.0)?;
    Ok(proof.state_digest)
}
