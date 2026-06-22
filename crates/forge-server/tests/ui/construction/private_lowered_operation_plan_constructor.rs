use forge_server::ForgeServerLoweredOperationPlan;

fn main() {
    let _ = ForgeServerLoweredOperationPlan {
        query_handoff: loop {},
        strategy: loop {},
        evidence_policy: loop {},
        counters: loop {},
        receipt: loop {},
        canonical_digest: "forged".to_string(),
    };
}
