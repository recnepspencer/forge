use forge_relational::facade::config::{
    CascadeDeletePolicy, CrossContextPolicy, RelationalRuntimeProfile,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, KindId, PartitionId, RelationId};
use forge_relational::facade::merge::{
    IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope, MergeIntent,
    MergePlanningRequest, RelationalMergeInspectionArtifact,
};
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationIntegrityDeclarations,
    RelationKindRegistration, RelationPayloadClass, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::symbols::InternedString;
use forge_relational::facade::transactions::{
    CommitResult, CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent,
    EntityReference, EntitySpec, MutationIntent, RecordRef, RelationMutationIntent, RelationSpec,
    TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
};
use serde_json::json;

const TARGET_BRANCH: &str = "main";
const SOURCE_BRANCH: &str = "candidate";

pub fn source_addition_inspection_artifact() -> RelationalMergeInspectionArtifact {
    let mut runtime = runtime_with_default_merge_registry();
    create_entity_on_branch(&mut runtime, "root", BranchId(TARGET_BRANCH.to_string()));
    create_branch_from_main(&mut runtime, SOURCE_BRANCH);
    create_entity_on_branch(
        &mut runtime,
        "feature-only",
        BranchId(SOURCE_BRANCH.to_string()),
    );
    inspect_execution_surface(&runtime)
}

pub fn deleted_vs_modified_inspection_artifact() -> RelationalMergeInspectionArtifact {
    let mut runtime = runtime_with_default_merge_registry();
    let entity =
        create_entity_on_branch(&mut runtime, "shared", BranchId(TARGET_BRANCH.to_string()));
    create_branch_from_main(&mut runtime, SOURCE_BRANCH);
    update_entity_on_branch(
        &mut runtime,
        entity,
        "main-modified",
        BranchId(TARGET_BRANCH.to_string()),
    );
    delete_entity_on_branch(&mut runtime, entity, BranchId(SOURCE_BRANCH.to_string()));
    inspect_execution_surface(&runtime)
}

pub fn topology_region_conflict_inspection_artifact() -> RelationalMergeInspectionArtifact {
    let mut runtime = runtime_with_topology_merge_registry();
    let source =
        create_entity_on_branch(&mut runtime, "source", BranchId(TARGET_BRANCH.to_string()));
    let target_a = create_entity_on_branch(
        &mut runtime,
        "target-a",
        BranchId(TARGET_BRANCH.to_string()),
    );
    let target_b = create_entity_on_branch(
        &mut runtime,
        "target-b",
        BranchId(TARGET_BRANCH.to_string()),
    );
    let target_c = create_entity_on_branch(
        &mut runtime,
        "target-c",
        BranchId(TARGET_BRANCH.to_string()),
    );
    let target_d = create_entity_on_branch(
        &mut runtime,
        "target-d",
        BranchId(TARGET_BRANCH.to_string()),
    );
    let relation_a = create_relation_on_branch(
        &mut runtime,
        source,
        target_a,
        "edge-a",
        "edge-a",
        BranchId(TARGET_BRANCH.to_string()),
    );
    let relation_b = create_relation_on_branch(
        &mut runtime,
        source,
        target_b,
        "edge-b",
        "edge-b",
        BranchId(TARGET_BRANCH.to_string()),
    );

    create_branch_from_main(&mut runtime, SOURCE_BRANCH);
    delete_relation_on_branch(
        &mut runtime,
        relation_a,
        BranchId(SOURCE_BRANCH.to_string()),
    );
    delete_relation_on_branch(
        &mut runtime,
        relation_b,
        BranchId(SOURCE_BRANCH.to_string()),
    );
    create_relation_on_branch(
        &mut runtime,
        source,
        target_c,
        "edge-a",
        "edge-a",
        BranchId(SOURCE_BRANCH.to_string()),
    );
    create_relation_on_branch(
        &mut runtime,
        source,
        target_d,
        "edge-b",
        "edge-b",
        BranchId(SOURCE_BRANCH.to_string()),
    );

    inspect_execution_surface(&runtime)
}

fn inspect_execution_surface(runtime: &RelationalRuntime) -> RelationalMergeInspectionArtifact {
    runtime
        .merge()
        .inspect_execution_surface(MergePlanningRequest::new(
            BranchId(TARGET_BRANCH.to_string()),
            BranchId(SOURCE_BRANCH.to_string()),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("relational merge inspection should succeed")
}

fn runtime_with_default_merge_registry() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(default_merge_registry())
        .build()
}

fn runtime_with_topology_merge_registry() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(topology_merge_registry())
        .build()
}

