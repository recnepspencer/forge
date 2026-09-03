use crate::branch::{
    ProductBranchHeadProtection, ProductBranchReferenceCell, RuntimeWorldBranchAdmissionDenial,
};
use crate::lifecycle::RuntimeWorldBranchCreationOutcome;

use super::state::{HistoryInstalledForkedBranch, ObservedForkedBranch};

impl ObservedForkedBranch {
    /// Both denial arms keep the recovering operation reservation alive until
    /// the retained record exists; only the performed arm may release it before
    /// returning, because no recovery custody follows it.
    pub(super) fn install(
        self,
    ) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial> {
        let Self {
            state,
            snapshot,
            observation,
        } = self;
        let HistoryInstalledForkedBranch {
            destination,
            recovery,
            commit,
            publication,
            product_history,
            history: _,
            mut operation,
        } = state;
        let transfer = publication
            .into_product_head_transfer(commit.basis())
            .expect("reserved publication custody matches the destination basis");
        let protection =
            match ProductBranchHeadProtection::owner_issued(snapshot, transfer, product_history) {
                Ok(protection) => protection,
                Err(failure) => {
                    drop(observation);
                    let protection = failure.into_protection();
                    operation
                        .begin_recovery()
                        .expect("a retained branch attempt enters recovery");
                    return Ok(RuntimeWorldBranchCreationOutcome::ProductUnpublished(
                        super::recovery::retain_from_protection(recovery, protection),
                    ));
                }
            };
        let cell = match ProductBranchReferenceCell::new(protection) {
            Ok(cell) => cell,
            Err(failure) => {
                drop(observation);
                let protection = failure.into_protection();
                operation
                    .begin_recovery()
                    .expect("a retained branch attempt enters recovery");
                return Ok(RuntimeWorldBranchCreationOutcome::ProductUnpublished(
                    super::recovery::retain_from_protection(recovery, protection),
                ));
            }
        };
        destination
            .reservation
            .install(destination.branch, destination.lifecycle, cell)
            .expect("the reserved destination remains installable after owner settlement");
        drop(operation);
        Ok(RuntimeWorldBranchCreationOutcome::Performed(observation))
    }
}
