use super::*;
use crate::facade::indexes::{
    BoundedIndexParityMode, BoundedRelationJoinLookupDenialKind, BoundedRelationJoinLookupRequest,
    DerivedIndexEntries, RelationJoinDefinition, RelationJoinEntry, RelationJoinKey,
    RelationJoinLeg, RelationJoinSharedEndpoint,
};

#[test]
fn relation_join_isolates_both_exact_endpoints_and_certifies_storage_parity() {
    let mut runtime = runtime_with_index_field_aspects();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let other_left = create_entity(&mut runtime, "other-left");
    let other_right = create_entity(&mut runtime, "other-right");
    let selected = create_entity(&mut runtime, "selected");
    let same_left = create_entity(&mut runtime, "same-left");
    let same_right = create_entity(&mut runtime, "same-right");
    bind_chain(&mut runtime, left, selected, right, "selected");
    bind_chain(&mut runtime, left, same_left, other_right, "same-left");
    bind_chain(&mut runtime, other_left, same_right, right, "same-right");
    let index = register_relation_join(&mut runtime, 81);
    build_current_generation(&mut runtime, index.index_id);
    let snapshot = runtime.visibility_authority().snapshot();
    let request = || join_request(snapshot.clone(), index.index_id, left, right, 2);

    let production = runtime
        .index_access()
        .execute_bounded_relation_join_lookup(request(), BoundedIndexParityMode::Production)
        .unwrap();
    let certified = runtime
        .index_access()
        .execute_bounded_relation_join_lookup(request(), BoundedIndexParityMode::Certification)
        .unwrap();

    assert_eq!(production.candidate_entity_ids(), &[selected]);
    assert_eq!(production.examined_entry_count(), 1);
    assert!(!production.overflowed());
    assert_eq!(
        production.candidate_entity_ids(),
        certified.candidate_entity_ids()
    );
    assert_eq!(production.overflowed(), certified.overflowed());
}

#[test]
fn relation_join_requires_an_exact_generation_and_reports_bounded_overflow() {
    let mut runtime = runtime_with_index_field_aspects();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let first = create_entity(&mut runtime, "first");
    bind_chain(&mut runtime, left, first, right, "first");
    let index = register_relation_join(&mut runtime, 82);
    build_current_generation(&mut runtime, index.index_id);

    let second = create_entity(&mut runtime, "second");
    bind_chain(&mut runtime, left, second, right, "second");
    let current = runtime.visibility_authority().snapshot();
    let missing = runtime
        .index_access()
        .execute_bounded_relation_join_lookup(
            join_request(current.clone(), index.index_id, left, right, 1),
            BoundedIndexParityMode::Production,
        )
        .unwrap_err();
    assert_eq!(
        missing.kind(),
        BoundedRelationJoinLookupDenialKind::ExactGenerationUnavailable
    );

    build_current_generation(&mut runtime, index.index_id);
    let bounded = runtime
        .index_access()
        .execute_bounded_relation_join_lookup(
            join_request(current, index.index_id, left, right, 1),
            BoundedIndexParityMode::Certification,
        )
        .unwrap();
    assert_eq!(bounded.candidate_entity_ids(), &[first]);
    assert_eq!(bounded.examined_entry_count(), 1);
    assert!(bounded.overflowed());
}

#[test]
fn relation_join_rejects_a_candidate_with_substituted_relation_evidence() {
    let mut runtime = runtime_with_index_field_aspects();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let selected = create_entity(&mut runtime, "selected");
    let other_left = create_entity(&mut runtime, "other-left");
    let other_right = create_entity(&mut runtime, "other-right");
    let other_shared = create_entity(&mut runtime, "other-shared");
    let (selected_left_relation, _) = bind_chain(&mut runtime, left, selected, right, "selected");
    let (_, substituted_right_relation) =
        bind_chain(&mut runtime, other_left, other_shared, other_right, "other");
    let index = register_relation_join(&mut runtime, 83);
    build_current_generation(&mut runtime, index.index_id);
    let key = RelationJoinKey::new(left, right);
    let generation = runtime
        .indexes
        .generations
        .get_mut(&index.index_id)
        .and_then(|generations| generations.last_mut())
        .unwrap();
    let DerivedIndexEntries::RelationJoin(entries) = &mut generation.entries else {
        panic!("relation join generation expected");
    };
    entries.get_mut(&key).unwrap()[0] =
        RelationJoinEntry::new(selected, selected_left_relation, substituted_right_relation);
    let snapshot = runtime.visibility_authority().snapshot();

    let denial = runtime
        .index_access()
        .execute_bounded_relation_join_lookup(
            join_request(snapshot, index.index_id, left, right, 2),
            BoundedIndexParityMode::Production,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        BoundedRelationJoinLookupDenialKind::CorruptIndexEntries
    );
}

fn register_relation_join(runtime: &mut RelationalRuntime, id: u64) -> DerivedIndexDefinition {
    runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(id),
        name: format!("relation.join.{id}"),
        kind: DerivedIndexKind::RelationJoin(RelationJoinDefinition::new(
            RelationJoinLeg::new(KindId(2), RelationJoinSharedEndpoint::Target, KindId(1)),
            RelationJoinLeg::new(KindId(2), RelationJoinSharedEndpoint::Source, KindId(1)),
            KindId(1),
        )),
        branch_scoped: false,
    })
}

fn build_current_generation(runtime: &mut RelationalRuntime, index_id: DerivedIndexId) {
    let source_commit_id = runtime.history().latest_commit().unwrap().commit_id;
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index_id],
        });
    assert!(build.failed_indexes.is_empty());
}

fn bind_chain(
    runtime: &mut RelationalRuntime,
    left: crate::facade::identity::EntityId,
    shared: crate::facade::identity::EntityId,
    right: crate::facade::identity::EntityId,
    name: &str,
) -> (
    crate::facade::identity::RelationId,
    crate::facade::identity::RelationId,
) {
    let left_relation = create_relation(runtime, left, shared, &format!("{name}-left"));
    let right_relation = create_relation(runtime, shared, right, &format!("{name}-right"));
    (left_relation, right_relation)
}

fn join_request(
    snapshot: crate::facade::snapshots::SnapshotHandle,
    index_id: DerivedIndexId,
    left: crate::facade::identity::EntityId,
    right: crate::facade::identity::EntityId,
    limit: usize,
) -> BoundedRelationJoinLookupRequest {
    BoundedRelationJoinLookupRequest::new(snapshot, index_id, left, right, limit).unwrap()
}
