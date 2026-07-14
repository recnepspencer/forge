use forge_store_lsm_authority::LsmCompactionRecordSet;

fn forge() -> LsmCompactionRecordSet {
    LsmCompactionRecordSet {
        value: panic!(),
        generation: panic!(),
        tombstone: panic!(),
    }
}

fn main() {
    let _ = forge();
}
