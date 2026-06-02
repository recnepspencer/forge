use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

use crate::certification::support::declaration_runtime::current_head_unsupported_declaration_families;
use crate::facade::{
    topology_runtime, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRuntimeAdapters,
    TopologyShellRehomeFaceMember, TopologyWireRehomeHalfEdgeMember,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_rejects_partial_wire_rehome_declaration_before_any_declaration_entry_execution(
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native.partial-rehome-wire.runtime",
        &MilestoneOnePrimitiveCase::WireOpen { half_edge_count: 4 },
    )
    .expect("seed topology");
    let (wire_id, half_edge_ids) = seeded_wire_and_half_edges(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        "query-native.partial-rehome-wire.runtime.workspace",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        CreateKey::new("query-native.partial-rehome-wire.runtime.new-wire").as_str(),
        wire_id,
        vec![TopologyWireRehomeHalfEdgeMember::new(
            "query-native.partial-rehome-wire.runtime.member-1",
            half_edge_ids[0],
        )],
    );
    assert!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration)
            .contains(&crate::facade::TopologyMutationFamily::AttachShellOrWireMembership)
    );
}

#[test]
fn current_head_runtime_rejects_partial_shell_rehome_declaration_before_any_declaration_entry_execution(
) {
    let mut runtime = build_milestone_one_runtime().expect("runtime");
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "query-native.partial-rehome-shell.runtime",
        &MilestoneOnePrimitiveCase::SheetPatch { face_count: 2 },
    )
    .expect("seed topology");
    let (region_id, shell_id, face_ids) =
        seeded_patch_region_shell_and_faces(&runtime, &verified.read_basis());
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        "query-native.partial-rehome-shell.runtime.workspace",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let declaration = TopologyRehomeAllOwnedFacesToNewShellDeclaration::new(
        CreateKey::new("query-native.partial-rehome-shell.runtime.new-shell").as_str(),
        "query-native.partial-rehome-shell.runtime.region-member",
        region_id,
        shell_id,
        vec![TopologyShellRehomeFaceMember::new(
            "query-native.partial-rehome-shell.runtime.face-1",
            face_ids[0],
        )],
    );
    assert!(
        current_head_unsupported_declaration_families(&mut workspace, &surfaces, &declaration)
            .contains(&crate::facade::TopologyMutationFamily::AttachShellOrWireMembership)
    );
}

fn seeded_wire_and_half_edges(
    runtime: &forge_relational::facade::runtime::RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> (
    forge_relational::facade::identity::EntityId,
    Vec<forge_relational::facade::identity::EntityId>,
) {
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
) -> (
    forge_relational::facade::identity::EntityId,
    forge_relational::facade::identity::EntityId,
    Vec<forge_relational::facade::identity::EntityId>,
) {
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
