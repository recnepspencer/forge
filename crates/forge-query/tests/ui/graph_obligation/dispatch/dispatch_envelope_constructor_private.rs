use forge_query::facade::runtime::{
    ForgeQueryGraphObligationDispatchContext, ForgeQueryGraphObligationDispatchEnvelope,
    ForgeQueryGraphObligationDispatchPlan, ForgeQueryGraphObligationVerdict,
};

fn main() {
    let context = ForgeQueryGraphObligationDispatchContext::graph_composition(
        "touch.digest",
        "world.digest",
    )
    .expect("context");
    let row = ForgeQueryGraphObligationDispatchPlan::blocking_invariant("topology.loop-wiring")
        .verdict(ForgeQueryGraphObligationVerdict::allow())
        .expect("row");

    let _ = ForgeQueryGraphObligationDispatchEnvelope {
        context,
        rows: vec![row],
    };
}
