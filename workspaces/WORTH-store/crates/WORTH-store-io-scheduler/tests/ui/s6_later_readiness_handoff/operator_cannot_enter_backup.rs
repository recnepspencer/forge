use worth_store_io_scheduler::S11OperatorIoReadinessHandoff;
use worth_store_operations::admit_s10_backup_export_io_readiness_seed;

fn main() {
    let operator: S11OperatorIoReadinessHandoff = todo!();
    let _ = admit_s10_backup_export_io_readiness_seed(operator);
}
