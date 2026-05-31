use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
};
use forge_relational::facade::identity::EntityId;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{
    seed_milestone_one_primitive, DerivedTopologyReadBasis, MilestoneOnePrimitiveCase,
};
use std::collections::BTreeSet;

use super::super::support::{
    current_head_query_handle, execute_current_head_topology_declaration, snapshot_query_handle,
};
use crate::facade::{
    topology_runtime, TopologyOperatorExecutionPath,
    TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRuntimeAdapters,
    TopologyShellRehomeFaceMember, TopologyWireRehomeHalfEdgeMember,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_handle_orchestrates_wire_rehome_declaration_across_all_query_lanes() {
    let handle = current_head_query_handle();
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        "query-native.rehome-wire.new-wire",
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            4,
            1,
        ),
        vec![TopologyWireRehomeHalfEdgeMember::new(
            "query-native.rehome-wire.member-1",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                8,
                1,
            ),
        )],
    );
    let ordinary = handle
        .orchestrate_declaration_entry(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head wire rehome declaration should envelope"));
    let checked = handle.orchestrate_declaration_entry_checked(declaration.clone());
    let proof = handle.orchestrate_declaration_entry_proof(declaration);

    match checked {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked wire rehome declaration"),
    }
    match proof.outcome() {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof wire rehome declaration"),
    }
}

#[test]
fn snapshot_handle_does_not_envelope_wire_rehome_declaration() {
    let handle = snapshot_query_handle();
    let ordinary = handle.orchestrate_declaration_entry(
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
            "query-native.snapshot.rehome-wire",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                4,
                1,
            ),
            vec![TopologyWireRehomeHalfEdgeMember::new(
                "query-native.snapshot.rehome-wire.member-1",
                EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    8,
                    1,
                ),
            )],
        ),
    );
    let checked = handle.orchestrate_declaration_entry_checked(
        TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
            "query-native.snapshot.rehome-wire",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                4,
                1,
            ),
            vec![TopologyWireRehomeHalfEdgeMember::new(
                "query-native.snapshot.rehome-wire.member-1",
                EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    8,
                    1,
                ),
            )],
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
fn current_head_runtime_executes_canonical_wire_rehome_batch_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.rehome-wire.runtime",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let (wire_id, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "query-native.rehome-wire.runtime.workspace")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        "query-native.rehome-wire.runtime.new-wire",
        wire_id,
        half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireRehomeHalfEdgeMember::new(
                    format!("query-native.rehome-wire.runtime.member-{}", index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("canonical wire rehome batch should execute through declaration entry");

    assert_eq!(
        execution.path,
        TopologyOperatorExecutionPath::DeclarationEntry {
            semantic_family_key: "topology.rehome_all_owned_half_edges_to_new_wire",
        }
    );
    let topology = execution.materialized.topology();
    assert!(!topology.wires.iter().any(|wire| wire.entity_id == wire_id));
    let new_wire = topology
        .wires
        .iter()
        .find(|wire| wire.label == "query-native.rehome-wire.runtime.new-wire")
        .expect("wire rehome should materialize the replacement wire");
    assert_eq!(
        new_wire
            .half_edge_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        half_edge_ids.iter().copied().collect::<BTreeSet<_>>()
    );
}

#[test]
fn current_head_handle_orchestrates_shell_rehome_declaration_across_all_query_lanes() {
    let handle = current_head_query_handle();
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        "query-native.rehome-shell.new-shell",
        "query-native.rehome-shell.region-member",
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            2,
            1,
        ),
        EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            5,
            1,
        ),
        vec![TopologyShellRehomeFaceMember::new(
            "query-native.rehome-shell.face-1",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                8,
                1,
            ),
        )],
    );
    let ordinary = handle
        .orchestrate_declaration_entry(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head shell rehome declaration should envelope"));
    let checked = handle.orchestrate_declaration_entry_checked(declaration.clone());
    let proof = handle.orchestrate_declaration_entry_proof(declaration);

    match checked {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked shell rehome declaration"),
    }
    match proof.outcome() {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof shell rehome declaration"),
    }
}

#[test]
fn snapshot_handle_does_not_envelope_shell_rehome_declaration() {
    let handle = snapshot_query_handle();
    let ordinary = handle.orchestrate_declaration_entry(
        TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
            "query-native.snapshot.rehome-shell",
            "query-native.snapshot.rehome-shell.region-member",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                2,
                1,
            ),
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                5,
                1,
            ),
            vec![TopologyShellRehomeFaceMember::new(
                "query-native.snapshot.rehome-shell.face-1",
                EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    8,
                    1,
                ),
            )],
        ),
    );
    let checked = handle.orchestrate_declaration_entry_checked(
        TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
            "query-native.snapshot.rehome-shell",
            "query-native.snapshot.rehome-shell.region-member",
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                2,
                1,
            ),
            EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                5,
                1,
            ),
            vec![TopologyShellRehomeFaceMember::new(
                "query-native.snapshot.rehome-shell.face-1",
                EntityId::new(
                    forge_relational::facade::identity::PartitionId::main(),
                    8,
                    1,
                ),
            )],
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
fn current_head_runtime_executes_canonical_shell_rehome_batch_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "query-native.rehome-shell.runtime",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region_id, shell_id, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "query-native.rehome-shell.runtime.workspace")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        "query-native.rehome-shell.runtime.new-shell",
        "query-native.rehome-shell.runtime.region-member",
        region_id,
        shell_id,
        face_ids
            .iter()
            .enumerate()
            .map(|(index, face_id)| {
                TopologyShellRehomeFaceMember::new(
                    format!("query-native.rehome-shell.runtime.face-{}", index + 1),
                    *face_id,
                )
            })
            .collect(),
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("canonical shell rehome batch should execute through declaration entry");

    assert_eq!(
        execution.path,
        TopologyOperatorExecutionPath::DeclarationEntry {
            semantic_family_key: "topology.rehome_all_owned_faces_to_new_shell",
        }
    );
    let topology = execution.materialized.topology();
    assert!(!topology
        .shells
        .iter()
        .any(|shell| shell.entity_id == shell_id));
    let new_shell = topology
        .shells
        .iter()
        .find(|shell| shell.label == "query-native.rehome-shell.runtime.new-shell")
        .expect("shell rehome should materialize the replacement shell");
    assert_eq!(new_shell.region_id, Some(region_id));
    assert_eq!(
        new_shell.face_ids.iter().copied().collect::<BTreeSet<_>>(),
        face_ids.iter().copied().collect::<BTreeSet<_>>()
    );
}

fn seeded_wire_and_half_edges(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> (EntityId, Vec<EntityId>) {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let wire = read_view
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .expect("seeded wire primitive should contain a wire");
    let half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    (wire, half_edge_ids)
}

fn seeded_patch_region_shell_and_faces(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> (EntityId, EntityId, Vec<EntityId>) {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .expect("seeded snapshot should remain readable");
    let mut region = None;
    let mut shell = None;
    let mut face_ids = Vec::new();
    for record in read_view.entities() {
        match EntityKind::from_kind_id(record.kind.kind_id) {
            Some(EntityKind::Topology(TopologyEntityKind::Region)) => {
                region = Some(record.entity_id)
            }
            Some(EntityKind::Topology(TopologyEntityKind::Shell)) => shell = Some(record.entity_id),
            Some(EntityKind::Topology(TopologyEntityKind::Face)) => face_ids.push(record.entity_id),
            _ => {}
        }
    }
    face_ids.sort();
    (
        region.expect("seeded patch should contain one region"),
        shell.expect("seeded patch should contain one shell"),
        face_ids,
    )
}
