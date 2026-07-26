use worth_query_execution::facade::domain_computation::WorthQueryCheckpointExportHandoff;

fn resume(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.resume();
}

fn restore(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.restore();
}

fn main() {}
