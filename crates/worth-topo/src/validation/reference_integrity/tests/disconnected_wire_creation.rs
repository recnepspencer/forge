use super::*;

#[test]
fn runtime_invariants_block_disconnected_wire_creation_at_commit_boundary() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let intent = RawTopologyIntent::new(
        vec![
            entity("wire.model", TopologyEntityKind::Model),
            entity("wire.body", TopologyEntityKind::Body),
            entity("wire.lump", TopologyEntityKind::Lump),
            entity("wire.region", TopologyEntityKind::Region),
            entity("wire.shell", TopologyEntityKind::Shell),
            entity("wire.face", TopologyEntityKind::Face),
            entity("wire.loop", TopologyEntityKind::Loop),
            entity("wire.wire", TopologyEntityKind::Wire),
            entity("wire.he0", TopologyEntityKind::HalfEdge),
            entity("wire.he1", TopologyEntityKind::HalfEdge),
            entity("wire.edge0", TopologyEntityKind::Edge),
            entity("wire.edge1", TopologyEntityKind::Edge),
            entity("wire.v0", TopologyEntityKind::Vertex),
            entity("wire.v1", TopologyEntityKind::Vertex),
            entity("wire.v2", TopologyEntityKind::Vertex),
            entity("wire.v3", TopologyEntityKind::Vertex),
            relation(
                "wire.model.owns_body",
                TopologyRelationKind::ModelOwnsBody,
                "wire.model",
                "wire.body",
            ),
            relation(
                "wire.body.owns_lump",
                TopologyRelationKind::BodyOwnsLump,
                "wire.body",
                "wire.lump",
            ),
            relation(
                "wire.lump.owns_region",
                TopologyRelationKind::LumpOwnsRegion,
                "wire.lump",
                "wire.region",
            ),
            relation(
                "wire.region.owns_shell",
                TopologyRelationKind::RegionOwnsShell,
                "wire.region",
                "wire.shell",
            ),
            relation(
                "wire.shell.owns_face",
                TopologyRelationKind::ShellOwnsFace,
                "wire.shell",
                "wire.face",
            ),
            relation(
                "wire.face.outer_loop",
                TopologyRelationKind::FaceOuterLoop,
                "wire.face",
                "wire.loop",
            ),
            relation(
                "wire.loop.owns_he0",
                TopologyRelationKind::LoopOwnsHalfEdge,
                "wire.loop",
                "wire.he0",
            ),
            relation(
                "wire.loop.owns_he1",
                TopologyRelationKind::LoopOwnsHalfEdge,
                "wire.loop",
                "wire.he1",
            ),
            relation(
                "wire.wire.owns_he0",
                TopologyRelationKind::WireOwnsHalfEdge,
                "wire.wire",
                "wire.he0",
            ),
            relation(
                "wire.wire.owns_he1",
                TopologyRelationKind::WireOwnsHalfEdge,
                "wire.wire",
                "wire.he1",
            ),
            relation(
                "wire.he0.next",
                TopologyRelationKind::HalfEdgeNext,
                "wire.he0",
                "wire.he0",
            ),
            relation(
                "wire.he0.prev",
                TopologyRelationKind::HalfEdgePrev,
                "wire.he0",
                "wire.he0",
            ),
            relation(
                "wire.he1.next",
                TopologyRelationKind::HalfEdgeNext,
                "wire.he1",
                "wire.he1",
            ),
            relation(
                "wire.he1.prev",
                TopologyRelationKind::HalfEdgePrev,
                "wire.he1",
                "wire.he1",
            ),
            relation(
                "wire.he0.radial",
                TopologyRelationKind::HalfEdgeRadialNext,
                "wire.he0",
                "wire.he0",
            ),
            relation(
                "wire.he1.radial",
                TopologyRelationKind::HalfEdgeRadialNext,
                "wire.he1",
                "wire.he1",
            ),
            relation(
                "wire.he0.edge",
                TopologyRelationKind::HalfEdgeUsesEdge,
                "wire.he0",
                "wire.edge0",
            ),
            relation(
                "wire.he1.edge",
                TopologyRelationKind::HalfEdgeUsesEdge,
                "wire.he1",
                "wire.edge1",
            ),
            relation(
                "wire.he0.start",
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                "wire.he0",
                "wire.v0",
            ),
            relation(
                "wire.he0.end",
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                "wire.he0",
                "wire.v1",
            ),
            relation(
                "wire.he1.start",
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                "wire.he1",
                "wire.v2",
            ),
            relation(
                "wire.he1.end",
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                "wire.he1",
                "wire.v3",
            ),
        ]
        .into_iter()
        .chain(naming_bundle(&[
            "wire.model",
            "wire.body",
            "wire.lump",
            "wire.region",
            "wire.shell",
            "wire.face",
            "wire.loop",
            "wire.wire",
            "wire.he0",
            "wire.he1",
            "wire.edge0",
            "wire.edge1",
            "wire.v0",
            "wire.v1",
            "wire.v2",
            "wire.v3",
        ]))
        .collect(),
        MutationOrigin::LocalEdit,
    );

    let error = verify_topology_intent(&mut runtime, intent)
        .expect_err("disconnected wire graph must block commit")
        .into_error();

    assert!(matches!(
        error,
        TopologyAuthorityError::Commit(TransactionCommitError::Conflict { .. })
    ));
}
