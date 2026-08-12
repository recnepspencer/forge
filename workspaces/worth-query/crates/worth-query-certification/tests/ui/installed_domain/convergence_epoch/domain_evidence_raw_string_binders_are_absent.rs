use worth_query_host::facade::installed::provider_session::WorthQueryExecutionProviderSession;

fn bind_raw(session: &WorthQueryExecutionProviderSession) {
    let _ = session.bind_direct_ordinary_domain_evidence("caller-snapshot", "caller-occurrence");
    let _ = session.bind_workflow_stage_ordinary_domain_evidence(
        "caller-run",
        "caller-stage",
        "caller-snapshot",
        "caller-occurrence",
    );
}

fn main() {}
