use worth_relational::facade::config::{
    CascadeDeletePolicy, CrossContextPolicy, RelationalRuntimeProfile,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::identity::{EntityId, KindId, PartitionId, RelationId};
use worth_relational::facade::merge::{
    IdentityBasisDeclaration, IdentityBasisKind, IdentityBasisScope, MergeIntent,
    MergePlanningRequest, RelationalMergeInspectionArtifact,
};
use worth_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use worth_relational::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationIntegrityDeclarations,
    RelationKindRegistration, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    CommitResult, CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent,
    EntityReference, EntitySpec, MutationIntent, RecordRef, RelationMutationIntent, RelationSpec,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use crate::aspect_field_authoring::single_native_string_aspect_field_patch;

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
    let request = runtime
        .bind_merge_planning_request(MergePlanningRequest::new(
            BranchId(TARGET_BRANCH.to_string()),
            BranchId(SOURCE_BRANCH.to_string()),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("merge inspection branches should be owner-bound");
    runtime
        .merge()
        .inspect_execution_surface(request)
        .expect("relational merge inspection should succeed")
}

fn runtime_with_default_merge_registry() -> RelationalRuntime {
    let registry = default_merge_registry();
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(registry)
        .build()
}

fn runtime_with_topology_merge_registry() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(topology_merge_registry())
        .build()
}

fn default_merge_registry() -> RelationalSchemaRegistry {
    let name_key = crate::aspect_field_authoring::aspect_key("name").expect("valid aspect key");
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                crate::aspect_field_authoring::entity_string_field_aspect("name", "name")
                    .expect("entity field aspect"),
            ])
            .with_identity_declarations(vec![
                IdentityBasisDeclaration {
                    scope: IdentityBasisScope::EntityKind(KindId(1)),
                    basis: IdentityBasisKind::StorageIdentity,
                },
                IdentityBasisDeclaration {
                    scope: IdentityBasisScope::EntityKind(KindId(1)),
                    basis: IdentityBasisKind::LineageIdentity,
                },
                IdentityBasisDeclaration {
                    scope: IdentityBasisScope::AspectKey(name_key.clone()),
                    basis: IdentityBasisKind::DeclaredKeySet(vec![name_key].into()),
                },
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_contract_declarations: KindAspectContractDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .expect("default merge registry")
}

fn topology_merge_registry() -> RelationalSchemaRegistry {
    let label_key = crate::aspect_field_authoring::aspect_key("label").expect("valid aspect key");
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                crate::aspect_field_authoring::entity_string_field_aspect("name", "name")
                    .expect("entity field aspect"),
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                    crate::aspect_field_authoring::relation_string_field_aspect("label", "label")
                        .expect("relation field aspect"),
                    crate::aspect_field_authoring::relation_source_endpoint_aspect("source")
                        .expect("relation source aspect"),
                    crate::aspect_field_authoring::relation_target_endpoint_aspect("target")
                        .expect("relation target aspect"),
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
    crate::runtime::fork_branch_from_exact_source(
        runtime,
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
    let mut txn = begin_branch_transaction(runtime, &branch_id);
    txn.push_batch(
        WorkerIntentBatch::new(format!("create-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: ClientKey::raw(name),
                fields: single_native_string_aspect_field_patch("name", "name", name)
                    .expect("entity name aspect patch"),
            }),
        )),
    );
    txn.commit(runtime).expect("entity create should commit")
}

fn update_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: EntityId,
    name: &str,
    branch_id: BranchId,
) {
    let mut txn = begin_branch_transaction(runtime, &branch_id);
    txn.push_batch(
        WorkerIntentBatch::new("update-entity").push(MutationIntent::Entity(
            EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                entity_id,
                fields: single_native_string_aspect_field_patch("name", "name", name)
                    .expect("entity name aspect patch"),
            }),
        )),
    );
    txn.commit(runtime).expect("entity update should commit");
}

fn delete_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: EntityId,
    branch_id: BranchId,
) {
    let mut txn = begin_branch_transaction(runtime, &branch_id);
    txn.push_batch(
        WorkerIntentBatch::new("delete-entity").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id }),
        )),
    );
    txn.commit(runtime).expect("entity delete should commit");
}

fn create_relation_on_branch(
    runtime: &mut RelationalRuntime,
    source: EntityId,
    target: EntityId,
    client_key: &str,
    label: &str,
    branch_id: BranchId,
) -> RelationId {
    let mut txn = begin_branch_transaction(runtime, &branch_id);
    txn.push_batch(
        WorkerIntentBatch::new("create-relation").push(MutationIntent::Create(
            CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: ClientKey::raw(client_key),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(target),
                fields: single_native_string_aspect_field_patch("label", "label", label)
                    .expect("relation label aspect patch"),
            }),
        )),
    );
    changed_relations(&txn.commit(runtime).expect("relation create should commit"))[0]
}

fn delete_relation_on_branch(
    runtime: &mut RelationalRuntime,
    relation_id: RelationId,
    branch_id: BranchId,
) {
    let mut txn = begin_branch_transaction(runtime, &branch_id);
    txn.push_batch(
        WorkerIntentBatch::new("delete-relation").push(MutationIntent::Relation(
            RelationMutationIntent::Delete(DeleteRelationIntent { relation_id }),
        )),
    );
    txn.commit(runtime).expect("relation delete should commit");
}

fn begin_branch_transaction(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
) -> worth_relational::facade::mvcc::BranchBoundRelationalTransaction {
    let context = runtime
        .admit_named_branch_basis(branch_id)
        .expect("branch context");
    runtime
        .begin_branch_transaction(
            &context,
            worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
        )
        .expect("owner-admitted branch basis")
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
