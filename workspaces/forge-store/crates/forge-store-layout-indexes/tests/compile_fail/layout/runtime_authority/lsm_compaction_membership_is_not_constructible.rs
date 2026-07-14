use forge_store_lsm_authority::LsmCompactionMembership;

fn forge() -> LsmCompactionMembership {
    LsmCompactionMembership {
        key: panic!(),
        record_set: panic!(),
        base: panic!(),
        version: 1,
        store_binding: String::new(),
        partition_probes: 1,
        component_probes: 3,
    }
}

fn main() {
    let _ = forge();
}
