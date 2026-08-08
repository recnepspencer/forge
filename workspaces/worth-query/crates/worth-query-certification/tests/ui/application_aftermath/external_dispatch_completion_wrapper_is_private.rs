use worth_query_execution::facade::domain_computation::{
    ExternalEffectPosture, WorthQueryExternalDispatchPosture,
};

fn forge_completion(attempt: ExternalEffectPosture) {
    let _ = WorthQueryExternalDispatchPosture::completed(attempt);
}

fn main() {}
