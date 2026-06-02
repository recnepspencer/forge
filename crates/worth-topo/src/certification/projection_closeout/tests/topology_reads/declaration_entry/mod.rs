mod inner_loop;
mod precedent_guard;
mod radial_program;
mod rehome;
mod rehome_denials;
mod scalar;
mod split;
mod successor_handle;
mod successor_runtime;
mod successor_runtime_support;

use schema::facade::platform::entities::TopologyEntityKind;

use super::support::{current_head_query_handle, snapshot_query_handle};
use crate::certification::support::declaration_runtime::{
    current_head_unsupported_declaration_families, execute_current_head_topology_declaration,
};
use crate::facade::{
    topology_runtime, BoundaryMembershipKind, ShellOrWireMembershipKind,
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachShellOrWireMembershipDeclaration,
    TopologyCreateTopologyEntityDeclaration, TopologyOperatorEnvelopeChecked,
    TopologyOperatorEnvelopeTerminalError, TopologyOperatorWorkflowHandleExt,
    TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_minimal_topology_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::ForgeQueryDeclarationEntryOrchestrationStage;

#[test]
fn current_head_handle_orchestrates_create_topology_entity_declaration_across_all_query_lanes() {
    let handle = current_head_query_handle();
    let ordinary = handle
        .orchestrate_topology_operator_envelope(TopologyCreateTopologyEntityDeclaration::new(
            "query-native.handle-entry.vertex",
            TopologyEntityKind::Vertex,
        ))
        .unwrap_or_else(|_| panic!("current-head create declaration should envelope"));
    let checked = handle.orchestrate_topology_operator_envelope_checked(
        TopologyCreateTopologyEntityDeclaration::new(
            "query-native.handle-entry.vertex",
            TopologyEntityKind::Vertex,
        ),
    );
    let proof = handle.orchestrate_topology_operator_envelope_proof(
        TopologyCreateTopologyEntityDeclaration::new(
            "query-native.handle-entry.vertex",
            TopologyEntityKind::Vertex,
        ),
    );

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked create declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof create declaration"),
    }
    assert_eq!(
        proof
            .stage_records()
            .last()
            .expect("proof should retain stage records")
            .stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed
    );
}

#[test]
fn snapshot_handle_does_not_envelope_create_topology_entity_declaration() {
    let handle = snapshot_query_handle();

    let ordinary = handle.orchestrate_topology_operator_envelope(
        TopologyCreateTopologyEntityDeclaration::new(
            "query-native.handle-entry.snapshot.vertex",
            TopologyEntityKind::Vertex,
        ),
    );
    let checked = handle.orchestrate_topology_operator_envelope_checked(
        TopologyCreateTopologyEntityDeclaration::new(
            "query-native.handle-entry.snapshot.vertex",
            TopologyEntityKind::Vertex,
        ),
    );

    assert!(
        matches!(
            ordinary,
            Err(TopologyOperatorEnvelopeTerminalError::RebindRequired(_))
        ),
        "snapshot read-only topology handle must report rebind-required, not a generic failure, for authoritative create declarations"
    );
    assert!(
        matches!(
            checked,
            TopologyOperatorEnvelopeChecked::RebindRequired(_)
        ),
        "snapshot read-only topology handle must preserve the checked rebind-required outcome for authoritative create declarations"
    );
}

#[test]
fn current_head_runtime_executes_single_create_declaration_through_declaration_entry() {
    let runtime = build_milestone_one_runtime().expect("runtime");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.create.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyCreateTopologyEntityDeclaration::new(
            "query-native.create.runtime.vertex",
            TopologyEntityKind::Vertex,
        ),
    )
    .expect("single create declaration should execute through declaration entry");

    assert!(execution
        .materialized()
        .topology()
        .vertices
        .iter()
        .any(|vertex| vertex.label == "query-native.create.runtime.vertex"));
}

#[test]
fn current_head_handle_orchestrates_attach_boundary_membership_declaration_across_all_query_lanes()
{
    let handle = current_head_query_handle();
    let declaration = TopologyAttachBoundaryMembershipDeclaration::new(
        "query-native.handle-entry.loop-membership",
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            1,
            1,
        ),
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            2,
            1,
        ),
    );
    let ordinary = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head attach-boundary declaration should envelope"));
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked attach-boundary declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof attach-boundary declaration"),
    }
}

#[test]
fn current_head_runtime_keeps_single_attach_boundary_declaration_on_unsupported_admission_boundary()
{
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_minimal_topology_through_schema_execution(
        &mut runtime,
        "query-native.attach-boundary.runtime",
    )
    .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.attach-boundary.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyAttachBoundaryMembershipDeclaration::new(
        "query-native.attach-boundary.runtime.rewire",
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        seeded.outer_loop,
        seeded.half_edge,
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![crate::facade::TopologyMutationFamily::AttachBoundaryMembership]
    );
}

#[test]
fn current_head_handle_orchestrates_attach_shell_or_wire_membership_declaration_across_all_query_lanes(
) {
    let handle = current_head_query_handle();
    let declaration = TopologyAttachShellOrWireMembershipDeclaration::new(
        "query-native.handle-entry.wire-membership",
        ShellOrWireMembershipKind::WireOwnsHalfEdge,
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            1,
            1,
        ),
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            2,
            1,
        ),
    );
    let ordinary = handle
        .orchestrate_topology_operator_envelope(declaration.clone())
        .unwrap_or_else(|_| {
            panic!("current-head attach-shell-or-wire declaration should envelope")
        });
    let checked = handle.orchestrate_topology_operator_envelope_checked(declaration.clone());
    let proof = handle.orchestrate_topology_operator_envelope_proof(declaration);

    match checked {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked attach-shell-or-wire declaration"),
    }
    match proof.outcome() {
        TopologyOperatorEnvelopeChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof attach-shell-or-wire declaration"),
    }
}

#[test]
fn current_head_runtime_keeps_single_attach_shell_or_wire_declaration_on_unsupported_admission_boundary(
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_minimal_topology_through_schema_execution(
        &mut runtime,
        "query-native.attach-wire.runtime",
    )
    .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.attach-wire.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyAttachShellOrWireMembershipDeclaration::new(
        "query-native.attach-wire.runtime.rewire",
        ShellOrWireMembershipKind::WireOwnsHalfEdge,
        seeded.wire,
        seeded.half_edge,
    );

    assert_eq!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration),
        vec![crate::facade::TopologyMutationFamily::AttachShellOrWireMembership]
    );
}
