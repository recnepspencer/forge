use forge_query::facade::{
    ForgeQueryCollection, ForgeQueryIntentExecution, ForgeQueryMutationReceipt, ForgeQueryRuntime,
};

fn main() {
    let runtime = ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([ForgeQueryCollection::new("Task", [])])
        .build()
        .unwrap();
    let execution = ForgeQueryIntentExecution::idempotent_noop(
        "strategy",
        "v1",
        "strategy-digest",
        "input-digest",
        "outcome-digest",
        ["invariant-ok"],
        "commit-1",
        "snapshot-1",
    );

    let _ = runtime.inspect(&execution);
    let _: ForgeQueryMutationReceipt = execution;
}
