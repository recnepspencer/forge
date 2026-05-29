use super::filtering::relation_rule_kind;
use super::test_support::{
    create_entity, create_relation_of_kind, relation_integrity_runtime, request_for_plan,
};
use crate::identity::data::KindId;
use crate::identity::data::PartitionId;
use crate::transactions::data::{
    CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent, EntitySpec,
    MergedCommitPlan, MutationIntent, RelationMutationIntent, ReplaceEntityIntent, TransactionId,
};
use crate::transactions::facade::EntityReference;

#[test]
fn request_excludes_unrelated_relation_kind_registrations_for_relation_create() {
    let mut runtime = relation_integrity_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let plan = MergedCommitPlan {
        transaction_id: TransactionId(11),
        merged_intents: vec![MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("planned"),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(target),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            },
        ))],
    };

    let request = request_for_plan(&runtime, &plan);
    let included_relation_kinds = runtime
        .aspect_semantics
        .relation_integrity_registrations
        .iter()
        .filter(|registration| request.includes_registration(registration))
        .filter_map(|registration| relation_rule_kind(&registration.rule))
        .collect::<Vec<_>>();

    assert_eq!(included_relation_kinds, vec![KindId(2)]);
}

#[test]
fn request_excludes_unrelated_relation_kind_registrations_for_entity_delete() {
    let mut runtime = relation_integrity_runtime();
    let anchor = create_entity(&mut runtime, "anchor");
    let target = create_entity(&mut runtime, "target");
    let isolated_a = create_entity(&mut runtime, "isolated-a");
    let isolated_b = create_entity(&mut runtime, "isolated-b");
    create_relation_of_kind(&mut runtime, KindId(2), anchor, target, "adjacent-kind2");
    create_relation_of_kind(
        &mut runtime,
        KindId(3),
        isolated_a,
        isolated_b,
        "remote-kind3",
    );

    let plan = MergedCommitPlan {
        transaction_id: TransactionId(12),
        merged_intents: vec![MutationIntent::Entity(EntityMutationIntent::Delete(
            DeleteEntityIntent { entity_id: anchor },
        ))],
    };

    let request = request_for_plan(&runtime, &plan);
    let included_relation_kinds = runtime
        .aspect_semantics
        .relation_integrity_registrations
        .iter()
        .filter(|registration| request.includes_registration(registration))
        .filter_map(|registration| relation_rule_kind(&registration.rule))
        .collect::<Vec<_>>();

    assert_eq!(included_relation_kinds, vec![KindId(2)]);
}

#[test]
fn request_excludes_unrelated_relation_kind_registrations_for_entity_replace() {
    let mut runtime = relation_integrity_runtime();
    let anchor = create_entity(&mut runtime, "anchor");
    let target = create_entity(&mut runtime, "target");
    let isolated_a = create_entity(&mut runtime, "isolated-a");
    let isolated_b = create_entity(&mut runtime, "isolated-b");
    create_relation_of_kind(&mut runtime, KindId(2), anchor, target, "adjacent-kind2");
    create_relation_of_kind(
        &mut runtime,
        KindId(3),
        isolated_a,
        isolated_b,
        "remote-kind3",
    );

    let plan = MergedCommitPlan {
        transaction_id: TransactionId(13),
        merged_intents: vec![MutationIntent::Entity(EntityMutationIntent::Replace(
            ReplaceEntityIntent {
                entity_id: anchor,
                replacement: EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("replacement"),
                    fields: crate::transactions::data::AspectFieldPatch::default(),
                },
            },
        ))],
    };

    let request = request_for_plan(&runtime, &plan);
    let included_relation_kinds = runtime
        .aspect_semantics
        .relation_integrity_registrations
        .iter()
        .filter(|registration| request.includes_registration(registration))
        .filter_map(|registration| relation_rule_kind(&registration.rule))
        .collect::<Vec<_>>();

    assert_eq!(included_relation_kinds, vec![KindId(2)]);
}

#[test]
fn request_includes_deleted_relation_kind_scope_for_delete_only_commits() {
    let mut runtime = relation_integrity_runtime();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_id = create_relation_of_kind(&mut runtime, KindId(2), source, target, "edge");

    let plan = MergedCommitPlan {
        transaction_id: TransactionId(14),
        merged_intents: vec![MutationIntent::Relation(RelationMutationIntent::Delete(
            DeleteRelationIntent { relation_id },
        ))],
    };

    let request = request_for_plan(&runtime, &plan);
    let included_relation_kinds = runtime
        .aspect_semantics
        .relation_integrity_registrations
        .iter()
        .filter(|registration| request.includes_registration(registration))
        .filter_map(|registration| relation_rule_kind(&registration.rule))
        .collect::<Vec<_>>();

    assert_eq!(included_relation_kinds, vec![KindId(2)]);
}
