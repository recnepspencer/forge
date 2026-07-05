use forge_store_io_scheduler::{
    admit_s11_operator_io_readiness_seed, S10CompactionIoReadinessHandoff,
};

fn main() {
    let compaction: S10CompactionIoReadinessHandoff = todo!();
    let _ = admit_s11_operator_io_readiness_seed(compaction);
}
