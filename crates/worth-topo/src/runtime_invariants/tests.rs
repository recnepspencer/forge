#[cfg(test)]
mod runtime_invariant_tests {
    use forge_relational::facade::transactions::TransactionCommitError;
    use schema::facade::topology_authoring::{seed_minimal_topology, verify_topology_intent};
    use schema::facade::{
        bootstrap_runtime_invariant_plan, CreateKey, EntityKind, EntityReference, MutationOrigin,
        NamingEntityKind, NamingRelationKind, RawTopologyIntent, RelationKind,
        TopologyAuthorityError, TopologyEntityKind, TopologyMutation, TopologyRelationKind,
    };

    use crate::facade::{milestone_one_runtime_builder, milestone_one_runtime_invariants};

    #[test]
    fn runtime_invariant_pack_matches_declared_bootstrap_plan() {
        let declared = bootstrap_runtime_invariant_plan();
        let registrations =
            milestone_one_runtime_invariants().expect("runtime invariant registrations");

        assert_eq!(registrations.len(), declared.topology.len());
    }

    #[test]
    fn runtime_builder_helper_applies_schema_and_runtime_invariants() {
        let _runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();
    }

    #[test]
    fn runtime_invariants_accept_seeded_topology_on_the_actual_authority_path() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "runtime-invariant-seed")
            .expect("seeded milestone-one topology should commit through runtime invariants");

        let read = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("seeded snapshot should remain readable");
        assert!(read.get_entity(seeded.model).is_some());
        assert!(read.get_entity(seeded.shell).is_some());
    }

    #[test]
    fn runtime_invariants_block_create_batches_missing_persistent_names() {
        let mut runtime = milestone_one_runtime_builder()
            .expect(" milestone one runtime builder")
            .build();

        let intent = RawTopologyIntent::new(
            vec![
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("unnamed.model"),
                    kind: EntityKind::Topology(TopologyEntityKind::Model),
                },
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new("unnamed.body"),
                    kind: EntityKind::Topology(TopologyEntityKind::Body),
                },
                TopologyMutation::CreateRelation {
                    create_key: CreateKey::new("unnamed.model.owns_body"),
                    kind: RelationKind::Topology(TopologyRelationKind::ModelOwnsBody),
                    source: EntityReference::Created(CreateKey::new("unnamed.model")),
                    target: EntityReference::Created(CreateKey::new("unnamed.body")),
                },
            ],
            MutationOrigin::LocalEdit,
        );

        let error = verify_topology_intent(&mut runtime, intent)
            .expect_err("missing persistent-name coverage must block commit")
            .into_error();

        assert!(matches!(
            error,
            TopologyAuthorityError::Commit(TransactionCommitError::Conflict { .. })
        ));
    }

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

        let error = verify_topology_intent(&mut runtime, intent)
            .expect_err("branch vertex with reused edge identities must block commit")
            .into_error();

        assert!(matches!(
            error,
            TopologyAuthorityError::Commit(TransactionCommitError::Conflict { .. })
        ));
    }

    fn entity(create_key: &str, kind: TopologyEntityKind) -> TopologyMutation {
        TopologyMutation::CreateEntity {
            create_key: CreateKey::new(create_key),
            kind: EntityKind::Topology(kind),
        }
    }

    fn relation(
        create_key: &str,
        kind: TopologyRelationKind,
        source: &str,
        target: &str,
    ) -> TopologyMutation {
        TopologyMutation::CreateRelation {
            create_key: CreateKey::new(create_key),
            kind: RelationKind::Topology(kind),
            source: EntityReference::Created(CreateKey::new(source)),
            target: EntityReference::Created(CreateKey::new(target)),
        }
    }

    fn naming_bundle<'a>(
        topology_keys: &'a [&'a str],
    ) -> impl Iterator<Item = TopologyMutation> + 'a {
        topology_keys.iter().flat_map(|topology_key| {
            let name_key = format!("{topology_key}.persistent_name");
            [
                TopologyMutation::CreateEntity {
                    create_key: CreateKey::new(name_key.clone()),
                    kind: EntityKind::Naming(NamingEntityKind::PersistentName),
                },
                TopologyMutation::CreateRelation {
                    create_key: CreateKey::new(format!("{name_key}.targets")),
                    kind: RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity),
                    source: EntityReference::Created(CreateKey::new(name_key)),
                    target: EntityReference::Created(CreateKey::new(*topology_key)),
                },
            ]
            .into_iter()
        })
    }
}
