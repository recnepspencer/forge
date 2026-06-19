use forge_query::facade::runtime::{
    ForgeQueryGraphObligationDispatchPlan, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationVerdict,
};

fn main() {
    let _ = ForgeQueryGraphObligationDispatchPlan {
        kind: ForgeQueryGraphObligationKind::BlockingInvariant,
        rule_identity: ForgeQueryGraphObligationRuleIdentity::new(
            "topology",
            "loop-wiring",
            "v1",
        )
        .expect("rule"),
        verdict: ForgeQueryGraphObligationVerdict::allow(),
    };
}
