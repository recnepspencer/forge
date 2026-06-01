use crate::facade::identity::EntityId;
use crate::tests::support::*;
use forge_foundational::facade::AspectKey;

#[test]
fn visibility_aspect_versions_follow_canonical_delta_truth_and_ignore_undeclared_fields() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "alpha");
    let entity = changed_entities(&created)[0];
    let updated = update_entity(&mut runtime, entity, "beta");
    let versions = runtime.read_truth().entity_aspect_versions(entity).unwrap();

    assert_eq!(
        versions
            .iter()
            .map(|(aspect, _)| aspect.clone())
            .collect::<Vec<_>>(),
        vec![
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("name").unwrap(),
        ]
    );
    assert_eq!(
        versions,
        vec![
            (AspectKey::new("lifecycle").unwrap(), created.version_id.0),
            (AspectKey::new("name").unwrap(), updated.version_id.0),
        ]
    );

    let relation = create_relation(&mut runtime, entity, entity, "edge");
    let relation_versions = runtime
        .read_truth()
        .relation_aspect_versions(relation)
        .unwrap();
    assert_eq!(
        relation_versions
            .iter()
            .map(|(aspect, _)| aspect.clone())
            .collect::<Vec<_>>(),
        vec![
            AspectKey::new("label").unwrap(),
            AspectKey::new("lifecycle").unwrap(),
            AspectKey::new("source").unwrap(),
            AspectKey::new("target").unwrap(),
        ]
    );
}

#[test]
fn visibility_aspect_versions_reject_stale_generation_ids() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let entity = create_entity(&mut runtime, "before");
    let stale = EntityId::new(
        entity.partition_id,
        entity.local_slot.0,
        entity.generation.0 + 1,
    );

    assert!(runtime.read_truth().entity_aspect_versions(stale).is_none());
    assert!(runtime
        .read_truth()
        .entity_aspect_versions(entity)
        .is_some());
}
