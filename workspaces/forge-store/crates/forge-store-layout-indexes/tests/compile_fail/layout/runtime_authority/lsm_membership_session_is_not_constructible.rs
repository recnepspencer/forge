use forge_store_lsm_authority::LsmMembershipSession;

fn forge() -> LsmMembershipSession {
    LsmMembershipSession {
        keys: panic!(),
        store: panic!(),
        store_binding: String::new(),
        segment_id: 1,
        generation: 1,
        replay_posture: panic!(),
        reopen_counters: panic!(),
    }
}

fn main() {
    let _ = forge();
}
