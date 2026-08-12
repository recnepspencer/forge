use worth_query_execution::facade::primary_graph::{
    WorthQueryPrimaryGraphApplicationRuntime, WorthQueryRecoveryEffectAuthority,
    WorthQueryRecoveryHandle,
};
use worth_query_host::facade::domain::ApplicationSchema;

fn cannot_admit_undo_twice<Schema: ApplicationSchema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    handle: WorthQueryRecoveryHandle,
    authority: &WorthQueryRecoveryEffectAuthority,
) {
    let _first = runtime.admit_undo(handle, authority);
    let _second = runtime.admit_undo(handle, authority);
}

fn main() {}
