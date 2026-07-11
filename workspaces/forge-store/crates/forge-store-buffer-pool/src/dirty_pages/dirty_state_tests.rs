use crate::{
    AccessPolicyBufferLifecycleKind, DirtyPageAccessOrigin, DirtyShutdownPosture,
    ResidentFrameDenialKind,
};

use super::dirty_state_test_support::{admit_payload_frame, load_request, resident_frame_table};

#[test]
fn dirty_marking_is_resident_authority_state_with_exact_counts() {
    let mut table = resident_frame_table(8192, 2, 2);
    let admission = admit_payload_frame(&mut table, 7, 2, b"dirty-a");

    let first = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let second = table.mark_dirty(admission.resident_frame_token()).unwrap();

    assert_eq!(first.identity(), second.identity());
    assert_eq!(second.dirty_page_count().as_pages(), 1);
    assert_eq!(
        second.counters().dirty_bytes().as_bytes(),
        first.frame_size_bytes()
    );
    assert_eq!(table.counters().dirty_state().dirty_pages().as_pages(), 1);
    assert_eq!(table.dirty_counters().newly_dirty_count(), 1);
    assert_eq!(table.dirty_counters().already_dirty_count(), 1);
}

#[test]
fn dirty_page_state_derives_store_buffer_lifecycle_proof() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"dirty-lifecycle");

    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();

    assert_eq!(dirty.access_origin(), DirtyPageAccessOrigin::StoreBuffer);
    assert_eq!(
        dirty.access_policy_lifecycle_proof().kind(),
        AccessPolicyBufferLifecycleKind::DirtyPageTracked
    );
}

#[test]
fn mmap_dirty_page_state_derives_mmap_lifecycle_proof_until_publication() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"mmap-dirty");

    let mmap_dirty = table
        .mark_mmap_dirty(admission.resident_frame_token())
        .unwrap();
    let repeated_store_dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();

    assert_eq!(mmap_dirty.access_origin(), DirtyPageAccessOrigin::Mmap);
    assert_eq!(
        repeated_store_dirty.access_origin(),
        DirtyPageAccessOrigin::Mmap
    );
    assert_eq!(
        repeated_store_dirty.access_policy_lifecycle_proof().kind(),
        AccessPolicyBufferLifecycleKind::DirtyMmapPage
    );
}

#[test]
fn mmap_dirty_mark_upgrades_existing_store_buffer_dirty_lifecycle_proof() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"store-then-mmap-dirty");

    table.mark_dirty(admission.resident_frame_token()).unwrap();
    let mmap_dirty = table
        .mark_mmap_dirty(admission.resident_frame_token())
        .unwrap();

    assert_eq!(mmap_dirty.access_origin(), DirtyPageAccessOrigin::Mmap);
    assert_eq!(
        mmap_dirty.access_policy_lifecycle_proof().kind(),
        AccessPolicyBufferLifecycleKind::DirtyMmapPage
    );
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 1);
    assert_eq!(table.dirty_counters().already_dirty_count(), 1);
}

#[test]
fn dirty_budget_denies_before_second_page_is_admitted_dirty() {
    let mut table = resident_frame_table(8192, 2, 1);
    let first = admit_payload_frame(&mut table, 7, 2, b"dirty-a");
    let second = admit_payload_frame(&mut table, 8, 3, b"dirty-b");

    table.mark_dirty(first.resident_frame_token()).unwrap();
    let denial = table.mark_dirty(second.resident_frame_token()).unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPageBudgetExceeded
    );
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 1);
    assert_eq!(table.dirty_counters().dirty_budget_denial_count(), 1);
    assert_eq!(table.dirty_counters().write_scheduling_attempt_count(), 0);
}

#[test]
fn publication_scheduling_denies_when_pin_appears_after_plan() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"protected-dirty");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let pinned = table
        .lease_page(admission.resident_frame_token())
        .unwrap()
        .pin()
        .unwrap();
    std::mem::forget(pinned);

    let denial = table.record_dirty_write_scheduled(plan).unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPublicationBehindActiveLease
    );
    let counters = table.dirty_counters();
    assert_eq!(counters.write_scheduling_denial_count(), 1);
    assert_eq!(counters.write_scheduling_attempt_count(), 0);
    assert_eq!(counters.scheduled_not_durable_pages().as_pages(), 0);
    assert_eq!(table.pin_counters().denied_protected_mutation_count(), 1);
}

#[test]
fn dirty_publication_receipt_releases_budget_without_clean_shutdown_claim() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"publish-dirty");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();

    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    assert!(!plan.proves_durability());

    let receipt = table.record_dirty_write_scheduled(plan).unwrap();

    assert!(!receipt.proves_durability());
    assert_eq!(receipt.released_dirty_pages().as_pages(), 1);
    assert_eq!(receipt.counters().dirty_pages().as_pages(), 0);
    assert_eq!(
        receipt.counters().scheduled_not_durable_pages().as_pages(),
        1
    );
    assert_eq!(receipt.write_scheduling_attempt_count(), 1);

    let report = table.dirty_shutdown_closeout();

    assert_eq!(
        report.posture(),
        DirtyShutdownPosture::UnflushedDirtyPagesRemain
    );
    assert_eq!(report.unflushed_dirty_pages().as_pages(), 1);
    assert_eq!(report.counters().dirty_pages().as_pages(), 0);
    assert_eq!(
        report.counters().scheduled_not_durable_pages().as_pages(),
        1
    );
    assert!(!report.proves_durability());
}

