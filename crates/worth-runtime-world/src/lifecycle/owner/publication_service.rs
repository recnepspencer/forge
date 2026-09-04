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
    /// Publish the ready commit into the branch cell.
    ///
    /// The cell is the only authority for the head. Exact reuse resolves its
    /// commit from the observation the cell issued, so a movement has nothing
    /// to report to the registry and there is no window in which a derived
    /// index lags the head the cell already carries.
    fn publish(
        &self,
        ready: CompositePublicationReady,
        cell: &ProductBranchReferenceCell,
        late_cancellation: CompositeLateCancellationPosture,
    ) -> RuntimeWorldPublicationOutcome {
        ready.publish(cell, late_cancellation)
    }
}

#[cfg(test)]
#[path = "publication_service_tests.rs"]
mod tests;
