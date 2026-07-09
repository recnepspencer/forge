use worth_query::facade::runtime::{
    WorthQueryGraphObligationDispatchPlan, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphObligationVerdict,
};

fn main() {
    let _ = WorthQueryGraphObligationDispatchPlan {
        kind: WorthQueryGraphObligationKind::BlockingInvariant,
        rule_identity: WorthQueryGraphObligationRuleIdentity::new(
            "topology",
            "loop-wiring",
            "v1",
        )
        .expect("rule"),
        verdict: WorthQueryGraphObligationVerdict::allow(),
    };
}
