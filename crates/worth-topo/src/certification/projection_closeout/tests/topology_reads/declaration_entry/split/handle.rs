use super::super::super::support::{current_head_query_handle, snapshot_query_handle};
use super::support::{shell_split_declaration, wire_split_declaration};
use crate::facade::{
    TopologyOperatorEnvelopeChecked, TopologyOperatorEnvelopeTerminalError,
    TopologyOperatorWorkflowHandleExt,
};

#[test]
fn current_head_handle_orchestrates_wire_split_declaration_across_all_query_lanes() {
    let handle = current_head_query_handle();
    let declaration = wire_split_declaration();
    let ordinary = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head wire split declaration should envelope"));
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked wire split declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof wire split declaration"),
    }
}

#[test]
fn snapshot_handle_does_not_envelope_wire_split_declaration() {
    let handle = snapshot_query_handle();

    assert!(matches!(
        handle.orchestrate_topology_operator_envelope(wire_split_declaration()),
        Err(TopologyOperatorEnvelopeTerminalError::RebindRequired(_))
    ));
    assert!(matches!(
        handle.orchestrate_topology_operator_envelope_checked(wire_split_declaration()),
        TopologyOperatorEnvelopeChecked::RebindRequired(_)
    ));
}

#[test]
fn current_head_handle_orchestrates_shell_split_declaration_across_all_query_lanes() {
    let handle = current_head_query_handle();
    let declaration = shell_split_declaration();
    let ordinary = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head shell split declaration should envelope"));
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked shell split declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof shell split declaration"),
    }
}

#[test]
fn snapshot_handle_does_not_envelope_shell_split_declaration() {
    let handle = snapshot_query_handle();

    assert!(matches!(
        handle.orchestrate_topology_operator_envelope(shell_split_declaration()),
        Err(TopologyOperatorEnvelopeTerminalError::RebindRequired(_))
    ));
    assert!(matches!(
        handle.orchestrate_topology_operator_envelope_checked(shell_split_declaration()),
        TopologyOperatorEnvelopeChecked::RebindRequired(_)
    ));
}
