use worth_query_execution::facade::provider_session::WorthQueryLoweredProvisionalEffectProgram;

fn main() {
    let _forged = WorthQueryLoweredProvisionalEffectProgram {
        identity: "forged".into(),
        binding_identity: "forged-binding".into(),
        steps: Vec::new().into(),
    };
}
