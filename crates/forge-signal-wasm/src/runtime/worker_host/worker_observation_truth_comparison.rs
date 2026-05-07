use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::core::RuntimeCore;

use super::{
    canonical_worker_certification_digest, WorkerRuntimeObservationTruthReport, WorkerRuntimeShell,
};

pub(crate) fn compare_worker_observation_truth(
    worker_shell: &WorkerRuntimeShell,
    compatibility_runtime: &RuntimeCore,
) -> Result<WorkerRuntimeObservationTruthReport, ForgeSignalJsError> {
    let worker_first_observation_digest =
        canonical_worker_certification_digest(&worker_shell.latest_observation_summary()?)?;
    let compatibility_mode_observation_digest =
        canonical_worker_certification_digest(&compatibility_runtime.latest_observation()?)?;

    Ok(WorkerRuntimeObservationTruthReport {
        observation_truth_matches: worker_first_observation_digest
            == compatibility_mode_observation_digest,
        worker_first_observation_digest,
        compatibility_mode_observation_digest,
    })
}
