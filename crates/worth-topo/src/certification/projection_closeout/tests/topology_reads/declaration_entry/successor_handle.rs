use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

use super::super::support::{current_head_query_handle, snapshot_query_handle};
use crate::facade::{
    LoopSuccessorKind, TopologyLoopSuccessorRewireMember, TopologyOperatorEnvelopeChecked,
    TopologyOperatorEnvelopeTerminalError, TopologyOperatorWorkflowHandleExt,
    TopologyRewireLoopSuccessorProgramDeclaration,
};

#[test]
fn current_head_handle_orchestrates_loop_successor_program_declaration_across_all_query_lanes() {
    let handle = current_head_query_handle();
    let declaration = successor_program_declaration();
    let ordinary = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head successor declaration should envelope"));
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked successor declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof successor declaration"),
    }
}

#[test]
fn snapshot_handle_does_not_envelope_loop_successor_program_declaration() {
    let handle = snapshot_query_handle();
    let declaration = successor_program_declaration();

    let ordinary = handle.orchestrate_topology_operator_envelope(declaration.clone());
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration);

    assert!(matches!(
        ordinary,
        Err(TopologyOperatorEnvelopeTerminalError::RebindRequired(_))
    ));
    assert!(matches!(
        checked,
        TopologyOperatorEnvelopeChecked::RebindRequired(_)
    ));
}

fn successor_program_declaration() -> TopologyRewireLoopSuccessorProgramDeclaration {
    TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), 1, 1),
            LoopSuccessorKind::Next,
            EntityId::new(PartitionId::main(), 10, 1),
            EntityId::new(PartitionId::main(), 11, 1),
        ),
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), 2, 1),
            LoopSuccessorKind::Prev,
            EntityId::new(PartitionId::main(), 10, 1),
            EntityId::new(PartitionId::main(), 9, 1),
        ),
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), 3, 1),
            LoopSuccessorKind::Next,
            EntityId::new(PartitionId::main(), 8, 1),
            EntityId::new(PartitionId::main(), 7, 1),
        ),
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), 4, 1),
            LoopSuccessorKind::Prev,
            EntityId::new(PartitionId::main(), 11, 1),
            EntityId::new(PartitionId::main(), 8, 1),
        ),
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), 5, 1),
            LoopSuccessorKind::Next,
            EntityId::new(PartitionId::main(), 9, 1),
            EntityId::new(PartitionId::main(), 10, 1),
        ),
        TopologyLoopSuccessorRewireMember::new(
            RelationId::new(PartitionId::main(), 6, 1),
            LoopSuccessorKind::Prev,
            EntityId::new(PartitionId::main(), 11, 1),
            EntityId::new(PartitionId::main(), 10, 1),
        ),
    ])
}