#[test]
fn dirty_shutdown_reports_unflushed_state_without_discarding_it() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"shutdown-dirty");
    table.mark_dirty(admission.resident_frame_token()).unwrap();

    let report = table.dirty_shutdown_closeout();

    assert_eq!(
        report.posture(),
        DirtyShutdownPosture::UnflushedDirtyPagesRemain
    );
    assert_eq!(report.unflushed_dirty_pages().as_pages(), 1);
    assert!(!report.proves_durability());
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 1);
    assert_eq!(table.dirty_counters().dirty_shutdown_unflushed_count(), 1);
}

#[test]
fn dirty_unpublished_frame_cannot_be_reused_as_clean_residency() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"dirty-before-reuse");
    table.mark_dirty(admission.resident_frame_token()).unwrap();

    let denial = table
        .reuse_frame_slot(admission.slot(), load_request(8, 3, b"replacement"))
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyFrameUnpublished
    );
    assert_eq!(table.dirty_counters().dirty_discard_denial_count(), 1);
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 1);
}

#[test]
fn scheduled_dirty_publication_frame_cannot_be_reused_as_clean_residency() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"scheduled-reuse");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let receipt = table.record_dirty_write_scheduled(plan).unwrap();

    let denial = table
        .reuse_frame_slot(admission.slot(), load_request(8, 3, b"replacement"))
        .unwrap_err();

    assert!(!receipt.proves_durability());
    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyFrameUnpublished
    );
    assert_eq!(table.dirty_counters().dirty_discard_denial_count(), 1);
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 0);
    assert_eq!(
        table
            .dirty_counters()
            .scheduled_not_durable_pages()
            .as_pages(),
        1
    );
}

#[test]
fn dirty_after_scheduled_write_counts_one_unflushed_resident_frame() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"scheduled-redirty");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    table.record_dirty_write_scheduled(plan).unwrap();

    let redirty = table.mark_dirty(admission.resident_frame_token()).unwrap();

    assert_eq!(redirty.counters().dirty_pages().as_pages(), 1);
    assert_eq!(
        redirty.counters().scheduled_not_durable_pages().as_pages(),
        1
    );
    assert_eq!(
        redirty
            .counters()
            .scheduled_dirty_overlap_pages()
            .as_pages(),
        1
    );
    assert_eq!(redirty.counters().unflushed_dirty_pages().as_pages(), 1);

    let denial = table
        .reuse_frame_slot(admission.slot(), load_request(8, 3, b"replacement"))
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyFrameUnpublished
    );
    assert_eq!(table.dirty_counters().dirty_discard_denial_count(), 1);
}

#[test]
fn second_publication_behind_pending_write_does_not_double_count_unflushed_pages() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"scheduled-twice");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let first_plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    table.record_dirty_write_scheduled(first_plan).unwrap();
    let redirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let second_plan = table.plan_dirty_publication(redirty.identity()).unwrap();

    let second_receipt = table.record_dirty_write_scheduled(second_plan).unwrap();
    let report = table.dirty_shutdown_closeout();

    assert_eq!(second_receipt.counters().dirty_pages().as_pages(), 0);
    assert_eq!(
        second_receipt
            .counters()
            .scheduled_not_durable_pages()
            .as_pages(),
        1
    );
    assert_eq!(
        second_receipt
            .counters()
            .scheduled_dirty_overlap_pages()
            .as_pages(),
        0
    );
    assert_eq!(second_receipt.counters().publication_receipt_count(), 2);
    assert_eq!(
        report.posture(),
        DirtyShutdownPosture::UnflushedDirtyPagesRemain
    );
    assert_eq!(report.unflushed_dirty_pages().as_pages(), 1);
    assert_eq!(
        report.counters().scheduled_not_durable_pages().as_pages(),
        1
    );
    assert!(!report.proves_durability());
}

#[test]
fn dirty_publication_plan_stales_when_new_dirty_episode_starts() {
    let mut table = resident_frame_table(8192, 1, 1);
    let admission = admit_payload_frame(&mut table, 7, 2, b"stale-dirty-episode");
    let dirty = table.mark_dirty(admission.resident_frame_token()).unwrap();
    let first_plan = table.plan_dirty_publication(dirty.identity()).unwrap();
    let stale_plan = table.plan_dirty_publication(dirty.identity()).unwrap();

    table.record_dirty_write_scheduled(first_plan).unwrap();
    table.mark_dirty(admission.resident_frame_token()).unwrap();
    let denial = table.record_dirty_write_scheduled(stale_plan).unwrap_err();

    assert_eq!(
        denial.kind(),
        ResidentFrameDenialKind::DirtyPublicationPlanStale
    );
    assert_eq!(
        table.dirty_counters().stale_publication_plan_denial_count(),
        1
    );
    assert_eq!(table.dirty_counters().write_scheduling_attempt_count(), 1);
    assert_eq!(table.dirty_counters().publication_receipt_count(), 1);
    assert_eq!(table.dirty_counters().dirty_pages().as_pages(), 1);
}
