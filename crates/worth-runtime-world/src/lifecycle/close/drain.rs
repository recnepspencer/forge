use std::sync::atomic::Ordering;

use crate::lifecycle::owner::{RuntimeWorldBootstrapState, RuntimeWorldOwnerState};
use crate::recovery::{
    ProductUnpublishedNextAction, ProductUnpublishedOwnerEffects, ProductUnpublishedRecoveryHandle,
    ProductUnpublishedRetentionPosture,
};

use super::report::{
    RuntimeWorldCloseReleaseCounts, RuntimeWorldCloseReport, RuntimeWorldRetainedRecordReport,
};
use super::RuntimeWorldCloseDenial;

/// A retained record whose posture is `RetainedExact` keeps exactly one
/// relational and one signal pin live.
const RETAINED_EXACT_COMPONENT_PIN_PAIR: usize = 2;

/// Admit a close attempt. A declared critical section that is still in flight
/// cannot be drained, so it denies here rather than closing over live work.
///
/// An installed retained record is deliberately not a denial: close settles
/// what it can and exposes the rest in its terminal report.
pub(super) fn admit_close<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<(), RuntimeWorldCloseDenial>
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
    // Close stops new admission at the operation ledger. Publish the queued
    // waiter before blocking so the transition from "no close" to "a close is
    // admitting" is observable rather than inferred from elapsed time.
    state.close_admission_waiters.fetch_add(1, Ordering::SeqCst);
    let operation = state
        .operation
        .state
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    state.close_admission_waiters.fetch_sub(1, Ordering::SeqCst);
    if operation.recovery_active != 0 {
        return Err(RuntimeWorldCloseDenial::InFlightCriticalSection);
    }
    if operation.active != 0 {
        return Err(RuntimeWorldCloseDenial::AlreadyClosing);
    }
    drop(operation);
    if state.recovery.reserved_slots() != 0 {
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
pub(super) fn drain_for_close<D, I, E, Ctx, T>(
    state: &RuntimeWorldOwnerState<D, I, E, Ctx, T>,
) -> Result<RuntimeWorldCloseReport, RuntimeWorldCloseDenial>
where
    D: Copy + Ord + std::fmt::Debug + Send + Sync + 'static,
    I: Copy + Ord + Send + Sync + 'static,
    T: Copy + Ord + Send + Sync + 'static,
{
    let retained_records = enumerate_retained_records(state)?;
    let settled_records = retained_records
        .iter()
        .filter(|record| {
            !record
                .next_actions()
                .contains(&ProductUnpublishedNextAction::SettleOwnerEffects)
        })
        .count();
    let counts = RuntimeWorldCloseReleaseCounts {
        settled_records,
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

/// Split the record's own live obligation count into the exact component pins
/// it holds and the composite-scoped custody it holds. The two halves always
/// sum to the count the record reports, so the report cannot disagree with the
/// record it describes.
fn describe(effects: &ProductUnpublishedOwnerEffects) -> RuntimeWorldRetainedRecordReport {
    let live_component_obligations = match effects.retention_posture() {
        ProductUnpublishedRetentionPosture::RetainedExact => RETAINED_EXACT_COMPONENT_PIN_PAIR,
        ProductUnpublishedRetentionPosture::ReacquisitionPending => 0,
    };
    let live_obligations = effects.live_obligation_count();
    assert!(
        live_component_obligations <= live_obligations,
        "a retained record cannot hold more component pins than live obligations"
    );
    RuntimeWorldRetainedRecordReport::new(
        effects.identity().clone(),
        effects.cause(),
        live_component_obligations,
        live_obligations - live_component_obligations,
        effects.next_actions().to_vec(),
    )
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
