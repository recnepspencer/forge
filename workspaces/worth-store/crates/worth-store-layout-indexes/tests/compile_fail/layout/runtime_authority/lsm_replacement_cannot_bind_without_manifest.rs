use worth_store_lsm_authority::{
    admit_lsm_membership_replacement, AdmittedLsmReplacementOutput, LsmCompactionMembership,
};

fn bypass(membership: &LsmCompactionMembership, output: AdmittedLsmReplacementOutput) {
    let _ = admit_lsm_membership_replacement(membership, output);
}

fn main() {}
