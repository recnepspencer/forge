use std::sync::atomic::Ordering;

use crate::lifecycle::owner::{RuntimeWorldBootstrapState, RuntimeWorldOwnerState};
use crate::recovery::{
    ProductUnpublishedOwnerEffects, ProductUnpublishedRecoveryHandle,
    ProductUnpublishedRetentionPosture,
};
use crate::retention::RetainedPartialRetentionObligation;

use super::report::{
    RuntimeWorldCloseReleaseCounts, RuntimeWorldCloseReport, RuntimeWorldRetainedRecordReport,
};
use super::RuntimeWorldCloseDenial;

/// A record in the `ReacquisitionPending` posture holds a
/// `ReservedComponentPinPairCapacity` instead of issued pins
/// (`recovery/product_unpublished.rs`): the reserved charge for exactly the
/// relational and signal scopes the retained posture holds as obligations.
/// Pinned by `close_reports_a_pending_record_as_a_reserved_component_pair`.
const RESERVED_COMPONENT_PIN_PAIR: usize = 2;

/// Close the owner: admit, drain, and flip Open -> Closing -> Closed while
/// holding one operation-admission guard across all three.
///
/// The guard is what makes the report complete. `admit_close` checks the
/// ledger and `RuntimeWorldCloseContract::begin` flips the state; a reservation
/// admitted between those two points would be closed over silently and would
/// be missing from the report, so nothing may be admitted in that window.
pub(super) fn close_owner<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<RuntimeWorldCloseReport, RuntimeWorldCloseDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let bootstrap = state
        .bootstrap
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    if *bootstrap == RuntimeWorldBootstrapState::InProgress {
        return Err(RuntimeWorldCloseDenial::AlreadyClosing);
    }
    drop(bootstrap);
    // Publish the queued waiter before blocking so the transition from "no
    // close" to "a close is admitting" is observable rather than inferred from
    // elapsed time.
    state.close_admission_waiters.fetch_add(1, Ordering::SeqCst);
    let operation = state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    state.close_admission_waiters.fetch_sub(1, Ordering::SeqCst);
    admit_close(
        operation.recovery_active,
        operation.active,
        state.recovery.reserved_slots(),
    )?;
    let report = drain_for_close(state)?;
    let mut close = state
        .close
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    close.begin()?;
    close.finish()?;
    drop(close);
    drop(operation);
    Ok(report)
}

/// Decide whether this owner can be drained at all, from the operation ledger
/// the caller is holding. A declared critical section that is still in flight
/// cannot be drained, so it denies here rather than closing over live work.
///
/// An installed retained record is deliberately not a denial: close settles
/// what it can and exposes the rest in its terminal report.
fn admit_close(
    recovery_active: usize,
    active: usize,
    reserved_recovery_slots: usize,
) -> Result<(), RuntimeWorldCloseDenial> {
    if recovery_active != 0 {
        return Err(RuntimeWorldCloseDenial::InFlightCriticalSection);
    }
    if active != 0 || reserved_recovery_slots != 0 {
        return Err(RuntimeWorldCloseDenial::AlreadyClosing);
    }
    Ok(())
}

/// Settle what close can settle, enumerate every retained owner obligation it
/// cannot, and release every pin the owner can still release.
///
/// SPEC-P4-008: an installed retained record is a report row, never a denial.
/// The only remaining denial here is a record whose critical section is still
/// in flight, which no report row can honestly describe.
fn drain_for_close<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<RuntimeWorldCloseReport, RuntimeWorldCloseDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let retained_records = enumerate_retained_records(state)?;
    let counts = RuntimeWorldCloseReleaseCounts {
        // Enumerating a retained record is exposure, never settlement. The
        // drain settles no record today: every record it can read survives the
        // close that named it, so the settled count is the zero it settled.
        settled_records: 0,
        // Product-head and observation pins are held by live product branch
        // cells and by caller-held observations. Neither is enumerable from the
        // owner state today, so close reports the zero it actually released
        // instead of a count inferred from a budget limit.
        released_product_head_pins: 0,
        released_observation_pins: 0,
        // Every installed commit is still protected, either by a product head
        // or by a retained record enumerated above, so close releases no
        // history protection.
        released_history_pins: 0,
        released_unique_component_pins: release_reclaimable_component_pins(state),
        retired_owner_created_custody: 0,
    };
    Ok(RuntimeWorldCloseReport::new(
        retained_records,
        counts,
        Vec::new(),
    ))
}

/// Name every retained record the catalog still holds. A record that is inside
/// its own update critical section cannot be read without racing that update,
/// so it denies rather than producing a row that may already be stale.
fn enumerate_retained_records<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<Vec<RuntimeWorldRetainedRecordReport>, RuntimeWorldCloseDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let affinity = state.recovery.affinity();
    let identities = state.recovery.identities();
    let mut rows = Vec::with_capacity(identities.len());
    for identity in identities {
        let handle = ProductUnpublishedRecoveryHandle::new(identity, affinity);
        let Some(record) = state.recovery.lookup_record(&handle) else {
            return Err(RuntimeWorldCloseDenial::InFlightCriticalSection);
        };
        rows.push(describe(
            &ProductUnpublishedOwnerEffects::from_catalog_record(record),
        ));
    }
    Ok(rows)
}

/// Split the record's own live obligation count into the component-scoped
/// charge it holds and the composite-scoped custody it holds. The split is
/// total by construction: the composite half is whatever the record's own count
/// leaves, so the report can never contradict the record it describes and never
/// panics on one it cannot describe.
fn describe(effects: &ProductUnpublishedOwnerEffects) -> RuntimeWorldRetainedRecordReport {
    let live_obligations = effects.live_obligation_count();
    let live_component_obligations = component_charge(effects).min(live_obligations);
    RuntimeWorldRetainedRecordReport::new(
        effects.identity().clone(),
        effects.cause(),
        live_component_obligations,
        live_obligations - live_component_obligations,
        effects.next_actions().to_vec(),
    )
}

/// How many component-scoped charges the record holds, read off its own
/// custody rather than restated as a shape literal.
fn component_charge(effects: &ProductUnpublishedOwnerEffects) -> usize {
    match effects.retention_obligation() {
        Some(obligation) => issued_component_pins(obligation),
        None => {
            debug_assert_eq!(
                effects.retention_posture(),
                ProductUnpublishedRetentionPosture::ReacquisitionPending
            );
            RESERVED_COMPONENT_PIN_PAIR
        }
    }
}

/// The exact pins a `RetainedPartialRetentionObligation` holds, counted from
/// the obligation itself.
fn issued_component_pins(obligation: &RetainedPartialRetentionObligation) -> usize {
    [obligation.relational(), obligation.signal()].len()
}

/// Release every exact component pin that no live dependency and no component
/// owner lease still holds. The count is what the registry actually reclaimed.
fn release_reclaimable_component_pins<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> usize
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let live = state.retention.unique_pin_count();
    state.retention.reclaim(live).reclaimed()
}
