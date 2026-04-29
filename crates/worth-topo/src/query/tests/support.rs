use super::*;

pub(super) fn seed_sheet_disk_topology(
    workspace: &mut ForgeQueryWorkspace,
) -> forge_query::facade::ForgeQueryWriteReceipt {
    let entity_ids = [
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Model.kind_name(),
            "model-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Body.kind_name(),
            "body-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Lump.kind_name(),
            "lump-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Region.kind_name(),
            "region-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Shell.kind_name(),
            "shell-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Face.kind_name(),
            "face-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Loop.kind_name(),
            "loop-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Wire.kind_name(),
            "wire-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::HalfEdge.kind_name(),
            "half-edge-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Edge.kind_name(),
            "edge-a",
        ),
        insert_entity(
            workspace,
            WorthTopologyEntityKind::Vertex.kind_name(),
            "vertex-a",
        ),
    ];

    insert_relation(
        workspace,
        WorthTopologyRelationKind::ModelOwnsBody.kind_name(),
        &entity_ids[0],
        &entity_ids[1],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::BodyOwnsLump.kind_name(),
        &entity_ids[1],
        &entity_ids[2],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::LumpOwnsRegion.kind_name(),
        &entity_ids[2],
        &entity_ids[3],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::RegionOwnsShell.kind_name(),
        &entity_ids[3],
        &entity_ids[4],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::ShellOwnsFace.kind_name(),
        &entity_ids[4],
        &entity_ids[5],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::FaceOuterLoop.kind_name(),
        &entity_ids[5],
        &entity_ids[6],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::LoopOwnsHalfEdge.kind_name(),
        &entity_ids[6],
        &entity_ids[8],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::WireOwnsHalfEdge.kind_name(),
        &entity_ids[7],
        &entity_ids[8],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::HalfEdgeNext.kind_name(),
        &entity_ids[8],
        &entity_ids[8],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::HalfEdgePrev.kind_name(),
        &entity_ids[8],
        &entity_ids[8],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::HalfEdgeRadialNext.kind_name(),
        &entity_ids[8],
        &entity_ids[8],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::HalfEdgeUsesEdge.kind_name(),
        &entity_ids[8],
        &entity_ids[9],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::HalfEdgeStartsAtVertex.kind_name(),
        &entity_ids[8],
        &entity_ids[10],
    );
    insert_relation(
        workspace,
        WorthTopologyRelationKind::HalfEdgeEndsAtVertex.kind_name(),
        &entity_ids[8],
        &entity_ids[10],
    )
}

fn insert_entity(workspace: &mut ForgeQueryWorkspace, kind: &str, structure: &str) -> String {
    workspace
        .insert("WorthTopologyEntity", |builder| {
            builder
                .metadata(
                    WorthTopologyQueryMutationEvidence::metadata_key(),
                    default_query_mutation_evidence([
                        "topology.structure".to_string(),
                        "naming.persistent_name".to_string(),
                    ]),
                )
                .aspect("topology.kind", kind)
                .aspect("topology.structure", structure)
                .aspect("naming.persistent_name", structure)
        })
        .expect("entity insert should succeed")
        .deltas()[0]
        .entity_identity
        .clone()
}

fn insert_relation(
    workspace: &mut ForgeQueryWorkspace,
    kind: &str,
    source: &str,
    target: &str,
) -> forge_query::facade::ForgeQueryWriteReceipt {
    let worth_kind = parse_relation_kind(kind).expect("test relation kind must parse");
    let dependency_path =
        topology_relation_dependency_path(worth_kind).map(|path| path.to_string());
    workspace
        .insert("WorthTopologyRelation", |builder| {
            let builder = builder
                .metadata(
                    WorthTopologyQueryMutationEvidence::metadata_key(),
                    default_query_mutation_evidence(
                        dependency_path.clone().into_iter().collect::<Vec<_>>(),
                    ),
                )
                .aspect("topology.kind", kind)
                .aspect("topology.source_identity", source)
                .aspect("topology.target_identity", target);
            if let Some(path) = topology_relation_dependency_path(worth_kind) {
                builder.aspect(path, kind)
            } else {
                builder
            }
        })
        .expect("relation insert should succeed")
}

pub(super) fn default_query_mutation_evidence(
    touched_aspect_paths: impl IntoIterator<Item = String>,
) -> WorthTopologyQueryMutationEvidence {
    WorthTopologyQueryMutationEvidence {
        authority_snapshot_id: 7,
        authority_branch_id: "worth.query.main".to_string(),
        authoritative_mutation_origin: WorthMutationOrigin::LocalEdit,
        derivation_origin: WorthMutationOrigin::LocalEdit,
        truth_basis_digest_hex: "query-topology-test-basis".to_string(),
        touched_aspect_paths: touched_aspect_paths.into_iter().collect(),
        precision_fallback_count: 0,
        precision_budget_fallback_count: 0,
    }
}
