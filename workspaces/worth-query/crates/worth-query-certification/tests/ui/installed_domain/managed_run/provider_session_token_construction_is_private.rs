use worth_query_execution::facade::provider_session::WorthQueryProviderSessionToken;

fn main() {
    let _forged = WorthQueryProviderSessionToken {
        identity: "forged".into(),
        plan_identity: "forged-plan".into(),
        provider_identity: "forged-provider".into(),
        provider_generation: 1,
        generation: 1,
        physical_session_identity: "forged-physical-session".into(),
    };
}
