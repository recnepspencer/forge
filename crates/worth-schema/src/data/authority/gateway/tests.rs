use super::*;
use crate::data::aspects::{Aspect, TopologyAspect};
use crate::data::entities::{DiagnosticsEntityKind, EntityKind, TopologyEntityKind};
use crate::data::relations::{NamingRelationKind, TopologyRelationKind};
use crate::data::seed::seed_minimal_topology;
use forge_relational::facade::runtime::RelationalRuntimeApi;

#[test]
fn authority_rejects_identity_shaped_entity_mutations_without_existing_truth() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();
    let _seeded = seed_minimal_topology(&mut runtime, "authority-create-reject").unwrap();

    let intent = RawTopologyIntent::new(
        vec![TopologyMutation::UpsertEntity {
            entity_id: EntityId::new(
                forge_relational::facade::identity::PartitionId::main(),
                999,
                1,
            ),
            kind: EntityKind::Topology(TopologyEntityKind::Shell),
        }],
        crate::data::authority::MutationOrigin::LocalEdit,
    );

    let error = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(intent)
        .unwrap_err()
        .into_error();

    assert!(matches!(
        error,
        TopologyAuthorityError::UnsupportedIdentityEntityMutation(_)
    ));
}

#[test]
fn authority_can_publish_same_commit_topology_graph_creates_with_symbolic_keys() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();

    let intent = RawTopologyIntent::new(
        vec![
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new("create.model"),
                kind: EntityKind::Topology(TopologyEntityKind::Model),
            },
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new("create.body"),
                kind: EntityKind::Topology(TopologyEntityKind::Body),
            },
            TopologyMutation::CreateRelation {
                create_key: CreateKey::new("create.model.owns_body"),
                kind: RelationKind::Topology(TopologyRelationKind::ModelOwnsBody),
                source: EntityReference::Created(CreateKey::new("create.model")),
                target: EntityReference::Created(CreateKey::new("create.body")),
            },
        ],
        crate::data::authority::MutationOrigin::Seed,
    );

    let verified = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(intent)
        .expect("create batch should commit")
        .into_primary_result();

    assert_eq!(verified.commits.len(), 1);
    assert_eq!(verified.branch_id.0, "main");
    let read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect("verified create snapshot should remain readable");
    assert_eq!(
        read.entities()
            .iter()
            .filter(|record| {
                EntityKind::from_kind_id(record.kind.kind_id)
                    .and_then(|kind| entity_record_label(record, kind))
                    .is_some_and(|label| label.starts_with("create."))
            })
            .count(),
        2
    );
    assert_eq!(
        read.relations()
            .iter()
            .filter(|record| record.kind.kind_id
                == RelationKind::Topology(TopologyRelationKind::ModelOwnsBody).kind_id())
            .count(),
        1
    );
}

#[test]
fn authority_traced_commit_surfaces_schema_owned_trace_envelope() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();

    let traced = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(RawTopologyIntent::new(
            vec![TopologyMutation::CreateEntity {
                create_key: CreateKey::new("traced.model"),
                kind: EntityKind::Topology(TopologyEntityKind::Model),
            }],
            crate::data::authority::MutationOrigin::Seed,
        ))
        .expect("traced create batch should commit");

    assert_eq!(traced.primary_result().branch_id.0, "main");
    assert_eq!(
        traced
            .decision_trace()
            .authority
            .as_ref()
            .expect("authority trace evidence")
            .commit_count,
        1
    );
    assert_eq!(
        traced.integrity_markers().truth_basis_identity,
        Some(
            traced
                .primary_result()
                .read_basis
                .authority
                .truth_basis_identity
                .clone()
        )
    );
    assert!(traced
        .performance_accounting()
        .counters
        .iter()
        .any(|counter| counter.name == "authority.total_phase_count"));
}

#[test]
fn authority_can_publish_existing_topology_deletions_into_verified_commit_artifacts() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();
    let seeded = seed_minimal_topology(&mut runtime, "authority-delete").unwrap();

    let intent = RawTopologyIntent::new(
        vec![TopologyMutation::RemoveEntity {
            entity_id: seeded.vertex,
        }],
        crate::data::authority::MutationOrigin::LocalEdit,
    );

    let verified = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(intent)
        .expect("delete should commit")
        .into_primary_result();

    assert_eq!(verified.commits.len(), 1);
    let read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect("verified snapshot should remain readable");
    assert!(read.get_entity(seeded.vertex).is_none());
    assert!(
        verified
            .read_basis
            .touched_aspects()
            .contains(&Aspect::Topology(TopologyAspect::Structure))
            || verified
                .read_basis
                .touched_aspects()
                .contains(&Aspect::Topology(TopologyAspect::Boundary))
    );
}

