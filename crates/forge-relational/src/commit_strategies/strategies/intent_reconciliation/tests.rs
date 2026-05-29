use super::super::{
    IntentReconciliationAction, IntentReconciliationInput, IntentReconciliationOutput,
    IntentReconciliationStrategy,
};
use crate::commit_strategies::data::{StrategyCallerProvenance, StrategyRequestOrigin};
use crate::logic::builder::RelationalRuntimeBuilder;
use crate::tests::support::{
    create_entity, entity_field_aspect, entity_u64_field_aspect, lifecycle_aspect, update_entity,
    AspectSchemaFixture,
};
use crate::transactions::data::AspectFieldPatch;
use forge_foundational::facade::{AspectValue, FieldKey, InternedString};

fn strategy_registry() -> crate::schema::data::RelationalSchemaRegistry {
    AspectSchemaFixture {
        entity_aspects: vec![
            entity_field_aspect("name", "name"),
            entity_u64_field_aspect("replicas", "replicas"),
            lifecycle_aspect(),
        ],
        ..AspectSchemaFixture::default()
    }
    .build_registry()
}

#[test]
fn intent_reconciliation_strategy_emits_update_when_aspect_fields_differ() {
    let descriptor = IntentReconciliationStrategy::descriptor(
        crate::commit_strategies::data::CommitStrategyId(501),
    );
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_registry())
        .commit_strategy(
            crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                .expect("strategy registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build();
    let entity = create_entity(&mut runtime, "before");
    let request = runtime
        .commit_strategies()
        .canonicalize_request(
            &IntentReconciliationInput {
                entity_id: entity,
                desired_fields: AspectFieldPatch::single(
                    forge_foundational::facade::AspectKey::new("name")
                        .expect("valid test aspect key"),
                    FieldKey::new("name").expect("valid test field key"),
                    AspectValue::String(InternedString::Raw("after".to_string())),
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
    let output = IntentReconciliationOutput::decode(execution.output().canonical_bytes())
        .expect("output decode");

    assert_eq!(output.action, IntentReconciliationAction::UpdateEntity);
    assert_eq!(execution.mutation_program().total_intent_count(), 1);
}

#[test]
fn intent_reconciliation_strategy_emits_noop_when_aspect_fields_match() {
    let descriptor = IntentReconciliationStrategy::descriptor(
        crate::commit_strategies::data::CommitStrategyId(502),
    );
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_registry())
        .commit_strategy(
            crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                .expect("strategy registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build();
    let entity = create_entity(&mut runtime, "before");
    update_entity(&mut runtime, entity, "stable");
    let request = runtime
        .commit_strategies()
        .canonicalize_request(
            &IntentReconciliationInput {
                entity_id: entity,
                desired_fields: AspectFieldPatch::single(
                    forge_foundational::facade::AspectKey::new("name")
                        .expect("valid test aspect key"),
                    FieldKey::new("name").expect("valid test field key"),
                    AspectValue::String(InternedString::Raw("stable".to_string())),
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
    let output = IntentReconciliationOutput::decode(execution.output().canonical_bytes())
        .expect("output decode");

    assert_eq!(output.action, IntentReconciliationAction::NoChange);
    assert_eq!(execution.mutation_program().total_intent_count(), 0);
}

#[test]
fn intent_reconciliation_strategy_preserves_untouched_declared_fields() {
    let descriptor = IntentReconciliationStrategy::descriptor(
        crate::commit_strategies::data::CommitStrategyId(503),
    );
    let mut runtime = RelationalRuntimeBuilder::new()
        .schema_registry(strategy_registry())
        .commit_strategy(
            crate::commit_strategies::data::CommitStrategyRegistration::new(descriptor.clone())
                .expect("strategy registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build();
    let entity = create_entity(&mut runtime, "before");
    let request = runtime
        .commit_strategies()
        .canonicalize_request(
            &IntentReconciliationInput {
                entity_id: entity,
                desired_fields: AspectFieldPatch::single(
                    forge_foundational::facade::AspectKey::new("replicas")
                        .expect("valid test aspect key"),
                    FieldKey::new("replicas").expect("valid test field key"),
                    AspectValue::UInt64(3),
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
    let intent = &execution.mutation_program().worker_batches()[0].intents[0];
    let updated_aspect_value = match intent {
        crate::transactions::data::MutationIntent::Entity(
            crate::transactions::data::EntityMutationIntent::UpdateFields(intent),
        ) => intent.fields.get_single_field(
            &forge_foundational::facade::AspectKey::new("replicas").expect("valid test aspect key"),
            &FieldKey::new("replicas").expect("valid test field key"),
        ),
        other => panic!("expected update entity fields intent, got {other:?}"),
    };

    assert_eq!(updated_aspect_value, Some(&AspectValue::UInt64(3)));
}
