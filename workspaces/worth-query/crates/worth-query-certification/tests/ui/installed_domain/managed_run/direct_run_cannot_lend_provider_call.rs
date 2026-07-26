use worth_query_execution::facade::domain_computation::{
    WorthQueryGraphProviderCallKind, WorthQueryManagedGraphCallRequest, WorthQueryRunningDirectRun,
};
use worth_query_host::facade::domain::WorthQueryInstalledGraphParticipationAuthority;

fn lend_call(
    run: &mut WorthQueryRunningDirectRun,
    graph: &WorthQueryInstalledGraphParticipationAuthority,
) {
    let _ = run.bind_graph_provider_call(
        graph,
        WorthQueryManagedGraphCallRequest::new(
            WorthQueryGraphProviderCallKind::Observe,
            "forged-direct-call",
        ),
    );
}

fn main() {}
