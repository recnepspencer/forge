use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationChecked, ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationTerminalError, ForgeQueryEntity,
};
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::support::{current_head_query_handle, snapshot_query_handle};
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::facade::{topology_runtime, TopologyDeclaredQuerySurfaces, TopologyRuntimeAdapters};
use crate::projection::{query_entity_id_from_row, query_relation_id_from_row};
use crate::topology_operators::{
    TopologyRadialSpliceMember, TopologySpliceRadialAdjacencyProgramDeclaration,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_handle_orchestrates_radial_splice_program_declaration_across_all_query_lanes() {
    let declaration = radial_program_declaration();
    let handle = current_head_query_handle();
    let ordinary = handle
        .orchestrate_declaration_entry(declaration.clone())
        .unwrap_or_else(|_| panic!("current-head radial splice program should envelope"));
    let checked = handle.orchestrate_declaration_entry_checked(declaration.clone());
    let proof = handle.orchestrate_declaration_entry_proof(declaration);

    match checked {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped checked radial splice program"),
    }
    match proof.outcome() {
        ForgeQueryDeclarationEntryOrchestrationChecked::Enveloped(envelope) => {
            assert_eq!(ordinary.envelope_digest(), envelope.envelope_digest());
        }
        _ => panic!("expected enveloped proof radial splice program"),
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
fn snapshot_handle_does_not_envelope_radial_splice_program_declaration() {
    let handle = snapshot_query_handle();
    let ordinary = handle.orchestrate_declaration_entry(radial_program_declaration());

    assert!(matches!(
        ordinary,
        Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(_))
    ));
}

#[test]
fn current_head_runtime_executes_canonical_radial_splice_program_through_declaration_entry() {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query-native.grouped.radial-splice-program",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, "query-native.grouped.radial-splice-program")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = canonical_radial_program_declaration(&mut workspace, &surfaces);
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("radial splice program should execute through declaration entry");

    assert_eq!(
        execution.semantic_family_key(),
        "topology.splice_radial_adjacency_program"
    );
}

fn radial_program_declaration() -> TopologySpliceRadialAdjacencyProgramDeclaration {
    TopologySpliceRadialAdjacencyProgramDeclaration::new(vec![
        TopologyRadialSpliceMember::new(
            forge_relational::facade::identity::RelationId::new(
                forge_relational::facade::identity::PartitionId::main(),
                7,
                1,
            ),
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                8,
                1,
            ),
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                9,
                1,
            ),
        ),
        TopologyRadialSpliceMember::new(
            forge_relational::facade::identity::RelationId::new(
                forge_relational::facade::identity::PartitionId::main(),
                10,
                1,
            ),
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                9,
                1,
            ),
            forge_relational::facade::identity::EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                8,
                1,
            ),
        ),
    ])
}

fn canonical_radial_program_declaration(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    surfaces: &TopologyDeclaredQuerySurfaces,
) -> TopologySpliceRadialAdjacencyProgramDeclaration {
    let entity_rows = workspace.read::<serde_json::Value>(surfaces.entities());
    let relation_rows = workspace.read::<serde_json::Value>(surfaces.relations());
    let source_identity = first_source_identity_for_relation_kind(
        &relation_rows,
        TopologyRelationKind::HalfEdgeRadialNext,
    );
    let cycle = radial_cycle_identities(&relation_rows, &source_identity);
    let reordered_cycle = reorder_radial_cycle(&cycle);
    radial_cycle_reorder_declaration(&entity_rows, &relation_rows, &cycle, &reordered_cycle)
}

fn radial_cycle_reorder_declaration(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    current_cycle: &[String],
    reordered_cycle: &[String],
) -> TopologySpliceRadialAdjacencyProgramDeclaration {
    TopologySpliceRadialAdjacencyProgramDeclaration::new(
        reordered_cycle
            .iter()
            .enumerate()
            .filter_map(|(index, source_identity)| {
                let current_target = next_identity(current_cycle, source_identity)?;
                let reordered_target =
                    reordered_cycle[(index + 1) % reordered_cycle.len()].as_str();
                (current_target != reordered_target).then_some((source_identity, reordered_target))
            })
            .map(|(source_identity, reordered_target)| {
                TopologyRadialSpliceMember::new(
                    relation_id_for_source_kind(
                        relation_rows,
                        source_identity,
                        TopologyRelationKind::HalfEdgeRadialNext,
                    ),
                    entity_id_for_identity(entity_rows, source_identity),
                    entity_id_for_identity(entity_rows, reordered_target),
                )
            })
            .collect(),
    )
}

fn relation_id_for_source_kind(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> forge_relational::facade::identity::RelationId {
    let row = relation_rows
        .iter()
        .find(|row| row_matches_source_kind(row, source_identity, relation_kind))
        .expect("radial splice relation should resolve");
    query_relation_id_from_row(row).expect("relation id should decode")
}

fn first_source_identity_for_relation_kind(
    relation_rows: &[ForgeQueryEntity],
    relation_kind: TopologyRelationKind,
) -> String {
    relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                == Some(relation_kind.kind_name())
        })
        .and_then(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("source_identity"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .expect("radial relation source should resolve")
}

fn radial_cycle_identities(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
) -> Vec<String> {
    let mut cycle = vec![source_identity.to_string()];
    loop {
        let next = relation_target_identity_for_source_kind(
            relation_rows,
            cycle.last().expect("cycle should remain non-empty"),
            TopologyRelationKind::HalfEdgeRadialNext,
        );
        if next == source_identity {
            return cycle;
        }
        assert!(
            !cycle.iter().any(|identity| identity == &next),
            "radial cycle should not revisit a half-edge before closing"
        );
        cycle.push(next);
    }
}

fn relation_target_identity_for_source_kind(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> String {
    relation_rows
        .iter()
        .find(|row| row_matches_source_kind(row, source_identity, relation_kind))
        .and_then(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("target_identity"))
                .and_then(|value| value.as_str())
                .map(str::to_string)
        })
        .expect("radial splice target identity should resolve")
}

fn reorder_radial_cycle(cycle: &[String]) -> Vec<String> {
    assert!(
        cycle.len() >= 3,
        "radial cycle must have at least three half-edges"
    );
    let mut reordered_cycle = cycle.to_vec();
    reordered_cycle.swap(1, 2);
    reordered_cycle
}

fn next_identity<'a>(cycle: &'a [String], source_identity: &str) -> Option<&'a str> {
    cycle
        .iter()
        .position(|identity| identity == source_identity)
        .map(|index| cycle[(index + 1) % cycle.len()].as_str())
}

fn row_matches_source_kind(
    row: &ForgeQueryEntity,
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> bool {
    row.payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
        == Some(relation_kind.kind_name())
        && row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(|value| value.as_str())
            == Some(source_identity)
}

fn entity_id_for_identity(
    entity_rows: &[ForgeQueryEntity],
    identity: &str,
) -> forge_relational::facade::identity::EntityId {
    let row = entity_rows
        .iter()
        .find(|row| row.identity == identity)
        .expect("entity identity should resolve");
    query_entity_id_from_row(row).expect("entity id should decode")
}
