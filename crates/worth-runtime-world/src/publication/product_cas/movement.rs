use crate::branch::ProductBranchReferenceCell;
use crate::publication::custody::RetainedCommitDisposition;
use crate::recovery::ProductUnpublishedCause;

use super::super::{CompositeLateCancellationPosture, RuntimeWorldPublicationOutcome};
use super::CompositePublicationReadyInputs;

pub(super) fn attempt_product_movement(
    mut ready: CompositePublicationReadyInputs,
    cell: &ProductBranchReferenceCell,
    late: CompositeLateCancellationPosture,
) -> RuntimeWorldPublicationOutcome {
    match ready.custody.attempt_movement(
        &ready.expected_head,
        &ready.commit,
        &ready.owner_results,
        &mut ready.counters,
        late,
        cell,
    ) {
        Ok(performed) => RuntimeWorldPublicationOutcome::Performed(performed),
        Err(loss) => {
            ready.counters.record_cas_loss();
            RuntimeWorldPublicationOutcome::ProductUnpublished(ready.custody.retain(
                ProductUnpublishedCause::ProductPublicationLost,
                Some(loss.observed_head().clone()),
                RetainedCommitDisposition::ReleaseUnused,
            ))
        }
    }
}
