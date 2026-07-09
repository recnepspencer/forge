use worth_query::facade::QueryDiffResultBundle;

fn main() {
    let _ = QueryDiffResultBundle {
        context: unsafe { std::mem::zeroed() },
        change_set: unsafe { std::mem::zeroed() },
        metadata: unsafe { std::mem::zeroed() },
        replay_digest: String::new(),
        counter_snapshot_digest: String::new(),
    };
}
