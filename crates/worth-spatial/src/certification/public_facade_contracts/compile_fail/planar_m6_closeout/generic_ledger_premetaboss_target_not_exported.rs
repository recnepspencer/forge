use worth_spatial::facade::planar_m6_closeout::{
    M6PremetabossFamily, M6PremetabossPlatformTarget,
};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedger;

fn main() {
    let ledger = WorkloadEvidenceLedger::from_rows(Vec::new()).unwrap();
    let complete = ledger.certify_complete().unwrap();
    let _ = M6PremetabossPlatformTarget::from_complete_ledger(
        M6PremetabossFamily::BooleanReadinessFinalBoss,
        &complete,
    );
}
