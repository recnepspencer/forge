use forge_store_wal::WalReplayTailCursorReport;

fn main() {
    let _ = WalReplayTailCursorReport {
        first_lsn: 1,
        end_lsn: 2,
        segment_count: 1,
        ordered_range_count: 1,
        ordering_proof: todo!(),
    };
}