#[test]
fn authority_can_publish_branch_local_topology_commits_on_a_real_branch() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();

    let seeded = seed_minimal_topology(&mut runtime, "authority-branch").unwrap();
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("feature branch should be creatable");

    let intent = RawTopologyIntent::new(
        vec![TopologyMutation::RemoveEntity {
            entity_id: seeded.vertex,
        }],
        crate::data::authority::MutationOrigin::BranchLocalApplication,
    );

    let verified = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_on_branch_traced(intent, BranchId("feature".to_string()))
        .expect("branch-local delete should commit")
        .into_primary_result();

    assert_eq!(verified.branch_id.0, "feature");
    assert_eq!(verified.persisted_truth.branch_id.0, "feature");
    assert_eq!(verified.read_basis.branch_id().0, "feature");
    let history = runtime.history();
    let feature_head = history
        .branch_head(&BranchId("feature".to_string()))
        .expect("feature branch head");
    let main_head = history
        .branch_head(&BranchId("main".to_string()))
        .expect("main branch head");
    assert_ne!(feature_head.commit_id, main_head.commit_id);
}

#[test]
fn authority_can_publish_mixed_create_and_existing_mutations_in_one_commit() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();

    let seeded = seed_minimal_topology(&mut runtime, "authority-mixed").unwrap();

    let intent = RawTopologyIntent::new(
        vec![
            TopologyMutation::UpsertEntity {
                entity_id: seeded.shell,
                kind: EntityKind::Topology(TopologyEntityKind::Shell),
            },
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new("authority-mixed.diagnostics.wire"),
                kind: EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation),
            },
        ],
        crate::data::authority::MutationOrigin::LocalEdit,
    );

    let verified = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(intent)
        .expect("mixed create and existing mutations should commit together")
        .into_primary_result();

    assert_eq!(verified.commits.len(), 1);
    let read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect("verified snapshot should remain readable");
    assert!(read.entities().iter().any(|record| {
        record.kind.kind_id
            == EntityKind::Diagnostics(DiagnosticsEntityKind::WireInterpretation).kind_id()
    }));
}

#[test]
fn authority_create_after_seed_preserves_existing_topology_and_names() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();

    let _seeded = seed_minimal_topology(&mut runtime, "authority-create-after-seed").unwrap();
    let intent = RawTopologyIntent::new(
        vec![
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new("authority-create-after-seed.added_vertex"),
                kind: EntityKind::Topology(TopologyEntityKind::Vertex),
            },
            TopologyMutation::CreateEntity {
                create_key: CreateKey::new(
                    "authority-create-after-seed.added_vertex.persistent_name",
                ),
                kind: EntityKind::Naming(crate::data::entities::NamingEntityKind::PersistentName),
            },
            TopologyMutation::CreateRelation {
                create_key: CreateKey::new(
                    "authority-create-after-seed.added_vertex.persistent_name.targets",
                ),
                kind: RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity),
                source: EntityReference::Created(CreateKey::new(
                    "authority-create-after-seed.added_vertex.persistent_name",
                )),
                target: EntityReference::Created(CreateKey::new(
                    "authority-create-after-seed.added_vertex",
                )),
            },
        ],
        crate::data::authority::MutationOrigin::LocalEdit,
    );

    let verified = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(intent)
        .expect("post-seed create should commit")
        .into_primary_result();

    let read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect("verified snapshot should remain readable");

    for label in [
        "authority-create-after-seed.model",
        "authority-create-after-seed.body",
        "authority-create-after-seed.vertex",
        "authority-create-after-seed.added_vertex",
    ] {
        assert!(read.entities().iter().any(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                .and_then(|kind| entity_record_label(record, kind))
                .is_some_and(|entity_label| entity_label == label)
        }));
    }

    let naming_targets = read
        .relations()
        .iter()
        .filter(|record| {
            record.kind.kind_id
                == RelationKind::Naming(NamingRelationKind::PersistentNameTargetsEntity).kind_id()
        })
        .count();
    assert_eq!(naming_targets, 12);
}

#[test]
fn authority_rejects_create_key_that_collides_with_live_entity_label() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_setup(|schema| {
            schema.schema_registry(crate::facade::bootstrap_schema_registry().unwrap());
        })
        .build();

    let _seeded = seed_minimal_topology(&mut runtime, "authority-live-label").unwrap();
    let error = TopologyAuthority::new(&mut runtime)
        .apply_topology_intent_traced(RawTopologyIntent::new(
            vec![TopologyMutation::CreateEntity {
                create_key: CreateKey::new("authority-live-label.vertex"),
                kind: EntityKind::Topology(TopologyEntityKind::Vertex),
            }],
            crate::data::authority::MutationOrigin::LocalEdit,
        ))
        .expect_err("duplicate live entity label should be rejected")
        .into_error();

    assert!(matches!(
        error,
        TopologyAuthorityError::DuplicateLiveEntityLabel(_)
    ));
}
