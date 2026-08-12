use worth_query_execution::facade::primary_graph::WorthQueryRecoveryHandle;

fn cannot_clone(handle: WorthQueryRecoveryHandle) {
    let _copied = handle.clone();
}

fn main() {}
