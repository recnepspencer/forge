//! Native atlas settlement, rollback, and effects-indeterminate recovery.

use std::rc::Rc;

use super::admission::{candidate_entry_mut, next_entry_after_plan};
use super::ownership::UiNativeTextAtlas;
use super::recovery::{UiNativeTextAtlasDenial, UiNativeTextAtlasRecovery};
use super::settlement::{UiNativeTextAtlasCommitOutcome, UiNativeTextAtlasCommitReceipt};
use super::transaction::{
    UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasTransactionPlan, UiNativeTextAtlasUpload,
};

impl UiNativeTextAtlas {
    pub(crate) fn settle(
        &self,
        mut plan: UiNativeTextAtlasTransactionPlan,
        uploads: &[UiNativeTextAtlasUpload],
        external: UiNativeTextAtlasExternalOutcome,
    ) -> UiNativeTextAtlasCommitOutcome {
        if !Rc::ptr_eq(&self.core, &plan.core) {
            return UiNativeTextAtlasCommitOutcome::Denied(UiNativeTextAtlasDenial::StalePlan);
        }
        let core_handle = Rc::clone(&plan.core);
        if plan_is_stale(&core_handle, &plan) {
            release_plan(&mut plan, &core_handle);
            return UiNativeTextAtlasCommitOutcome::Denied(UiNativeTextAtlasDenial::StalePlan);
        }
        if external == UiNativeTextAtlasExternalOutcome::Rejected {
            release_plan(&mut plan, &core_handle);
            return UiNativeTextAtlasCommitOutcome::Denied(UiNativeTextAtlasDenial::UploadRejected);
        }
        if !plan.admits_uploads(uploads) {
            return UiNativeTextAtlasCommitOutcome::Denied(
                UiNativeTextAtlasDenial::RasterBatchMismatch,
            );
        }
        match external {
            UiNativeTextAtlasExternalOutcome::Rejected => {
                unreachable!("rejected was settled above")
            }
            UiNativeTextAtlasExternalOutcome::EffectsIndeterminate => {
                recover_indeterminate(&mut plan, &core_handle)
            }
            UiNativeTextAtlasExternalOutcome::Submitted => {
                commit_submitted(&mut plan, &core_handle, uploads)
            }
        }
    }
}

fn plan_is_stale(
    core_handle: &Rc<std::cell::RefCell<super::ownership::AtlasCore>>,
    plan: &UiNativeTextAtlasTransactionPlan,
) -> bool {
    let core = core_handle.borrow();
    core.reservation != Some(plan.reservation) || core.generation != plan.predecessor_generation
}

fn release_plan(
    plan: &mut UiNativeTextAtlasTransactionPlan,
    core_handle: &Rc<std::cell::RefCell<super::ownership::AtlasCore>>,
) {
    plan.committed = true;
    core_handle.borrow_mut().reservation = None;
}

fn recover_indeterminate(
    plan: &mut UiNativeTextAtlasTransactionPlan,
    core_handle: &Rc<std::cell::RefCell<super::ownership::AtlasCore>>,
) -> UiNativeTextAtlasCommitOutcome {
    let recovery = UiNativeTextAtlasRecovery::from_native_host(
        plan.demand_identity,
        plan.candidate_generation,
        core_handle.borrow().lineage,
    );
    {
        let mut core = core_handle.borrow_mut();
        core.next_entry = core.next_entry.max(next_entry_after_plan(plan));
        core.quarantined = true;
        core.reservation = None;
    }
    plan.committed = true;
    UiNativeTextAtlasCommitOutcome::EffectsIndeterminate(recovery)
}

fn commit_submitted(
    plan: &mut UiNativeTextAtlasTransactionPlan,
    core_handle: &Rc<std::cell::RefCell<super::ownership::AtlasCore>>,
    uploads: &[UiNativeTextAtlasUpload],
) -> UiNativeTextAtlasCommitOutcome {
    update_candidate_digests(plan, uploads);
    let mut core = core_handle.borrow_mut();
    let committed_epoch = core.completed_use_epoch.saturating_add(1);
    std::mem::take(&mut plan.candidate_alpha).apply(&mut core.alpha);
    std::mem::take(&mut plan.candidate_color).apply(&mut core.color);
    stamp_used_entries(&mut core, plan, committed_epoch);
    for release in &plan.pin_releases {
        core.pins.remove(release);
    }
    for addition in &plan.pin_additions {
        core.pins.insert(*addition);
    }
    update_changed_pin_counts(&mut core, plan);
    core.next_entry = core.next_entry.max(next_entry_after_plan(plan));
    core.generation = plan.candidate_generation;
    core.completed_use_epoch = committed_epoch;
    core.reservation = None;
    plan.committed = true;
    UiNativeTextAtlasCommitOutcome::Committed(UiNativeTextAtlasCommitReceipt {
        generation: core.generation,
        misses: u32::try_from(plan.misses.len()).unwrap_or(u32::MAX),
        hits: u32::try_from(plan.hits.len()).unwrap_or(u32::MAX),
        evictions: u32::try_from(plan.evictions.len()).unwrap_or(u32::MAX),
        committed_pins: u32::try_from(core.pins.len()).unwrap_or(u32::MAX),
        staged_bytes: plan.staged_bytes,
        physical_staged_bytes: plan.physical_staged_bytes,
        peak_entries: plan.peak_entries,
        peak_texel_bytes: plan.peak_texel_bytes,
    })
}

fn stamp_used_entries(
    core: &mut super::ownership::AtlasCore,
    plan: &UiNativeTextAtlasTransactionPlan,
    completed_use_epoch: u64,
) {
    for key in plan
        .hits
        .iter()
        .copied()
        .chain(plan.misses.iter().map(|demand| demand.key()))
    {
        if let Some(entry) = candidate_entry_mut(&mut core.alpha, &mut core.color, key) {
            entry.completed_use_epoch = completed_use_epoch;
        }
    }
}

fn update_candidate_digests(
    plan: &mut UiNativeTextAtlasTransactionPlan,
    uploads: &[UiNativeTextAtlasUpload],
) {
    for upload in uploads {
        if let Some(entry) = plan
            .candidate_alpha
            .added_entry_mut(upload.key())
            .or_else(|| plan.candidate_color.added_entry_mut(upload.key()))
        {
            entry.digest = upload.digest();
            entry.bearing = upload.bearing();
            entry.staged_bytes = u64::try_from(upload.bytes().len()).unwrap_or(u64::MAX);
        }
    }
}

fn update_changed_pin_counts(
    core: &mut super::ownership::AtlasCore,
    plan: &UiNativeTextAtlasTransactionPlan,
) {
    for key in plan
        .pin_change_keys
        .iter()
        .copied()
        .chain(plan.misses.iter().map(|demand| demand.key()))
    {
        let count = u32::try_from(core.pins.iter().filter(|pin| pin.key_matches(key)).count())
            .unwrap_or(u32::MAX);
        if let Some(entry) = candidate_entry_mut(&mut core.alpha, &mut core.color, key) {
            entry.pin_count = count;
        }
    }
}
