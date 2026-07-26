use worth_query_execution::facade::provider_session::WorthQueryDirectResourceReadmissionPending;
use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending;

fn advance_bridge(pending: BridgeExecutionBasisReadmissionPending) {
    pending.commit();
}

fn advance_query(pending: WorthQueryDirectResourceReadmissionPending) {
    pending.advance();
}

fn main() {}
