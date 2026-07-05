use forge_store_io_scheduler::S10BackupExportIoReadinessHandoff;
use forge_store_operations::admit_s10_compaction_io_readiness_seed;

fn main() {
    let backup: S10BackupExportIoReadinessHandoff = todo!();
    let _ = admit_s10_compaction_io_readiness_seed(backup);
}
