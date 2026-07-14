use worth_store_lsm_authority::{admit_lsm_replacement_output, LsmCompactionMembership};
use worth_store_wal::AdmittedWalAppendReceipt;

fn bypass(membership: &LsmCompactionMembership, output: AdmittedWalAppendReceipt) {
    let _ = admit_lsm_replacement_output(membership, output);
}

fn main() {}
