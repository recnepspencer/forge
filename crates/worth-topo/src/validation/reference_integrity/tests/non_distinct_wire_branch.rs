use super::*;

#[test]
fn runtime_invariants_block_illegal_wire_branch_with_non_distinct_edges() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();

    let intent = RawTopologyIntent::new(
        vec![
            entity("branch.model", TopologyEntityKind::Model),
            entity("branch.body", TopologyEntityKind::Body),
            entity("branch.lump", TopologyEntityKind::Lump),
            entity("branch.region", TopologyEntityKind::Region),
            entity("branch.shell", TopologyEntityKind::Shell),
            entity("branch.face", TopologyEntityKind::Face),
            entity("branch.loop", TopologyEntityKind::Loop),
            entity("branch.wire", TopologyEntityKind::Wire),
            entity("branch.he0", TopologyEntityKind::HalfEdge),
            entity("branch.he1", TopologyEntityKind::HalfEdge),
            entity("branch.he2", TopologyEntityKind::HalfEdge),
            entity("branch.edge0", TopologyEntityKind::Edge),
            entity("branch.edge2", TopologyEntityKind::Edge),
            entity("branch.center", TopologyEntityKind::Vertex),
            entity("branch.v1", TopologyEntityKind::Vertex),
            entity("branch.v2", TopologyEntityKind::Vertex),
            entity("branch.v3", TopologyEntityKind::Vertex),
            relation(
                "branch.model.owns_body",
                TopologyRelationKind::ModelOwnsBody,
                "branch.model",
                "branch.body",
            ),
            relation(
                "branch.body.owns_lump",
                TopologyRelationKind::BodyOwnsLump,
                "branch.body",
                "branch.lump",
            ),
            relation(
                "branch.lump.owns_region",
                TopologyRelationKind::LumpOwnsRegion,
                "branch.lump",
                "branch.region",
            ),
            relation(
                "branch.region.owns_shell",
                TopologyRelationKind::RegionOwnsShell,
                "branch.region",
                "branch.shell",
            ),
            relation(
                "branch.shell.owns_face",
                TopologyRelationKind::ShellOwnsFace,
                "branch.shell",
                "branch.face",
            ),
            relation(
                "branch.face.outer_loop",
                TopologyRelationKind::FaceOuterLoop,
                "branch.face",
                "branch.loop",
            ),
            relation(
                "branch.loop.owns_he0",
                TopologyRelationKind::LoopOwnsHalfEdge,
                "branch.loop",
                "branch.he0",
            ),
            relation(
                "branch.loop.owns_he1",
                TopologyRelationKind::LoopOwnsHalfEdge,
                "branch.loop",
                "branch.he1",
            ),
            relation(
                "branch.loop.owns_he2",
                TopologyRelationKind::LoopOwnsHalfEdge,
                "branch.loop",
                "branch.he2",
            ),
            relation(
                "branch.wire.owns_he0",
                TopologyRelationKind::WireOwnsHalfEdge,
                "branch.wire",
                "branch.he0",
            ),
            relation(
                "branch.wire.owns_he1",
                TopologyRelationKind::WireOwnsHalfEdge,
                "branch.wire",
                "branch.he1",
            ),
            relation(
                "branch.wire.owns_he2",
                TopologyRelationKind::WireOwnsHalfEdge,
                "branch.wire",
                "branch.he2",
            ),
            relation(
                "branch.he0.next",
                TopologyRelationKind::HalfEdgeNext,
                "branch.he0",
                "branch.he0",
            ),
            relation(
                "branch.he0.prev",
                TopologyRelationKind::HalfEdgePrev,
                "branch.he0",
                "branch.he0",
            ),
            relation(
                "branch.he1.next",
                TopologyRelationKind::HalfEdgeNext,
                "branch.he1",
                "branch.he1",
            ),
            relation(
                "branch.he1.prev",
                TopologyRelationKind::HalfEdgePrev,
                "branch.he1",
                "branch.he1",
            ),
            relation(
                "branch.he2.next",
                TopologyRelationKind::HalfEdgeNext,
                "branch.he2",
                "branch.he2",
            ),
            relation(
                "branch.he2.prev",
                TopologyRelationKind::HalfEdgePrev,
                "branch.he2",
                "branch.he2",
            ),
            relation(
                "branch.he0.radial",
                TopologyRelationKind::HalfEdgeRadialNext,
                "branch.he0",
                "branch.he0",
            ),
            relation(
                "branch.he1.radial",
                TopologyRelationKind::HalfEdgeRadialNext,
                "branch.he1",
                "branch.he1",
            ),
            relation(
                "branch.he2.radial",
                TopologyRelationKind::HalfEdgeRadialNext,
                "branch.he2",
                "branch.he2",
            ),
            relation(
                "branch.he0.edge",
                TopologyRelationKind::HalfEdgeUsesEdge,
                "branch.he0",
                "branch.edge0",
            ),
            relation(
                "branch.he1.edge",
                TopologyRelationKind::HalfEdgeUsesEdge,
                "branch.he1",
                "branch.edge0",
            ),
            relation(
                "branch.he2.edge",
                TopologyRelationKind::HalfEdgeUsesEdge,
                "branch.he2",
                "branch.edge2",
            ),
            relation(
                "branch.he0.start",
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                "branch.he0",
                "branch.center",
            ),
            relation(
                "branch.he0.end",
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                "branch.he0",
                "branch.v1",
            ),
            relation(
                "branch.he1.start",
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                "branch.he1",
                "branch.center",
            ),
            relation(
                "branch.he1.end",
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                "branch.he1",
                "branch.v2",
            ),
            relation(
                "branch.he2.start",
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                "branch.he2",
                "branch.center",
            ),
            relation(
                "branch.he2.end",
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                "branch.he2",
                "branch.v3",
            ),
        ]
        .into_iter()
        .chain(naming_bundle(&[
            "branch.model",
            "branch.body",
            "branch.lump",
            "branch.region",
            "branch.shell",
            "branch.face",
            "branch.loop",
            "branch.wire",
            "branch.he0",
            "branch.he1",
            "branch.he2",
            "branch.edge0",
            "branch.edge2",
            "branch.center",
            "branch.v1",
            "branch.v2",
            "branch.v3",
        ]))
        .collect(),
        MutationOrigin::LocalEdit,
    );

    let error = commit_raw_intent(&mut runtime, intent)
        .expect_err("branch vertex with reused edge identities must block commit");

    assert!(matches!(error, TransactionCommitError::Conflict { .. }));
}
