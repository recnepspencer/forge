use crate::epoch::EpochRetryDecision;

use crate::{
    ExecutedIsolationReceipts, IsolationReadinessDenial, PhysicalIsolationCounterSnapshot,
};

pub(crate) fn project_closeout_counters(
    receipts: ExecutedIsolationReceipts<'_>,
) -> Result<PhysicalIsolationCounterSnapshot, IsolationReadinessDenial> {
    let stable = receipts.stable_read.counters();
    let compaction_pre = receipts.compaction.pre_cutover_read().counters();
    let compaction_post = receipts.compaction.post_cutover_read().counters();
    let checkpoint_pre = receipts.checkpoint.pre_publication_read().counters();
    let checkpoint_post = receipts.checkpoint.post_publication_read().counters();
    let reclaim = receipts.reclaim.counters();
    let publication = receipts.publication.counters();
    let epoch_retry = match receipts.epoch_freshness.freshness().decision() {
        EpochRetryDecision::Current => 0,
        EpochRetryDecision::Retry | EpochRetryDecision::RebindRequired => 1,
    };
    PhysicalIsolationCounterSnapshot::from_store_executed_counts(
        1,
        stable.blocking_io_events()
            + compaction_pre.blocking_io_events()
            + compaction_post.blocking_io_events()
            + checkpoint_pre.blocking_io_events()
            + checkpoint_post.blocking_io_events(),
        stable.retry_decisions()
            + compaction_pre.retry_decisions()
            + compaction_post.retry_decisions()
            + checkpoint_pre.retry_decisions()
            + checkpoint_post.retry_decisions()
            + epoch_retry,
        1,
        1,
        reclaim.executed_reachability_inputs(),
        publication.readiness_joins() + reclaim.blocked_reclaims(),
        reclaim.blocked_reclaims(),
        receipts
            .stable_read
            .read_plan_release()
            .protected_references_released(),
    )
}

pub(crate) fn proof_progression_identity(
    receipts: ExecutedIsolationReceipts<'_>,
    counters: PhysicalIsolationCounterSnapshot,
) -> u64 {
    let mut identity = 0x5355_0000_0000_0001_u64;
    identity = mix_u64(identity, counters.outcome_count());
    identity = mix_u64(identity, counters.wait_count());
    identity = mix_u64(identity, counters.retry_count());
    identity = mix_u64(identity, counters.latch_counter_rows());
    identity = mix_u64(identity, counters.reclaim_counter_rows());
    identity = mix_u64(identity, counters.protected_byte_footprint());
    identity = mix_u64(
        identity,
        receipts.stable_read.read_plan_release().root_epoch().get(),
    );
    identity = mix_u64(identity, receipts.publication.epochs().root().old().get());
    identity = mix_u64(identity, receipts.publication.epochs().root().new().get());
    identity = mix_u64(
        identity,
        u64::from(
            receipts
                .compaction
                .pre_cutover_reader_retained_old_structure(),
        ),
    );
    identity = mix_u64(
        identity,
        u64::from(
            receipts
                .checkpoint
                .post_publication_reader_observed_new_epoch(),
        ),
    );
    mix_u64(identity, receipts.reclaim.counters().candidate_ranges())
}

#[cfg(any(test, feature = "certification-authority"))]
pub(crate) fn foreground_reservation_test_progression_identity(
    counters: PhysicalIsolationCounterSnapshot,
) -> u64 {
    let mut identity = 0x5355_0000_0000_0002_u64;
    identity = mix_u64(identity, counters.outcome_count());
    identity = mix_u64(identity, counters.wait_count());
    identity = mix_u64(identity, counters.retry_count());
    identity = mix_u64(identity, counters.latch_counter_rows());
    identity = mix_u64(identity, counters.reclaim_counter_rows());
    mix_u64(identity, counters.protected_byte_footprint())
}

const fn mix_u64(mut digest: u64, value: u64) -> u64 {
    let bytes = value.to_le_bytes();
    let mut index = 0;
    while index < bytes.len() {
        digest ^= bytes[index] as u64;
        digest = digest.wrapping_mul(0x1000_0000_01b3);
        index += 1;
    }
    digest
}
