use worth_query_execution::facade::domain_computation::WorthQueryCheckpointExportHandoff;

fn resume(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.resume();
}

fn restore(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.restore();
}

fn advance(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.advance();
}

fn publish(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.publish();
}

fn reconstruct_yielded(handoff: WorthQueryCheckpointExportHandoff) {
    handoff.into_yielded();
}

fn main() {}
