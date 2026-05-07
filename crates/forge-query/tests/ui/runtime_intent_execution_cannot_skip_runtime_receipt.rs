use forge_query::facade::{
    ForgeQueryIntentExecution, ForgeQueryMutationReceipt, ForgeQueryRuntime,
};

fn main() {
    let runtime = fake_runtime();
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

fn fake_runtime() -> ForgeQueryRuntime {
    todo!()
}
