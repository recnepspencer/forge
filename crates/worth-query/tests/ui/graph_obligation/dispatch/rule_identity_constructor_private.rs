use worth_query::facade::runtime::WorthQueryGraphObligationRuleIdentity;

fn main() {
    let _ = WorthQueryGraphObligationRuleIdentity {
        namespace: "topology".to_string(),
        name: "loop-wiring".to_string(),
        semantic_version: "v1".to_string(),
        domain_invariant_family: "topology:loop-wiring:v1".to_string(),
    };
}
