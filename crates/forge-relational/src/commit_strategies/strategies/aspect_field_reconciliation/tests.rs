use super::{
    AspectFieldReconciliationInput, AspectFieldReconciliationOutput,
    AspectFieldReconciliationStrategy,
};
use crate::commit_strategies::data::{
    StrategyCallerProvenance, StrategyExecutorFailureClass, StrategyRequestOrigin,
};
use crate::config::data::CascadeDeletePolicy;
use crate::logic::builder::RelationalRuntimeBuilder;
use crate::tests::support::{
    create_entity, entity_field_aspect, entity_u64_field_aspect, lifecycle_aspect,
    AspectSchemaFixture,
};
use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, CanonicalFieldPath, FieldKey, LocatorAuthority,
};

fn strategy_runtime(
    descriptor: crate::commit_strategies::data::CommitStrategyDescriptor,
    registry: crate::schema::data::RelationalSchemaRegistry,
) -> crate::logic::runtime::RelationalRuntime {
    RelationalRuntimeBuilder::new()
        .schema_registry(registry)
        .commit_strategy(
            crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                .expect("strategy registration"),
        )
        .commit_strategy_executor(AspectFieldReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build()
}

fn registry_with_replicas_field() -> crate::schema::data::RelationalSchemaRegistry {
    AspectSchemaFixture {
        cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            entity_u64_field_aspect(
                crate::tests::support::aspect_key("replicas"),
                crate::tests::support::field_key("replicas"),
            ),
            lifecycle_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry()
}

fn field_locator(aspect_key: &str, field_key: &str) -> AspectFieldLocator {
    AspectFieldLocator::new(
        LocatorAuthority::Planned,
        AspectKey::new(aspect_key).expect("valid test aspect key"),
        CanonicalFieldPath::single(FieldKey::new(field_key).expect("valid test field key")),
    )
}

#[test]
fn aspect_field_reconciliation_strategy_updates_only_declared_field_aspect() {
    let descriptor = AspectFieldReconciliationStrategy::descriptor(
        crate::commit_strategies::data::CommitStrategyId(701),
    );
    let mut runtime = strategy_runtime(descriptor, registry_with_replicas_field());
    let entity = create_entity(&mut runtime, "before");
    crate::tests::support::update_entity(&mut runtime, entity, "before");
    let request = runtime
        .commit_strategies()
        .canonicalize_request(
            &AspectFieldReconciliationInput {
                entity_id: entity,
                field_locator: field_locator("replicas", "replicas"),
                desired_value: forge_foundational::facade::AspectValue::UInt64(5),
            }
            .into_raw_request(StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            })
            .expect("raw strategy request"),
        )
        .expect("canonical request");
    let snapshot = runtime.visibility_authority().snapshot();
    let execution = runtime
        .commit_strategies()
        .execute(&request, &snapshot)
        .expect("strategy execution");
    let output = AspectFieldReconciliationOutput::decode(execution.output().canonical_bytes())
        .expect("output decode");
    let updated_replicas = match &execution.mutation_program().worker_batches()[0].intents[0] {
        crate::transactions::data::MutationIntent::Entity(
            crate::transactions::data::EntityMutationIntent::UpdateFields(intent),
        ) => intent.fields.get_single_field(
            &forge_foundational::facade::AspectKey::new("replicas").expect("valid test aspect key"),
            &forge_foundational::facade::FieldKey::new("replicas").expect("valid test field key"),
        ),
        other => panic!("expected update entity fields intent, got {other:?}"),
    };

    assert!(output.updated);
    assert_eq!(output.field_locator, field_locator("replicas", "replicas"));
    assert_eq!(
        updated_replicas,
        Some(&forge_foundational::facade::AspectValue::UInt64(5))
    );
}

#[test]
fn aspect_field_reconciliation_strategy_noops_when_authoritative_field_matches() {
    let descriptor = AspectFieldReconciliationStrategy::descriptor(
        crate::commit_strategies::data::CommitStrategyId(703),
    );
    let mut runtime = strategy_runtime(descriptor, registry_with_replicas_field());
    let entity = create_entity(&mut runtime, "stable");
    let request = runtime
        .commit_strategies()
        .canonicalize_request(
            &AspectFieldReconciliationInput {
                entity_id: entity,
                field_locator: field_locator("name", "name"),
                desired_value: forge_foundational::facade::AspectValue::String(
                    forge_foundational::facade::InternedString::Raw("stable".to_string()),
                ),
            }
            .into_raw_request(StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            })
            .expect("raw strategy request"),
        )
        .expect("canonical request");
    let snapshot = runtime.visibility_authority().snapshot();
    let execution = runtime
        .commit_strategies()
        .execute(&request, &snapshot)
        .expect("strategy execution");
    let output = AspectFieldReconciliationOutput::decode(execution.output().canonical_bytes())
        .expect("output decode");

    assert!(!output.updated);
    assert_eq!(execution.mutation_program().total_intent_count(), 0);
}

#[test]
fn aspect_field_reconciliation_strategy_rejects_undeclared_field() {
    let descriptor = AspectFieldReconciliationStrategy::descriptor(
        crate::commit_strategies::data::CommitStrategyId(702),
    );
    let registry = AspectSchemaFixture {
        cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
        entity_aspects: vec![
            entity_field_aspect(
                crate::tests::support::aspect_key("name"),
                crate::tests::support::field_key("name"),
            ),
            lifecycle_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry();
    let mut runtime = strategy_runtime(descriptor, registry);
    let entity = create_entity(&mut runtime, "before");
    let request = runtime
        .commit_strategies()
        .canonicalize_request(
            &AspectFieldReconciliationInput {
                entity_id: entity,
                field_locator: field_locator("replicas", "replicas"),
                desired_value: forge_foundational::facade::AspectValue::UInt64(5),
            }
            .into_raw_request(StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            })
            .expect("raw strategy request"),
        )
        .expect("canonical request");
    let snapshot = runtime.visibility_authority().snapshot();
    let error = runtime
        .commit_strategies()
        .execute(&request, &snapshot)
        .expect_err("undeclared aspect field should be rejected");

    match error {
        crate::commit_strategies::StrategyExecutionError::ExecutorFailed { failure, .. } => {
            assert_eq!(failure.class, StrategyExecutorFailureClass::DomainRejection);
            assert!(failure
                .detail
                .contains("aspect field locator 'replicas:replicas'"));
        }
        other => panic!("expected executor failure, got {other:?}"),
    }
}