fn default_merge_registry() -> RelationalSchemaRegistry {
    let name_key = aspect_key("name");
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![entity_payload_aspect(
                "name", "name",
            )])
            .with_identity_declarations(vec![IdentityBasisDeclaration {
                scope: IdentityBasisScope::AspectKey(name_key.clone()),
                basis: IdentityBasisKind::DeclaredKeySet(vec![name_key].into()),
            }]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .expect("default merge registry")
}

fn topology_merge_registry() -> RelationalSchemaRegistry {
    let label_key = aspect_key("label");
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::new(vec![
                    relation_payload_aspect("label", "label"),
                    relation_source_aspect(),
                    relation_target_aspect(),
                ])
                .with_identity_declarations(vec![IdentityBasisDeclaration {
                    scope: IdentityBasisScope::AspectKey(label_key.clone()),
                    basis: IdentityBasisKind::DeclaredKeySet(vec![label_key].into()),
                }]),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .expect("topology merge registry")
}

fn create_branch_from_main(runtime: &mut RelationalRuntime, branch_name: &str) {
    runtime
        .history_authority()
        .create_branch(
            BranchId(branch_name.to_string()),
            &BranchId(TARGET_BRANCH.to_string()),
        )
        .expect("branch creation should succeed");
}

fn create_entity_on_branch(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch_id: BranchId,
) -> EntityId {
    changed_entities(&commit_entity_create(runtime, name, branch_id))[0]
}

fn commit_entity_create(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch_id: BranchId,
) -> CommitResult {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("create-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw(name.to_string()),
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    txn.commit().expect("entity create should commit")
}

fn update_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: EntityId,
    name: &str,
    branch_id: BranchId,
) {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("update-entity").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id,
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    txn.commit().expect("entity update should commit");
}

fn delete_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: EntityId,
    branch_id: BranchId,
) {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("delete-entity").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id }),
        )),
    );
    txn.commit().expect("entity delete should commit");
}

fn create_relation_on_branch(
    runtime: &mut RelationalRuntime,
    source: EntityId,
    target: EntityId,
    client_key: &str,
    label: &str,
    branch_id: BranchId,
) -> RelationId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("create-relation").push(MutationIntent::Create(
            CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw(client_key.to_string()),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(target),
                payload: Some(RecordPayload::StructuredJson(json!({ "label": label }))),
            }),
        )),
    );
    changed_relations(&txn.commit().expect("relation create should commit"))[0]
}

fn delete_relation_on_branch(
    runtime: &mut RelationalRuntime,
    relation_id: RelationId,
    branch_id: BranchId,
) {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("delete-relation").push(MutationIntent::Relation(
            RelationMutationIntent::Delete(DeleteRelationIntent { relation_id }),
        )),
    );
    txn.commit().expect("relation delete should commit");
}

fn changed_entities(outcome: &CommitResult) -> Vec<EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            RecordRef::Entity(entity_id) => Some(*entity_id),
            RecordRef::Relation(_) => None,
        })
        .collect()
}

fn changed_relations(outcome: &CommitResult) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            RecordRef::Relation(relation_id) => Some(*relation_id),
            RecordRef::Entity(_) => None,
        })
        .collect()
}

fn aspect_key(name: &str) -> AspectKey {
    AspectKey(InternedString::Raw(name.to_string()))
}

fn entity_payload_aspect(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key(name),
        binding: AspectBinding::EntityPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

fn relation_payload_aspect(name: &str, field: &str) -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key(name),
        binding: AspectBinding::RelationPayloadField {
            field: InternedString::Raw(field.to_string()),
        },
        comparator: AspectComparator::JsonScalarEquality,
        precision: AspectPrecision::Structured,
    }
}

fn relation_source_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("source"),
        binding: AspectBinding::RelationSourceEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}

fn relation_target_aspect() -> DeclaredAspect {
    DeclaredAspect {
        key: aspect_key("target"),
        binding: AspectBinding::RelationTargetEndpoint,
        comparator: AspectComparator::EndpointIdentityEquality,
        precision: AspectPrecision::Structured,
    }
}
