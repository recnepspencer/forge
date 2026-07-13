use worth_query::facade::runtime::{WorthQueryEphemeralGraphIndexScope, WorthQueryEphemeralGraphIndexScopeKind};

fn main() {
    let _ = WorthQueryEphemeralGraphIndexScope {
        digest: "scope".to_string(),
        kind: WorthQueryEphemeralGraphIndexScopeKind::ReadExecution,
        admitted_plan_digest: "plan".to_string(),
        snapshot_identity_digest: "snapshot".to_string(),
        byte_budget: 1,
    };
}
