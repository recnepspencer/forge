use super::*;

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
