use worth_query_execution::facade::provider_session::WorthQueryPreparedProviderSession;

fn stage_early(prepared: &WorthQueryPreparedProviderSession<'_>) {
    let _ = prepared.effect_authority();
}

fn main() {}
