use crate::courtroom::closeout::{
    certify_s8_layout_closeout_suite, project_s8_layout_handoff_grammar,
};
use forge_store_layout_indexes::layout_certification::{
    S9LayoutMachineState, S9LayoutMachineTransition, S9LayoutStateMachine,
    S9_DOWNSTREAM_PROTOCOL_DESTINATIONS,
};
use forge_store_layout_indexes::layout_closeout::layout_closeout;
use forge_store_physical_certification::layout_harness::runtime::LayoutRuntimeCoverageMatrix;
use forge_store_readiness::admit_s8_layout_handoff_readiness;

#[test]
fn s8_layout_closeout_suite_rejects_inventory_without_executed_runtime_matrix() {
    assert!(certify_s8_layout_closeout_suite(&LayoutRuntimeCoverageMatrix::default()).is_err());
}

#[test]
fn admitted_layout_grammar_reaches_the_live_courtroom_boundary() {
    let handoff = project_s8_layout_handoff_grammar(
        admit_s8_layout_handoff_readiness(
            layout_closeout()
                .admit_s9_layout_handoff()
                .expect("complete layout grammar"),
        )
        .expect("readiness preserves admitted grammar"),
    )
    .expect("courtroom consumes the same admitted lower witness");
    assert!(handoff
        .grammar()
        .requires(S9LayoutStateMachine::HiddenScanDenial));
    let degraded = handoff
        .machine_contract(S9LayoutStateMachine::DegradedExactScan)
        .expect("courtroom preserves the admitted machine contract");
    assert!(degraded.transitions().iter().any(|artifact| {
        let edge = artifact.edge();
        edge.from() == S9LayoutMachineState::Ready
            && edge.transition() == S9LayoutMachineTransition::AdmitExactCounters
            && edge.to() == S9LayoutMachineState::ExactCountersObserved
    }));
    assert!(degraded.transitions().iter().any(|artifact| {
        let edge = artifact.edge();
        edge.from() == S9LayoutMachineState::ExactCountersObserved
            && edge.transition() == S9LayoutMachineTransition::Execute
            && edge.to() == S9LayoutMachineState::Executed
    }));
    for target in S9_DOWNSTREAM_PROTOCOL_DESTINATIONS {
        assert!(handoff.declares_pending_protocol_target(target));
    }
}
