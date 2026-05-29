use schema::facade::{
    explain_authority_trace, explain_bridge_trace, explain_derived_trace, explain_signal_trace,
    narrate_boundary_envelope, narrate_boundary_failure, narrate_decision_trace,
    AuthorityNarrative, BridgeHistoricalNarrative, BridgeNarrative, BridgeRouteNarrative,
    DerivedNarrative, NarratedTrace, NarrativeLine, SignalNarrative,
};

fn main() {
    let _ = (
        explain_authority_trace,
        explain_bridge_trace,
        explain_derived_trace,
        explain_signal_trace,
        narrate_boundary_envelope::<()>,
        narrate_boundary_failure::<()>,
        narrate_decision_trace,
        None::<AuthorityNarrative>,
        None::<BridgeHistoricalNarrative>,
        None::<BridgeNarrative>,
        None::<BridgeRouteNarrative>,
        None::<DerivedNarrative>,
        None::<NarratedTrace>,
        None::<NarrativeLine>,
        None::<SignalNarrative>,
    );
}
