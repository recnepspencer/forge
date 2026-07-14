use worth_store_lsm_authority::LsmCompactionRecordSet;

fn worth() -> LsmCompactionRecordSet {
    LsmCompactionRecordSet {
        value: panic!(),
        generation: panic!(),
        tombstone: panic!(),
    }
}

fn main() {
    let _ = worth();
}
