use worth_query::facade::{
    WorthQueryIntentExecution, WorthQueryMutationReceipt, WorthQueryRuntime,
};

fn main() {
    let runtime = fake_runtime();
    let execution = WorthQueryIntentExecution::idempotent_noop(
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
    let _: WorthQueryMutationReceipt = execution;
}

fn fake_runtime() -> WorthQueryRuntime {
    todo!()
}
