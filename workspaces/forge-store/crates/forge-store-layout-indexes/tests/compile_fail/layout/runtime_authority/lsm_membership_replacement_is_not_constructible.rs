use forge_store_lsm_authority::AdmittedLsmMembershipReplacement;

fn forge() -> AdmittedLsmMembershipReplacement {
    AdmittedLsmMembershipReplacement {
        checkpoint: panic!(),
        output: panic!(),
    }
}

fn main() {
    let _ = forge();
}
