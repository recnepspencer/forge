use crate::facade::config::{CascadeDeletePolicy, CrossContextPolicy};
use crate::facade::identity::KindId;
use crate::facade::runtime::{RelationalInitialSchemaInstallationDenialKind, RelationalRuntimeApi};
use crate::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationIntegrityDeclarations,
    RelationKindRegistration, RelationalSchemaRegistry, SchemaId, SchemaRegistryErrorClass,
    SchemaVersionId,
};
use crate::facade::transactions::{CreateIntent, EntitySpec, MutationIntent, WorkerIntentBatch};
use crate::facade::{identity::PartitionId, symbols::ClientKey};

#[test]
fn initial_schema_installation_retains_existing_kinds_and_rejects_duplicates() {
    let initial = RelationalSchemaRegistry::new()
        .register_entity_kind(entity(KindId(4), "existing"))
        .unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(initial)
        .build();
    let receipt = runtime
        .prepare_initial_schema_installation()
        .unwrap()
        .install(
            RelationalSchemaRegistry::new()
                .register_entity_kind(entity(KindId(8), "added"))
                .unwrap()
                .register_relation_kind(relation(KindId(9), "added-relation"))
                .unwrap(),
        )
        .unwrap();
    assert_eq!(receipt.retained_entity_kind_count(), 2);
    assert_eq!(receipt.retained_relation_kind_count(), 1);
    assert!(runtime
        .config()
        .schema
        .registry
        .entity_kinds
        .contains_key(&KindId(4)));
    assert!(runtime
        .config()
        .schema
        .registry
        .entity_kinds
        .contains_key(&KindId(8)));

    let duplicate = runtime
        .prepare_initial_schema_installation()
        .unwrap()
        .install(
            RelationalSchemaRegistry::new()
                .register_entity_kind(entity(KindId(8), "replacement"))
                .unwrap(),
        )
        .unwrap_err();
    assert_eq!(
        duplicate.kind(),
        RelationalInitialSchemaInstallationDenialKind::SchemaRejected
    );
}

#[test]
fn initial_schema_authority_closes_after_first_commit() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            RelationalSchemaRegistry::new()
                .register_entity_kind(entity(KindId(1), "committed"))
                .unwrap(),
        )
        .build();
    let mut transaction =
        crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    transaction.push_batch(WorkerIntentBatch::new("close-schema-installation").push(
        MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: ClientKey::raw("first"),
            fields: Default::default(),
        })),
    ));
    transaction.commit().unwrap();

    let denial = runtime.prepare_initial_schema_installation().unwrap_err();
    assert_eq!(
        denial.kind(),
        RelationalInitialSchemaInstallationDenialKind::RuntimeAlreadyCommitted
    );
}

#[test]
fn registry_registration_never_overwrites_same_family_authority() {
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(entity(KindId(3), "first"))
        .unwrap();
    let denial = registry
        .register_entity_kind(entity(KindId(3), "second"))
        .unwrap_err();
    assert_eq!(
        denial.class,
        SchemaRegistryErrorClass::DuplicateEntityKind(KindId(3))
    );
}

fn entity(kind_id: KindId, name: &str) -> EntityKindRegistration {
    EntityKindRegistration {
        kind_id,
        kind_name: name.to_string(),
        schema_id: SchemaId("initial-installation".to_string()),
        schema_version_id: SchemaVersionId(1),
        aspect_contract_declarations: KindAspectContractDeclarations::default(),
    }
}

fn relation(kind_id: KindId, name: &str) -> RelationKindRegistration {
    RelationKindRegistration {
        kind_id,
        kind_name: name.to_string(),
        schema_id: SchemaId("initial-installation".to_string()),
        schema_version_id: SchemaVersionId(1),
        cross_context_policy: CrossContextPolicy::Forbid,
        cascade_delete_policy: CascadeDeletePolicy::RetainDanglingForAudit,
        aspect_contract_declarations: KindAspectContractDeclarations::default(),
        relation_integrity: RelationIntegrityDeclarations::default(),
    }
}
