use worth_query::facade::runtime::{
    WorthQueryGraphObligationDispatchContext, WorthQueryGraphObligationDispatchEnvelope,
    WorthQueryGraphObligationDispatchPlan, WorthQueryGraphObligationVerdict,
};

fn main() {
    let context = WorthQueryGraphObligationDispatchContext::graph_composition(
        "touch.digest",
        "world.digest",
    )
    .expect("context");
    let row = WorthQueryGraphObligationDispatchPlan::blocking_invariant("topology.loop-wiring")
        .verdict(WorthQueryGraphObligationVerdict::allow())
        .expect("row");

    let _ = WorthQueryGraphObligationDispatchEnvelope {
        context,
        rows: vec![row],
    };
}
