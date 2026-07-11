use forge_store_recovery_physics::CrashBoundaryLayoutReport;

use super::super::{BlobPublicationCounterSnapshot, BlobPublicationDenial};

pub(crate) fn verify_replayable_report(
    report: &CrashBoundaryLayoutReport,
) -> Result<(), BlobPublicationDenial> {
    if report.replayable_durable_wal().is_some() {
        Ok(())
    } else {
        Err(BlobPublicationDenial::WalReplayEvidenceRequired {
            counters: BlobPublicationCounterSnapshot::start().with_denied_promotion(),
        })
    }
}

pub(crate) fn replayable_durable_wal<'a>(
    report: &'a CrashBoundaryLayoutReport,
) -> Option<&'a forge_store_recovery_physics::UnacknowledgedDurableWal> {
    report.replayable_durable_wal()
}
