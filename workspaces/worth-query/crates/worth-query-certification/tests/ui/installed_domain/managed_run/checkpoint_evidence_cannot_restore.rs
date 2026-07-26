use worth_query_execution::facade::domain_computation::WorthQueryProviderCheckpointEvidence;

fn restore(evidence: WorthQueryProviderCheckpointEvidence) {
    evidence.restore();
}

fn main() {}
