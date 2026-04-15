#[cfg(test)]
mod runtime_invariant_tests {
    use forge_relational::facade::transactions::TransactionCommitError;
    use worth_schema::facade::{
        seed_minimal_topology, worth_bootstrap_runtime_invariant_plan, RawWorthTopologyIntent,
        WorthCreateKey, WorthEntityKind, WorthEntityReference, WorthMutationOrigin,
        WorthNamingEntityKind, WorthNamingRelationKind, WorthRelationKind,
        WorthTopologyAuthority, WorthTopologyAuthorityError, WorthTopologyEntityKind,
        WorthTopologyMutation, WorthTopologyRelationKind,
    };

    use crate::facade::{
        worth_milestone_one_runtime_builder, worth_milestone_one_runtime_invariants,
    };

    #[test]
    fn runtime_invariant_pack_matches_declared_bootstrap_plan() {
        let declared = worth_bootstrap_runtime_invariant_plan();
        let registrations =
            worth_milestone_one_runtime_invariants().expect("runtime invariant registrations");

        assert_eq!(registrations.len(), declared.topology.len());
    }

    #[test]
    fn runtime_builder_helper_applies_schema_and_runtime_invariants() {
        let _runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
    }

    #[test]
    fn runtime_invariants_accept_seeded_topology_on_the_actual_authority_path() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
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
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let intent = RawWorthTopologyIntent::new(
            vec![
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("unnamed.model"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
                },
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new("unnamed.body"),
                    kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Body),
                },
                WorthTopologyMutation::CreateRelation {
                    create_key: WorthCreateKey::new("unnamed.model.owns_body"),
                    kind: WorthRelationKind::Topology(WorthTopologyRelationKind::ModelOwnsBody),
                    source: WorthEntityReference::Created(WorthCreateKey::new("unnamed.model")),
                    target: WorthEntityReference::Created(WorthCreateKey::new("unnamed.body")),
                },
            ],
            WorthMutationOrigin::LocalEdit,
        );

        let error = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent(intent)
            .expect_err("missing persistent-name coverage must block commit");

        assert!(matches!(
            error,
            WorthTopologyAuthorityError::Commit(TransactionCommitError::Conflict { .. })
        ));
    }

    #[test]
    fn runtime_invariants_block_disconnected_wire_creation_at_commit_boundary() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let intent = RawWorthTopologyIntent::new(
            vec![
                entity("wire.model", WorthTopologyEntityKind::Model),
                entity("wire.body", WorthTopologyEntityKind::Body),
                entity("wire.lump", WorthTopologyEntityKind::Lump),
                entity("wire.region", WorthTopologyEntityKind::Region),
                entity("wire.shell", WorthTopologyEntityKind::Shell),
                entity("wire.face", WorthTopologyEntityKind::Face),
                entity("wire.loop", WorthTopologyEntityKind::Loop),
                entity("wire.wire", WorthTopologyEntityKind::Wire),
                entity("wire.he0", WorthTopologyEntityKind::HalfEdge),
                entity("wire.he1", WorthTopologyEntityKind::HalfEdge),
                entity("wire.edge0", WorthTopologyEntityKind::Edge),
                entity("wire.edge1", WorthTopologyEntityKind::Edge),
                entity("wire.v0", WorthTopologyEntityKind::Vertex),
                entity("wire.v1", WorthTopologyEntityKind::Vertex),
                entity("wire.v2", WorthTopologyEntityKind::Vertex),
                entity("wire.v3", WorthTopologyEntityKind::Vertex),
                relation(
                    "wire.model.owns_body",
                    WorthTopologyRelationKind::ModelOwnsBody,
                    "wire.model",
                    "wire.body",
                ),
                relation(
                    "wire.body.owns_lump",
                    WorthTopologyRelationKind::BodyOwnsLump,
                    "wire.body",
                    "wire.lump",
                ),
                relation(
                    "wire.lump.owns_region",
                    WorthTopologyRelationKind::LumpOwnsRegion,
                    "wire.lump",
                    "wire.region",
                ),
                relation(
                    "wire.region.owns_shell",
                    WorthTopologyRelationKind::RegionOwnsShell,
                    "wire.region",
                    "wire.shell",
                ),
                relation(
                    "wire.shell.owns_face",
                    WorthTopologyRelationKind::ShellOwnsFace,
                    "wire.shell",
                    "wire.face",
                ),
                relation(
                    "wire.face.outer_loop",
                    WorthTopologyRelationKind::FaceOuterLoop,
                    "wire.face",
                    "wire.loop",
                ),
                relation(
                    "wire.loop.owns_he0",
                    WorthTopologyRelationKind::LoopOwnsHalfEdge,
                    "wire.loop",
                    "wire.he0",
                ),
                relation(
                    "wire.loop.owns_he1",
                    WorthTopologyRelationKind::LoopOwnsHalfEdge,
                    "wire.loop",
                    "wire.he1",
                ),
                relation(
                    "wire.wire.owns_he0",
                    WorthTopologyRelationKind::WireOwnsHalfEdge,
                    "wire.wire",
                    "wire.he0",
                ),
                relation(
                    "wire.wire.owns_he1",
                    WorthTopologyRelationKind::WireOwnsHalfEdge,
                    "wire.wire",
                    "wire.he1",
                ),
                relation(
                    "wire.he0.next",
                    WorthTopologyRelationKind::HalfEdgeNext,
                    "wire.he0",
                    "wire.he0",
                ),
                relation(
                    "wire.he0.prev",
                    WorthTopologyRelationKind::HalfEdgePrev,
                    "wire.he0",
                    "wire.he0",
                ),
                relation(
                    "wire.he1.next",
                    WorthTopologyRelationKind::HalfEdgeNext,
                    "wire.he1",
                    "wire.he1",
                ),
                relation(
                    "wire.he1.prev",
                    WorthTopologyRelationKind::HalfEdgePrev,
                    "wire.he1",
                    "wire.he1",
                ),
                relation(
                    "wire.he0.radial",
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                    "wire.he0",
                    "wire.he0",
                ),
                relation(
                    "wire.he1.radial",
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                    "wire.he1",
                    "wire.he1",
                ),
                relation(
                    "wire.he0.edge",
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                    "wire.he0",
                    "wire.edge0",
                ),
                relation(
                    "wire.he1.edge",
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                    "wire.he1",
                    "wire.edge1",
                ),
                relation(
                    "wire.he0.start",
                    WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                    "wire.he0",
                    "wire.v0",
                ),
                relation(
                    "wire.he0.end",
                    WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
                    "wire.he0",
                    "wire.v1",
                ),
                relation(
                    "wire.he1.start",
                    WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                    "wire.he1",
                    "wire.v2",
                ),
                relation(
                    "wire.he1.end",
                    WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
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
            WorthMutationOrigin::LocalEdit,
        );

        let error = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent(intent)
            .expect_err("disconnected wire graph must block commit");

        assert!(matches!(
            error,
            WorthTopologyAuthorityError::Commit(TransactionCommitError::Conflict { .. })
        ));
    }

    #[test]
    fn runtime_invariants_block_illegal_wire_branch_with_non_distinct_edges() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();

        let intent = RawWorthTopologyIntent::new(
            vec![
                entity("branch.model", WorthTopologyEntityKind::Model),
                entity("branch.body", WorthTopologyEntityKind::Body),
                entity("branch.lump", WorthTopologyEntityKind::Lump),
                entity("branch.region", WorthTopologyEntityKind::Region),
                entity("branch.shell", WorthTopologyEntityKind::Shell),
                entity("branch.face", WorthTopologyEntityKind::Face),
                entity("branch.loop", WorthTopologyEntityKind::Loop),
                entity("branch.wire", WorthTopologyEntityKind::Wire),
                entity("branch.he0", WorthTopologyEntityKind::HalfEdge),
                entity("branch.he1", WorthTopologyEntityKind::HalfEdge),
                entity("branch.he2", WorthTopologyEntityKind::HalfEdge),
                entity("branch.edge0", WorthTopologyEntityKind::Edge),
                entity("branch.edge2", WorthTopologyEntityKind::Edge),
                entity("branch.center", WorthTopologyEntityKind::Vertex),
                entity("branch.v1", WorthTopologyEntityKind::Vertex),
                entity("branch.v2", WorthTopologyEntityKind::Vertex),
                entity("branch.v3", WorthTopologyEntityKind::Vertex),
                relation(
                    "branch.model.owns_body",
                    WorthTopologyRelationKind::ModelOwnsBody,
                    "branch.model",
                    "branch.body",
                ),
                relation(
                    "branch.body.owns_lump",
                    WorthTopologyRelationKind::BodyOwnsLump,
                    "branch.body",
                    "branch.lump",
                ),
                relation(
                    "branch.lump.owns_region",
                    WorthTopologyRelationKind::LumpOwnsRegion,
                    "branch.lump",
                    "branch.region",
                ),
                relation(
                    "branch.region.owns_shell",
                    WorthTopologyRelationKind::RegionOwnsShell,
                    "branch.region",
                    "branch.shell",
                ),
                relation(
                    "branch.shell.owns_face",
                    WorthTopologyRelationKind::ShellOwnsFace,
                    "branch.shell",
                    "branch.face",
                ),
                relation(
                    "branch.face.outer_loop",
                    WorthTopologyRelationKind::FaceOuterLoop,
                    "branch.face",
                    "branch.loop",
                ),
                relation(
                    "branch.loop.owns_he0",
                    WorthTopologyRelationKind::LoopOwnsHalfEdge,
                    "branch.loop",
                    "branch.he0",
                ),
                relation(
                    "branch.loop.owns_he1",
                    WorthTopologyRelationKind::LoopOwnsHalfEdge,
                    "branch.loop",
                    "branch.he1",
                ),
                relation(
                    "branch.loop.owns_he2",
                    WorthTopologyRelationKind::LoopOwnsHalfEdge,
                    "branch.loop",
                    "branch.he2",
                ),
                relation(
                    "branch.wire.owns_he0",
                    WorthTopologyRelationKind::WireOwnsHalfEdge,
                    "branch.wire",
                    "branch.he0",
                ),
                relation(
                    "branch.wire.owns_he1",
                    WorthTopologyRelationKind::WireOwnsHalfEdge,
                    "branch.wire",
                    "branch.he1",
                ),
                relation(
                    "branch.wire.owns_he2",
                    WorthTopologyRelationKind::WireOwnsHalfEdge,
                    "branch.wire",
                    "branch.he2",
                ),
                relation(
                    "branch.he0.next",
                    WorthTopologyRelationKind::HalfEdgeNext,
                    "branch.he0",
                    "branch.he0",
                ),
                relation(
                    "branch.he0.prev",
                    WorthTopologyRelationKind::HalfEdgePrev,
                    "branch.he0",
                    "branch.he0",
                ),
                relation(
                    "branch.he1.next",
                    WorthTopologyRelationKind::HalfEdgeNext,
                    "branch.he1",
                    "branch.he1",
                ),
                relation(
                    "branch.he1.prev",
                    WorthTopologyRelationKind::HalfEdgePrev,
                    "branch.he1",
                    "branch.he1",
                ),
                relation(
                    "branch.he2.next",
                    WorthTopologyRelationKind::HalfEdgeNext,
                    "branch.he2",
                    "branch.he2",
                ),
                relation(
                    "branch.he2.prev",
                    WorthTopologyRelationKind::HalfEdgePrev,
                    "branch.he2",
                    "branch.he2",
                ),
                relation(
                    "branch.he0.radial",
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                    "branch.he0",
                    "branch.he0",
                ),
                relation(
                    "branch.he1.radial",
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                    "branch.he1",
                    "branch.he1",
                ),
                relation(
                    "branch.he2.radial",
                    WorthTopologyRelationKind::HalfEdgeRadialNext,
                    "branch.he2",
                    "branch.he2",
                ),
                relation(
                    "branch.he0.edge",
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                    "branch.he0",
                    "branch.edge0",
                ),
                relation(
                    "branch.he1.edge",
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                    "branch.he1",
                    "branch.edge0",
                ),
                relation(
                    "branch.he2.edge",
                    WorthTopologyRelationKind::HalfEdgeUsesEdge,
                    "branch.he2",
                    "branch.edge2",
                ),
                relation(
                    "branch.he0.start",
                    WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                    "branch.he0",
                    "branch.center",
                ),
                relation(
                    "branch.he0.end",
                    WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
                    "branch.he0",
                    "branch.v1",
                ),
                relation(
                    "branch.he1.start",
                    WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                    "branch.he1",
                    "branch.center",
                ),
                relation(
                    "branch.he1.end",
                    WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
                    "branch.he1",
                    "branch.v2",
                ),
                relation(
                    "branch.he2.start",
                    WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                    "branch.he2",
                    "branch.center",
                ),
                relation(
                    "branch.he2.end",
                    WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
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
            WorthMutationOrigin::LocalEdit,
        );

        let error = WorthTopologyAuthority::new(&mut runtime)
            .apply_topology_intent(intent)
            .expect_err("branch vertex with reused edge identities must block commit");

        assert!(matches!(
            error,
            WorthTopologyAuthorityError::Commit(TransactionCommitError::Conflict { .. })
        ));
    }

    fn entity(create_key: &str, kind: WorthTopologyEntityKind) -> WorthTopologyMutation {
        WorthTopologyMutation::CreateEntity {
            create_key: WorthCreateKey::new(create_key),
            kind: WorthEntityKind::Topology(kind),
        }
    }

    fn relation(
        create_key: &str,
        kind: WorthTopologyRelationKind,
        source: &str,
        target: &str,
    ) -> WorthTopologyMutation {
        WorthTopologyMutation::CreateRelation {
            create_key: WorthCreateKey::new(create_key),
            kind: WorthRelationKind::Topology(kind),
            source: WorthEntityReference::Created(WorthCreateKey::new(source)),
            target: WorthEntityReference::Created(WorthCreateKey::new(target)),
        }
    }

    fn naming_bundle<'a>(
        topology_keys: &'a [&'a str],
    ) -> impl Iterator<Item = WorthTopologyMutation> + 'a {
        topology_keys.iter().flat_map(|topology_key| {
            let name_key = format!("{topology_key}.persistent_name");
            [
                WorthTopologyMutation::CreateEntity {
                    create_key: WorthCreateKey::new(name_key.clone()),
                    kind: WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName),
                },
                WorthTopologyMutation::CreateRelation {
                    create_key: WorthCreateKey::new(format!("{name_key}.targets")),
                    kind: WorthRelationKind::Naming(
                        WorthNamingRelationKind::PersistentNameTargetsEntity,
                    ),
                    source: WorthEntityReference::Created(WorthCreateKey::new(name_key)),
                    target: WorthEntityReference::Created(WorthCreateKey::new(*topology_key)),
                },
            ]
            .into_iter()
        })
    }
}
