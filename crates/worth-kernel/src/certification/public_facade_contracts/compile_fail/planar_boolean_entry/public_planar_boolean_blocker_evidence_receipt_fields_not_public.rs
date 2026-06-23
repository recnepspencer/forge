use worth_kernel::workload_composition::PlanarBooleanBlockerEvidenceReceipt;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceSupport;

fn main() {
    let _ = PlanarBooleanBlockerEvidenceReceipt {
        blocker_digest: String::from("forged blocker digest"),
        support: WorkloadEvidenceSupport::Blocked,
    };
}
