use crate::boundary::errors::WorthSignalJsError;
use crate::runtime::summaries::RuntimeAsyncLifecycleCertification;

use super::{canonical_worker_certification_digest, WorkerRuntimeAsyncLifecycleTruthReport};

pub(crate) fn compare_worker_async_lifecycle_truth(
    worker_async_lifecycle: RuntimeAsyncLifecycleCertification,
    compatibility_async_lifecycle: RuntimeAsyncLifecycleCertification,
) -> Result<WorkerRuntimeAsyncLifecycleTruthReport, WorthSignalJsError> {
    let worker_first_async_lifecycle_digest =
        canonical_worker_certification_digest(&worker_async_lifecycle)?;
    let compatibility_mode_async_lifecycle_digest =
        canonical_worker_certification_digest(&compatibility_async_lifecycle)?;

    Ok(WorkerRuntimeAsyncLifecycleTruthReport {
        async_lifecycle_truth_matches: worker_first_async_lifecycle_digest
            == compatibility_mode_async_lifecycle_digest,
        request_admitted: worker_async_lifecycle.request_admitted
            && compatibility_async_lifecycle.request_admitted,
        completion_committed: worker_async_lifecycle.completion_committed
            && compatibility_async_lifecycle.completion_committed,
        worker_first_async_lifecycle_digest,
        compatibility_mode_async_lifecycle_digest,
    })
}
