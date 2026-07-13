use forge_store_layout_indexes::PreparedLsmCompaction;

fn forge() -> PreparedLsmCompaction {
    PreparedLsmCompaction {
        membership: panic!(),
        replay_tail: panic!(),
        output: panic!(),
        physical_intent: panic!(),
    }
}

fn main() {
    let _ = forge();
}
