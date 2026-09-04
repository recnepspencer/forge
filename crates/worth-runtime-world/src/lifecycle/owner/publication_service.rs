use super::RuntimeWorldOwnerRoot;

use crate::branch::ProductBranchReferenceCell;
use crate::lifecycle::ports::RuntimeWorldProductPublicationService;
use crate::publication::{
    CompositeLateCancellationPosture, CompositePublicationReady, RuntimeWorldPublicationOutcome,
};

impl<D, I, E, Ctx, T> RuntimeWorldProductPublicationService
    for RuntimeWorldOwnerRoot<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    /// Publish the ready commit into the branch cell, then tell the branch
    /// registry which head that cell now carries.
    ///
    /// The cell is the authority for the head; the registry's basis-to-commit
    /// index is derived from it and is what exact-reuse creation resolves
    /// against. Without this report the index would keep naming the commit the
    /// branch was installed with, and a creation from the published head would
    /// be denied for a head the owner itself had just published.
    fn publish(
        &self,
        ready: CompositePublicationReady,
        cell: &ProductBranchReferenceCell,
        late_cancellation: CompositeLateCancellationPosture,
    ) -> RuntimeWorldPublicationOutcome {
        let outcome = ready.publish(cell, late_cancellation);
        if let RuntimeWorldPublicationOutcome::Performed(performed) = &outcome {
            self.state
                .branches
                .record_published_head(performed.new_product_head());
        }
        outcome
    }
}

#[cfg(test)]
#[path = "publication_service_tests.rs"]
mod tests;
