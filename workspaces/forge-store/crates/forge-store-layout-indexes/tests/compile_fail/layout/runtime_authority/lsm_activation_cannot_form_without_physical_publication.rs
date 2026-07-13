use forge_store_lsm_authority::{
    prepare_lsm_membership_activation, AdmittedLsmReplacementOutput, LsmCompactionMembership,
};

fn bypass(membership: &LsmCompactionMembership, output: AdmittedLsmReplacementOutput) {
    let _ = prepare_lsm_membership_activation(membership, output);
}

fn main() {}
