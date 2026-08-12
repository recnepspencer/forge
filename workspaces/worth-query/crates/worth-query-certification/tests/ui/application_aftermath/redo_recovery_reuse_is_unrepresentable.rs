use worth_query_execution::facade::primary_graph::{
    WorthQueryPrimaryGraphApplicationRuntime, WorthQueryRecoveryEffectAuthority,
};
use worth_query_execution::facade::provisional_aftermath::{
    WorthQueryRedoIntent, WorthQueryRedoRecovery,
};
use worth_query_host::facade::domain::ApplicationSchema;

fn cannot_admit_redo_twice<Schema: ApplicationSchema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    recovery: WorthQueryRedoRecovery,
    authority: &WorthQueryRecoveryEffectAuthority,
    intent: &WorthQueryRedoIntent,
) {
    let _first = runtime.admit_redo(recovery, authority, intent);
    let _second = runtime.admit_redo(recovery, authority, intent);
}

fn main() {}
