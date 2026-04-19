use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::TransactionCommitError;

use crate::data::authority::{RawWorthTopologyIntent, WorthMutationOrigin, WorthTopologyAuthority};
use crate::data::entities::{WorthEntityKind, WorthNamingEntityKind, WorthTopologyEntityKind};
use crate::data::relations::{WorthRelationKind, WorthTopologyRelationKind};
use crate::data::seed::labels::WorthMinimalTopologyLabels;
use crate::data::seed::lookup::find_seeded_entity;
use crate::data::seed::types::WorthMinimalTopologySeed;
use crate::data::seed::{created_ref, WorthTopologyCreateBatchBuilder};

pub fn seed_minimal_topology(
    runtime: &mut RelationalRuntime,
    stem: &str,
) -> Result<WorthMinimalTopologySeed, TransactionCommitError> {
    let labels = WorthMinimalTopologyLabels::new(stem);
    let verified = WorthTopologyAuthority::new(runtime)
        .apply_topology_intent_traced(build_minimal_topology_intent(&labels))
        .map(|traced| traced.into_primary_result())
        .map_err(|error| match error.into_error() {
            crate::data::authority::WorthTopologyAuthorityError::Commit(error) => error,
            other => panic!("worth minimal topology seed should author successfully: {other:?}"),
        })?;
    let naming_read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect("worth seeded snapshot should remain readable");

    let ids = WorthMinimalTopologySeed {
        snapshot: verified.persisted_truth.snapshot.clone(),
        model: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
            &labels.model,
        ),
        body: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
            &labels.body,
        ),
        lump: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
            &labels.lump,
        ),
        region: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
            &labels.region,
        ),
        shell: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
            &labels.shell,
        ),
        face: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
            &labels.face,
        ),
        outer_loop: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
            &labels.outer_loop,
        ),
        wire: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
            &labels.wire,
        ),
        half_edge: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
            &labels.half_edge,
        ),
        edge: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
            &labels.edge,
        ),
        vertex: find_seeded_entity(
            &naming_read,
            WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            &labels.vertex,
        ),
        persistent_name_ids: collect_persistent_name_ids(&naming_read, &labels),
        persisted_truth: verified.persisted_truth.clone(),
        read_basis: verified.read_basis.clone(),
        read_artifact: crate::data::authority::WorthTopologyReadArtifact::from_read_basis(
            &verified.read_basis,
        ),
        certified_interpretation:
            crate::data::authority::CertifiedTopologyInterpretation::from_read_basis(
                verified.read_basis.clone(),
            ),
    };

    Ok(WorthMinimalTopologySeed { ..ids })
}

fn build_minimal_topology_intent(labels: &WorthMinimalTopologyLabels) -> RawWorthTopologyIntent {
    let builder = WorthTopologyCreateBatchBuilder::new()
        .topology_entity(
            labels.model.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
        )
        .topology_entity(
            labels.body.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
        )
        .topology_entity(
            labels.lump.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Lump),
        )
        .topology_entity(
            labels.region.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Region),
        )
        .topology_entity(
            labels.shell.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Shell),
        )
        .topology_entity(
            labels.face.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Face),
        )
        .topology_entity(
            labels.outer_loop.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Loop),
        )
        .topology_entity(
            labels.wire.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Wire),
        )
        .topology_entity(
            labels.half_edge.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::HalfEdge),
        )
        .topology_entity(
            labels.edge.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Edge),
        )
        .topology_entity(
            labels.vertex.clone(),
            WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
        )
        .relation(
            format!("{}.owns_body", labels.model),
            WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody),
            created_ref(labels.model.clone()),
            created_ref(labels.body.clone()),
        )
        .relation(
            format!("{}.owns_lump", labels.body),
            WorthRelationKind::Topology(WorthTopologyRelationKind::BodyOwnsLump),
            created_ref(labels.body.clone()),
            created_ref(labels.lump.clone()),
        )
        .relation(
            format!("{}.owns_region", labels.lump),
            WorthRelationKind::Topology(WorthTopologyRelationKind::LumpOwnsRegion),
            created_ref(labels.lump.clone()),
            created_ref(labels.region.clone()),
        )
        .relation(
            format!("{}.owns_shell", labels.region),
            WorthRelationKind::Topology(WorthTopologyRelationKind::RegionOwnsShell),
            created_ref(labels.region.clone()),
            created_ref(labels.shell.clone()),
        )
        .relation(
            format!("{}.owns_face", labels.shell),
            WorthRelationKind::Topology(WorthTopologyRelationKind::ShellOwnsFace),
            created_ref(labels.shell.clone()),
            created_ref(labels.face.clone()),
        )
        .relation(
            format!("{}.outer_loop", labels.face),
            WorthRelationKind::Topology(WorthTopologyRelationKind::FaceOuterLoop),
            created_ref(labels.face.clone()),
            created_ref(labels.outer_loop.clone()),
        )
        .relation(
            format!("{}.owns_half_edge", labels.outer_loop),
            WorthRelationKind::Topology(WorthTopologyRelationKind::LoopOwnsHalfEdge),
            created_ref(labels.outer_loop.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.owns_half_edge", labels.wire),
            WorthRelationKind::Topology(WorthTopologyRelationKind::WireOwnsHalfEdge),
            created_ref(labels.wire.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.next", labels.half_edge),
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.prev", labels.half_edge),
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgePrev),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.radial", labels.half_edge),
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeRadialNext),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.half_edge.clone()),
        )
        .relation(
            format!("{}.edge", labels.half_edge),
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeUsesEdge),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.edge.clone()),
        )
        .relation(
            format!("{}.start_vertex", labels.half_edge),
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeStartsAtVertex),
            created_ref(labels.half_edge.clone()),
            created_ref(labels.vertex.clone()),
        )
        .relation(
            format!("{}.end_vertex", labels.half_edge),
            WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeEndsAtVertex),
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

    builder.finish(WorthMutationOrigin::Seed)
}

fn collect_persistent_name_ids(
    read_view: &forge_relational::facade::runtime::RelationalReadView,
    labels: &WorthMinimalTopologyLabels,
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
            WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName),
            format!("{topology_label}.persistent_name").as_str(),
        )
    })
    .collect()
}
