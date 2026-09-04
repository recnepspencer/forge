use crate::branch::registry::{
    ProductBranchSourceInstallDenial, ProductBranchSourceInstallFailure,
};
use crate::branch::{
    ProductBranchHeadProtection, ProductBranchReferenceCell, RuntimeWorldBranchAdmissionDenial,
};
use crate::lifecycle::owner::RuntimeWorldOperationReservation;
use crate::lifecycle::RuntimeWorldBranchCreationOutcome;
use crate::recovery::ProductUnpublishedCause;

use super::recovery::RetainedForkNaming;
use super::state::{ForkedBranchDestination, HistoryInstalledForkedBranch, ObservedForkedBranch};
use super::ForkedBranchRecoveryContext;

impl ObservedForkedBranch {
    /// Every retained arm keeps the recovering operation reservation alive
    /// until the retained record exists; only the performed arm may release it
    /// before returning, because no recovery custody follows it.
    ///
    /// The product reference is installed under the source head's guard. A
    /// source that moved after the fork's last recheck, or was retired, is
    /// refused there, and the fork it already performed is retained as a
    /// stale product head naming the winner, never installed as a child of a
    /// head that is no longer current.
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
            operation,
        } = state;
        let transfer = publication
            .into_product_head_transfer(commit.basis())
            .expect("reserved publication custody matches the destination basis");
        // Either protection step fails only by losing an owner-issued
        // authority, and hands the protection back so the fork is retained.
        let cell = ProductBranchHeadProtection::owner_issued(snapshot, transfer, product_history)
            .map_err(|failure| failure.into_protection())
            .and_then(|protection| {
                ProductBranchReferenceCell::new(protection)
                    .map_err(|failure| failure.into_protection())
            });
        let cell = match cell {
            Ok(cell) => cell,
            Err(protection) => {
                drop(observation);
                let naming = RetainedForkNaming::owner_lost();
                return Ok(retain_uninstalled(recovery, protection, operation, naming));
            }
        };
        let ForkedBranchDestination {
            branch,
            lifecycle,
            reservation,
        } = destination;
        #[cfg(test)]
        super::super::super::install_control::pause_before_source_guarded_install(
            branch.owner_identity(),
        );
        match reservation.install_from_source(&recovery.expected_head, branch, lifecycle, cell) {
            Ok(()) => {
                drop(operation);
                Ok(RuntimeWorldBranchCreationOutcome::Performed(observation))
            }
            Err(failure) => {
                drop(observation);
                Ok(retain_refused_install(failure, recovery, operation))
            }
        }
    }
}

/// The registry refused the installation under the source guard. The
/// destination reservation is released, the protection is taken back from
/// the cell nobody else holds, and the fork is retained as a stale product
/// head that names the head which displaced it, if one did.
fn retain_refused_install(
    failure: ProductBranchSourceInstallFailure,
    recovery: ForkedBranchRecoveryContext,
    operation: RuntimeWorldOperationReservation,
) -> RuntimeWorldBranchCreationOutcome {
    let ProductBranchSourceInstallFailure {
        reservation,
        denial,
        cell,
    } = failure;
    drop(reservation);
    let last_observed_head = match denial {
        ProductBranchSourceInstallDenial::Registry(denial) => {
            panic!(
                "the reserved destination remains installable after owner settlement: {denial:?}"
            )
        }
        ProductBranchSourceInstallDenial::SourceRetired => None,
        ProductBranchSourceInstallDenial::SourceDisplaced(observed) => Some(observed),
    };
    let naming = RetainedForkNaming {
        cause: ProductUnpublishedCause::StaleProductHead,
        last_observed_head,
    };
    let protection = cell
        .into_protection()
        .expect("a reference cell the registry refused has no other holder");
    retain_uninstalled(recovery, protection, operation, naming)
}

/// Retain a settled fork that has its destination protection but no installed
/// product reference. The recovering operation reservation is released only
/// after the record exists; dropping it earlier would clear the
/// close-admission ledger while no installed recovery slot yet denies
/// `close()`.
fn retain_uninstalled(
    recovery: ForkedBranchRecoveryContext,
    protection: ProductBranchHeadProtection,
    mut operation: RuntimeWorldOperationReservation,
    naming: RetainedForkNaming,
) -> RuntimeWorldBranchCreationOutcome {
    operation
        .begin_recovery()
        .expect("a retained branch attempt enters recovery");
    RuntimeWorldBranchCreationOutcome::ProductUnpublished(super::recovery::retain_from_protection(
        recovery, protection, naming,
    ))
}
