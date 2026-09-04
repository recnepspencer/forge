//! Product-reference movement for an attempt that still holds the exact head
//! it admitted against.
//!
//! Materialization happens here and only here: the reserved commit becomes an
//! installed entry under product-head history protection, the publication
//! retention becomes a product-head transfer, and the resulting authority is
//! swapped into the branch-local reference cell under the cell's own write
//! lock, with the movement record written before the swap.

use std::sync::Arc;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceMovement, ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::recovery::ProductUnpublishedCause;

use super::super::{
    CompositeLateCancellationPosture, PerformedCompositePublication, RuntimeWorldPublicationOutcome,
};
use super::retained::{AttemptTerminal, RetainedSuccessorCustody, UnmaterializedSuccessor};
use super::CompositePublicationReadyInputs;

/// Materialize the reserved commit as product-head authority and swap it into
/// the cell. Only an attempt that still held the exact expected head reaches
/// here, so a loss at the CAS means the cell moved inside this window.
pub(super) fn attempt_product_movement(
    ready: CompositePublicationReadyInputs,
    cell: &ProductBranchReferenceCell,
    late_cancellation: CompositeLateCancellationPosture,
) -> RuntimeWorldPublicationOutcome {
    let (successor, mut terminal) = AttemptTerminal::split(ready);
    let successor_snapshot = derive_successor_snapshot(&terminal.expected_head, &terminal.commit);
    let protection = install_successor_protection(successor, &terminal.commit, successor_snapshot);
    terminal.counters.record_history_slot_installed();
    terminal.counters.record_product_cell_touch();
    terminal.counters.record_cas_attempt();
    match cell.compare_and_publish(&terminal.expected_head, protection) {
        Ok(movement) => perform(terminal, movement, late_cancellation),
        Err(failure) => {
            terminal.counters.record_cas_loss();
            let (winner_head, custody) = RetainedSuccessorCustody::from_cas_loss(failure);
            terminal.retain(
                winner_head,
                ProductUnpublishedCause::ProductPublicationLost,
                custody,
            )
        }
    }
}

/// The attempt moved the product reference. Its reserved recovery slot and its
/// operation reservation release here; nothing retained is left behind.
fn perform(
    terminal: AttemptTerminal,
    movement: ProductBranchReferenceMovement,
    late_cancellation: CompositeLateCancellationPosture,
) -> RuntimeWorldPublicationOutcome {
    let mut counters = terminal.counters;
    counters.record_cas_win();
    RuntimeWorldPublicationOutcome::Performed(PerformedCompositePublication::owner_issued(
        terminal.expected_head,
        movement,
        terminal.commit,
        terminal.attempt_identity,
        terminal.owner_results,
        late_cancellation,
        counters,
    ))
}

fn derive_successor_snapshot(
    expected_head: &ProductBranchObservation,
    commit: &Arc<CompositeRuntimeWorldCommit>,
) -> ProductBranchReferenceSnapshot {
    ProductBranchReferenceSnapshot::owner_issued(
        expected_head.owner_identity(),
        expected_head.branch_identity().clone(),
        expected_head.lifecycle_incarnation(),
        expected_head
            .reference_generation()
            .advance()
            .expect("reference generation capacity was checked before owner effects"),
        Arc::clone(commit),
    )
    .expect("the ready commit and expected head share one owner and branch lineage")
}

fn install_successor_protection(
    successor: UnmaterializedSuccessor,
    commit: &Arc<CompositeRuntimeWorldCommit>,
    successor_snapshot: ProductBranchReferenceSnapshot,
) -> ProductBranchHeadProtection {
    let UnmaterializedSuccessor {
        history,
        reserved_commit_capacity,
        publication_retention,
    } = successor;
    let entry = reserved_commit_capacity
        .install(Arc::clone(commit))
        .expect("the ready commit matches its reserved history slot");
    let installed_rollback = history.arm_installed_commit_rollback(entry.identity());
    let product_history = history
        .protect_product_head(entry.commit())
        .expect("the installed ready commit admits product-head history protection");
    let transfer = publication_retention
        .into_product_head_transfer(commit.basis())
        .expect("ready publication retention is bound to the exact successor basis");
    let protection =
        ProductBranchHeadProtection::owner_issued(successor_snapshot, transfer, product_history)
            .expect("ready component and history custody match the successor image");
    installed_rollback.commit();
    protection
}
