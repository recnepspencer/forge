use worth_query::facade::runtime::{WorthQueryRuntimeBackendPosture, WorthQueryRuntimePublicApiContract};

fn main() {
    let _worthd = WorthQueryRuntimePublicApiContract {
        backend_posture: WorthQueryRuntimeBackendPosture::Primary,
        families: Vec::new(),
        stable_family_count: 0,
        deferred_family_count: 0,
        unsupported_family_count: 0,
        contract_digest: String::new(),
    };
}
