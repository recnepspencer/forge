use forge_query::facade::runtime::{
    ForgeQueryEphemeralGraphIndexScope, ForgeQueryEphemeralGraphIndexScopeKind,
};

fn main() {
    let _ = ForgeQueryEphemeralGraphIndexScope {
        digest: "scope".to_string(),
        kind: ForgeQueryEphemeralGraphIndexScopeKind::ReadExecution,
        admitted_plan_digest: "plan".to_string(),
        snapshot_identity_digest: "snapshot".to_string(),
        byte_budget: 1,
    };
}
