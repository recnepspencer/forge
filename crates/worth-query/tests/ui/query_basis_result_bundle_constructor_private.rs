use worth_query::facade::policy::QueryBasisResultBundle;

fn main() {
    let _ = QueryBasisResultBundle {
        context: unsafe { std::mem::zeroed() },
        execution: unsafe { std::mem::zeroed() },
        metadata: unsafe { std::mem::zeroed() },
        replay_digest: String::new(),
        counter_snapshot_digest: String::new(),
    };
}
