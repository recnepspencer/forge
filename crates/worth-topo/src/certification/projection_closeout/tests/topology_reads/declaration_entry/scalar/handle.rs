use super::super::super::support::{current_head_query_handle, snapshot_query_handle};
use super::support::{
    detach_boundary_declaration, detach_radial_declaration, detach_shell_or_wire_declaration,
    retire_declaration, rewire_endpoint_declaration, splice_radial_declaration,
};
use crate::facade::{
    TopologyOperatorEnvelopeChecked, TopologyOperatorEnvelopeTerminalError,
    TopologyOperatorWorkflowHandleExt,
};

fn assert_current_head_envelopes<I>(declaration: I)
where
    I: Clone
        + forge_query::facade::ForgeQueryDeclarationInput<crate::query_domain::TopologyQueryDomain>,
{
    let handle = current_head_query_handle();
    let ordinary = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head declaration should envelope"));
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof declaration"),
    }
}

fn assert_snapshot_rebind_required<I>(declaration: I)
where
    I: Clone
        + forge_query::facade::ForgeQueryDeclarationInput<crate::query_domain::TopologyQueryDomain>,
{
    let handle = snapshot_query_handle();

    assert!(matches!(
        handle.orchestrate_topology_operator_envelope(declaration.clone()),
        Err(TopologyOperatorEnvelopeTerminalError::RebindRequired(_))
    ));
    assert!(matches!(
        handle.orchestrate_topology_operator_envelope_checked(declaration),
        TopologyOperatorEnvelopeChecked::RebindRequired(_)
    ));
}

#[test]
fn current_head_handle_orchestrates_remaining_scalar_declarations_across_all_query_lanes() {
    assert_current_head_envelopes(retire_declaration());
    assert_current_head_envelopes(detach_boundary_declaration());
    assert_current_head_envelopes(rewire_endpoint_declaration());
    assert_current_head_envelopes(detach_shell_or_wire_declaration());
    assert_current_head_envelopes(splice_radial_declaration());
    assert_current_head_envelopes(detach_radial_declaration());
}

#[test]
fn snapshot_handle_rebinds_remaining_scalar_declarations() {
    assert_snapshot_rebind_required(retire_declaration());
    assert_snapshot_rebind_required(detach_boundary_declaration());
    assert_snapshot_rebind_required(rewire_endpoint_declaration());
    assert_snapshot_rebind_required(detach_shell_or_wire_declaration());
    assert_snapshot_rebind_required(splice_radial_declaration());
    assert_snapshot_rebind_required(detach_radial_declaration());
}
