use worth_store_certification::S6CompactionHandoffEvidence;
use worth_store_operations::admit_s10_compaction_io_readiness_seed;

fn main() {
    let evidence: S6CompactionHandoffEvidence = todo!();
    let _ = admit_s10_compaction_io_readiness_seed(evidence);
}
