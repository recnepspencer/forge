use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::TransactionCommitError;

use crate::data::authority::{MutationOrigin, RawTopologyIntent, TopologyAuthority};
use crate::data::entities::{EntityKind, NamingEntityKind, TopologyEntityKind};
use crate::data::relations::{RelationKind, TopologyRelationKind};
use crate::data::seed::labels::MinimalTopologyLabels;
use crate::data::seed::lookup::find_seeded_entity;
use crate::data::seed::types::MinimalTopologySeed;
use crate::data::seed::{created_ref, TopologyCreateBatchBuilder};

pub fn seed_minimal_topology(
    runtime: &mut RelationalRuntime,
    stem: &str,
) -> Result<MinimalTopologySeed, TransactionCommitError> {
    let labels = MinimalTopologyLabels::new(stem);
    let verified = TopologyAuthority::new(runtime)
        .apply_topology_intent_traced(build_minimal_topology_intent(&labels))
        .map(|traced| traced.into_primary_result())
        .map_err(|error| match error.into_error() {
            crate::data::authority::TopologyAuthorityError::Commit(error) => error,
            other => panic!(" minimal topology seed should author successfully: {other:?}"),
        })?;
    let naming_read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect(" seeded snapshot should remain readable");

    let ids = MinimalTopologySeed {
        snapshot: verified.persisted_truth.snapshot.clone(),
        model: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Model),
            &labels.model,
        ),
        body: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Body),
            &labels.body,
        ),
        lump: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Lump),
            &labels.lump,
        ),
        region: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Region),
            &labels.region,
        ),
        shell: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Shell),
            &labels.shell,
        ),
        face: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Face),
            &labels.face,
        ),
        outer_loop: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Loop),
            &labels.outer_loop,
        ),
        wire: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Wire),
            &labels.wire,
        ),
        half_edge: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
            &labels.half_edge,
        ),
        edge: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Edge),
            &labels.edge,
        ),
        vertex: find_seeded_entity(
            &naming_read,
            EntityKind::Topology(TopologyEntityKind::Vertex),
            &labels.vertex,
        ),
        persistent_name_ids: collect_persistent_name_ids(&naming_read, &labels),
        persisted_truth: verified.persisted_truth.clone(),
        read_basis: verified.read_basis.clone(),
        read_artifact: crate::data::authority::TopologyReadArtifact::from_read_basis(
            &verified.read_basis,
        ),
        certified_interpretation:
            crate::data::authority::CertifiedTopologyInterpretation::from_read_basis(
                verified.read_basis.clone(),
            ),
    };

    Ok(MinimalTopologySeed { ..ids })
}

