use super::*;
use crate::facade::indexes::{
    BoundedEntityFieldLookupDenialKind, BoundedEntityFieldLookupRequest, BoundedIndexParityMode,
    MAX_BOUNDED_INDEX_CANDIDATES,
};

#[test]
fn bounded_lookup_caps_ordinary_work_and_certifies_storage_parity() {
    let mut runtime = runtime_with_index_field_aspects();
    let entities = ["alpha", "beta", "gamma"]
        .into_iter()
        .map(|name| {
            let outcome = create_entity_outcome(&mut runtime, name);
            changed_entities(&outcome)[0]
        })
        .collect::<Vec<_>>();
    for entity in &entities {
        update_entity(&mut runtime, *entity, "shared");
    }
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(71),
        name: "entity.name.bounded".to_string(),
        kind: DerivedIndexKind::EntityField {
            field_locator: aspect_field_locator(aspect_key("name"), field_key("name")),
        },
        branch_scoped: false,
    });
    let source_commit_id = runtime.history().latest_commit().unwrap().commit_id;
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    assert!(build.failed_indexes.is_empty());
    let snapshot = runtime.visibility_authority().snapshot();
    let request = || {
        BoundedEntityFieldLookupRequest::new(
            snapshot.clone(),
            index.index_id,
            KindId(1),
            aspect_field_locator(aspect_key("name"), field_key("name")),
            string_aspect_value("shared"),
            2,
        )
        .unwrap()
    };

    let production = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request(), BoundedIndexParityMode::Production)
        .unwrap();
    let certified = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request(), BoundedIndexParityMode::Certification)
        .unwrap();

    assert_eq!(production.candidate_entity_ids().len(), 2);
    assert_eq!(production.examined_entry_count(), 2);
    assert!(production.overflowed());
    assert_eq!(
        production.candidate_entity_ids(),
        certified.candidate_entity_ids()
    );
    assert_eq!(production.overflowed(), certified.overflowed());
}

#[test]
fn bounded_lookup_rejects_unbounded_requests_and_nonexact_generations() {
    let mut runtime = runtime_with_index_field_aspects();
    let created = create_entity_outcome(&mut runtime, "alpha");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(72),
        name: "entity.name.exact-generation".to_string(),
        kind: DerivedIndexKind::EntityField {
            field_locator: aspect_field_locator(aspect_key("name"), field_key("name")),
        },
        branch_scoped: false,
    });
    runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: created.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    let oversized = BoundedEntityFieldLookupRequest::new(
        created.snapshot.clone(),
        index.index_id,
        KindId(1),
        aspect_field_locator(aspect_key("name"), field_key("name")),
        string_aspect_value("alpha"),
        MAX_BOUNDED_INDEX_CANDIDATES + 1,
    )
    .unwrap_err();
    assert_eq!(
        oversized.kind(),
        BoundedEntityFieldLookupDenialKind::InvalidCandidateLimit
    );

    create_entity_outcome(&mut runtime, "later");
    let later_snapshot = runtime.visibility_authority().snapshot();
    let request = BoundedEntityFieldLookupRequest::new(
        later_snapshot,
        index.index_id,
        KindId(1),
        aspect_field_locator(aspect_key("name"), field_key("name")),
        string_aspect_value("alpha"),
        2,
    )
    .unwrap();
    let denial = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        BoundedEntityFieldLookupDenialKind::ExactGenerationUnavailable
    );
}
