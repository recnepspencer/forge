use crate::ordinary_outcome::{
    ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use super::support::{admitted_handle, orchestration_input, progressed_input};

#[test]
fn proof_stops_at_compatibility_when_no_bridge_request_is_supplied() {
    let handle = admitted_handle("main");
    let proof =
        handle.orchestrate_signal_compatibility_proof(orchestration_input(&handle, "face-a"));

    assert_eq!(
        proof.request().request_kind(),
        "signal_compatibility_orchestration"
    );
    assert_eq!(proof.witness_checks().len(), 1);
    assert!(proof.witness_checks()[0].did_pass());
    assert_eq!(proof.narrowing_decisions().len(), 1);
    assert!(proof.linked_artifacts().declaration_digest().is_some());
    assert!(proof.linked_artifacts().route_plan_digest().is_some());
    assert!(proof.linked_artifacts().receipt_digest().is_some());
    assert!(proof.linked_artifacts().envelope_digest().is_some());
}

#[test]
fn progressed_input_proof_carries_route_receipt_and_envelope_digests() {
    let handle = admitted_handle("main");
    let proof = handle.orchestrate_signal_compatibility_proof(progressed_input(&handle, "face-a"));
    let linked = proof.linked_artifacts();

    assert!(linked.declaration_digest().is_some());
    assert!(linked.progression_digest().is_some());
    assert!(linked.route_plan_digest().is_some());
    assert!(linked.receipt_digest().is_some());
    assert!(linked.envelope_digest().is_some());
}

#[test]
fn wrong_world_is_preserved_through_signal_orchestration() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    let outcome = right.orchestrate_signal_compatibility(orchestration_input(&left, "face-a"));
    assert!(matches!(
        outcome,
        crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_)
    ));
}

#[test]
fn ordinary_outcome_keeps_signal_checked_topology_visible() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    match right.orchestrate_signal_compatibility_outcome(orchestration_input(&left, "face-a")) {
        ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(
                posture
                    .checked_topology()
                    .signal_compatibility_orchestration_kind(),
                Some(
                    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongWorld
                )
            );
            assert!(posture
                .checked_topology()
                .signal_compatibility_orchestration_linked_artifacts()
                .is_some());
        }
        _ => panic!("expected wrong-world ordinary outcome"),
    }
}