fn build_minimal_topology_intent(labels: &MinimalTopologyLabels) -> RawTopologyIntent {
    let builder = TopologyCreateBatchBuilder::new()
        .topology_entity(
            labels.model.clone(),
            EntityKind::Topology(TopologyEntityKind::Model),
        )
        .topology_entity(
            labels.body.clone(),
            EntityKind::Topology(TopologyEntityKind::Body),
        )
        .topology_entity(
            labels.lump.clone(),
            EntityKind::Topology(TopologyEntityKind::Lump),
        )
        .topology_entity(
            labels.region.clone(),
            EntityKind::Topology(TopologyEntityKind::Region),
        )
        .topology_entity(
            labels.shell.clone(),
            EntityKind::Topology(TopologyEntityKind::Shell),
        )
        .topology_entity(
            labels.face.clone(),
            EntityKind::Topology(TopologyEntityKind::Face),
        )
        .topology_entity(
            labels.outer_loop.clone(),
            EntityKind::Topology(TopologyEntityKind::Loop),
        )
        .topology_entity(
            labels.wire.clone(),
            EntityKind::Topology(TopologyEntityKind::Wire),
        )
        .topology_entity(
            labels.half_edge.clone(),
            EntityKind::Topology(TopologyEntityKind::HalfEdge),
        )
        .topology_entity(
            labels.edge.clone(),
            EntityKind::Topology(TopologyEntityKind::Edge),
        )
        .topology_entity(
            labels.vertex.clone(),
            EntityKind::Topology(TopologyEntityKind::Vertex),
        )
        .relation(
            format!("{}.owns_body", labels.model),
            RelationKind::Topology(TopologyRelationKind::ModelOwnsBody),
            created_ref(labels.model.clone()),
            created_ref(labels.body.clone()),
        )
        .relation(
            format!("{}.owns_lump", labels.body),
            RelationKind::Topology(TopologyRelationKind::BodyOwnsLump),
            created_ref(labels.body.clone()),
            created_ref(labels.lump.clone()),
        )
        .relation(
            format!("{}.owns_region", labels.lump),
            RelationKind::Topology(TopologyRelationKind::LumpOwnsRegion),
            created_ref(labels.lump.clone()),
            created_ref(labels.region.clone()),
        )
        .relation(
            format!("{}.owns_shell", labels.region),
            RelationKind::Topology(TopologyRelationKind::RegionOwnsShell),
            created_ref(labels.region.clone()),
            created_ref(labels.shell.clone()),
        )
        .relation(
            format!("{}.owns_face", labels.shell),
            RelationKind::Topology(TopologyRelationKind::ShellOwnsFace),
            created_ref(labels.shell.clone()),
            created_ref(labels.face.clone()),
        )
        .relation(
            format!("{}.outer_loop", labels.face),
            RelationKind::Topology(TopologyRelationKind::FaceOuterLoop),
            created_ref(labels.face.clone()),
            created_ref(labels.outer_loop.clone()),
        )
        .relation(
            format!("{}.owns_half_edge", labels.outer_loop),
            RelationKind::Topology(TopologyRelationKind::LoopOwnsHalfEdge),
            created_ref(labels.outer_loop.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.owns_half_edge", labels.wire),
            RelationKind::Topology(TopologyRelationKind::WireOwnsHalfEdge),
            created_ref(labels.wire.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.next", labels.half_edge),
            RelationKind::Topology(TopologyRelationKind::HalfEdgeNext),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.prev", labels.half_edge),
            RelationKind::Topology(TopologyRelationKind::HalfEdgePrev),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.radial", labels.half_edge),
            RelationKind::Topology(TopologyRelationKind::HalfEdgeRadialNext),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.edge", labels.half_edge),
            RelationKind::Topology(TopologyRelationKind::HalfEdgeUsesEdge),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.edge.clone()),
        )
        .relation(
            format!("{}.start_vertex", labels.half_edge),
            RelationKind::Topology(TopologyRelationKind::HalfEdgeStartsAtVertex),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.vertex.clone()),
        )
        .relation(
            format!("{}.end_vertex", labels.half_edge),
            RelationKind::Topology(TopologyRelationKind::HalfEdgeEndsAtVertex),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.vertex.clone()),
        )
        .persistent_name_for(labels.model.clone())
        .persistent_name_for(labels.body.clone())
        .persistent_name_for(labels.lump.clone())
        .persistent_name_for(labels.region.clone())
        .persistent_name_for(labels.shell.clone())
        .persistent_name_for(labels.face.clone())
        .persistent_name_for(labels.outer_loop.clone())
        .persistent_name_for(labels.wire.clone())
        .persistent_name_for(labels.half_edge.clone())
        .persistent_name_for(labels.edge.clone())
        .persistent_name_for(labels.vertex.clone());

    builder.finish(MutationOrigin::Seed)
}

fn collect_persistent_name_ids(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    labels: &MinimalTopologyLabels,
) -> Vec<forge_relational::facade::identity::EntityId> {
    [
        labels.model.clone(),
        labels.body.clone(),
        labels.lump.clone(),
        labels.region.clone(),
        labels.shell.clone(),
        labels.face.clone(),
        labels.outer_loop.clone(),
        labels.wire.clone(),
        labels.half_edge.clone(),
        labels.edge.clone(),
        labels.vertex.clone(),
    ]
    .into_iter()
    .map(|topology_label| {
        find_seeded_entity(
            read_view,
            EntityKind::Naming(NamingEntityKind::PersistentName),
            format!("{topology_label}.persistent_name").as_str(),
        )
    })
    .collect()
}
