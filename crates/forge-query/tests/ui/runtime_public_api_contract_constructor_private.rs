use forge_query::facade::{
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimePublicApiContract,
};

fn main() {
    let _forged = ForgeQueryRuntimePublicApiContract {
        backend_posture: ForgeQueryRuntimeBackendPosture::Primary,
        families: Vec::new(),
        stable_family_count: 0,
        deferred_family_count: 0,
        unsupported_family_count: 0,
        contract_digest: String::new(),
    };
}
