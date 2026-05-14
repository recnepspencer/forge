use forge_query::facade::{
    BasisAuthorityPosture, BasisFamily, BasisLifecyclePosture, BasisScopePosture,
    BasisVisibilityPosture, NormalizedBasisIntent,
};

fn main() {
    let _ = NormalizedBasisIntent {
        family: BasisFamily::CurrentHead,
        authority: BasisAuthorityPosture::Runtime,
        scope: BasisScopePosture::Global,
        visibility: BasisVisibilityPosture::Full,
        lifecycle: BasisLifecyclePosture::Current,
        operation_lane: String::new(),
        policy_digest: None,
        tenant_schema_digest: None,
        lower_runtime_binding_digest: None,
        source_path: String::new(),
        normalized_digest: String::new(),
        counters: Default::default(),
    };
}
