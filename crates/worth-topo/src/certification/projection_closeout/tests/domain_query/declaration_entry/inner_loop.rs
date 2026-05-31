use super::super::support::{current_head_query_handle, snapshot_query_handle};
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::facade::{
    topology_runtime, TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyRuntimeAdapters,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;
use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
};
use schema::facade::topology_authoring::seed_minimal_topology;

#[test]
fn current_head_handle_orchestrates_create_inner_loop_on_existing_face_declaration_across_all_query_lanes(
) {
    let handle = current_head_query_handle();
    let declaration = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        "query-native.handle-entry.inner-loop",
        "query-native.handle-entry.inner-loop.face-membership",
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            1,
            1,
        ),
    );
    let ordinary = handle
        .orchestrate_declaration_entry(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head grouped inner-loop declaration should envelope"));
    let checked = handle.orchestrate_declaration_entry_checked(declaration.clone());
    let proof = handle.orchestrate_declaration_entry_proof(declaration);

    match checked {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked grouped inner-loop declaration"),
    }
    match proof.outcome() {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof grouped inner-loop declaration"),
    }
}

#[test]
fn snapshot_handle_does_not_envelope_create_inner_loop_on_existing_face_declaration() {
    let handle = snapshot_query_handle();

    let ordinary = handle.orchestrate_declaration_entry(
        TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
            "query-native.snapshot.inner-loop",
            "query-native.snapshot.inner-loop.face-membership",
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                1,
                1,
            ),
        ),
    );
    let checked = handle.orchestrate_declaration_entry_checked(
        TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
            "query-native.snapshot.inner-loop",
            "query-native.snapshot.inner-loop.face-membership",
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                1,
                1,
            ),
        ),
    );

    assert!(matches!(
        ordinary,
        Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(_))
    ));
    assert!(matches!(
        checked,
        ForgeQueryDeclarationEntryOrchestrationChecked::RebindRequired(_)
    ));
}

#[test]
fn current_head_runtime_executes_canonical_create_inner_loop_declaration_through_declaration_entry()
{
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let seeded = seed_minimal_topology(&mut runtime, "query-native.inner-loop.runtime")
        .expect("seed topology");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query-native.inner-loop.runtime").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
            "query-native.inner-loop.runtime.loop",
            "query-native.inner-loop.runtime.face-membership",
            seeded.face,
        ),
    )
    .expect("canonical inner-loop declaration should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.create_inner_loop_on_existing_face"
    );
    let face = execution
        .materialized
        .topology()
        .faces
        .iter()
        .find(|face| face.entity_id == seeded.face)
        .expect("seeded face should remain materialized");
    assert_eq!(face.inner_loop_ids.len(), 1);
}
