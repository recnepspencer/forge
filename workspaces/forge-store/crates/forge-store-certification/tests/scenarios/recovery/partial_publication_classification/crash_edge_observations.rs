use forge_store_test_support::harness::recovery::wal_durability as wal_durability_paths;

use forge_store_recovery_physics::{
    AcknowledgmentPrecondition, DurableAckReceipt, LogSequenceNumber, PartialPublicationCrashEdge,
    PartialPublicationPersistedBytes, WalLsnRange,
};

use wal_durability_paths::completed_posix_receipt_for_range;

pub(crate) fn before_wal_append_edge() -> PartialPublicationCrashEdge {
    PartialPublicationCrashEdge::before_wal_append("phase8-before-wal")
}

pub(crate) fn after_wal_append_before_durability_edge(
    start: u64,
    end_exclusive: u64,
) -> PartialPublicationCrashEdge {
    PartialPublicationCrashEdge::after_wal_append_before_durability(
        wal_range(start, end_exclusive),
        "phase8-before-durability",
    )
}

pub(crate) fn after_durability_before_ack_edge(
    start: u64,
    end_exclusive: u64,
) -> PartialPublicationCrashEdge {
    PartialPublicationCrashEdge::after_durability_before_ack(completed_posix_receipt_for_range(
        start,
        end_exclusive,
    ))
}

pub(crate) fn after_ack_before_page_flush_edge(
    start: u64,
    end_exclusive: u64,
) -> PartialPublicationCrashEdge {
    let ack = DurableAckReceipt::acknowledge(
        AcknowledgmentPrecondition::from_append_receipt(completed_posix_receipt_for_range(
            start,
            end_exclusive,
        ))
        .unwrap(),
    );
    PartialPublicationCrashEdge::after_ack_before_page_flush(ack.ack_basis().clone())
}

pub(crate) fn during_checkpoint_cutover_edge() -> PartialPublicationCrashEdge {
    PartialPublicationCrashEdge::during_checkpoint_cutover("phase8-checkpoint-cutover")
}

pub(crate) fn persisted_before_durability_bytes(
    start: u64,
    end_exclusive: u64,
) -> PartialPublicationPersistedBytes {
    PartialPublicationPersistedBytes::after_wal_append_before_durability(
        wal_range(start, end_exclusive),
        "phase8-before-durability",
    )
}

fn wal_range(start: u64, end_exclusive: u64) -> WalLsnRange {
    WalLsnRange::new(
        LogSequenceNumber::new(start),
        LogSequenceNumber::new(end_exclusive),
    )
    .unwrap()
}
