use worth_foundational::{FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow};

use crate::RecoveryCounterSnapshot;

pub(crate) fn recovery_performance_counter_rows(
    counters: RecoveryCounterSnapshot,
) -> Vec<FoundationalPerformanceCounterRow> {
    let mut rows = [
        (
            "recovery.replayed_frames",
            counters.replayed_frames() as u64,
        ),
        ("recovery.skipped_frames", counters.skipped_frames() as u64),
        (
            "recovery.validated_checkpoints",
            counters.validated_checkpoints(),
        ),
        (
            "recovery.scanned_segments",
            counters.scanned_segments() as u64,
        ),
        ("recovery.page_redos", counters.page_redos() as u64),
        (
            "recovery.memory_envelope_bytes",
            counters.memory_envelope_bytes(),
        ),
        (
            "recovery.memory_envelope_frames",
            counters.memory_envelope_frames() as u64,
        ),
        ("recovery.allocation_bytes", counters.allocation_bytes()),
        ("recovery.total_store_pages", counters.total_store_pages()),
        (
            "recovery.residue_rejections",
            counters.residue_rejections() as u64,
        ),
        (
            "recovery.verifier_forbidden_full_store_scans",
            counters.forbidden_full_store_scans(),
        ),
    ]
    .into_iter()
    .map(|(name, value)| {
        FoundationalPerformanceCounterRow::new(
            FoundationalPerformanceCounterName::new(name).expect("static counter name"),
            value,
        )
    })
    .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.name().cmp(right.name()));
    rows
}
