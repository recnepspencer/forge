use super::custom_rules::*;
use super::validation_engine_fixtures::*;

#[test]
fn engine_executes_custom_invariant_packets() {
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .custom_invariant(CustomInvariantRegistration::new(AlwaysViolatesCustomRule).unwrap())
        .build();

    let results = InvariantEngine::new(&runtime).execute(
        InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            &runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()),
            runtime.current_version_id(),
            None,
            None,
        ),
    );

    assert_eq!(results.results().len(), 1);
    match &results.results()[0].rule {
        InvariantReportedRule::Custom(identity) => {
            assert_eq!(identity.rule_id.as_str(), "test.custom.violation");
        }
        other => panic!("expected custom invariant result, got {other:?}"),
    }
    assert!(matches!(
        results.results()[0].verdict,
        crate::validation::data::InvariantVerdict::Violation(_)
    ));
}

#[test]
fn engine_executes_custom_packets_against_real_structural_surfaces() {
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .custom_invariant(CustomInvariantRegistration::new(StructuralSurfaceRule).unwrap())
        .build();
    runtime.performance_access().reset_counters();
    let plan = MergedCommitPlan {
        transaction_id: TransactionId(3),
        merged_intents: vec![
            MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: crate::facade::identity::KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("source"),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            }))
            .into(),
            MutationIntent::Create(CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: crate::facade::identity::KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("edge"),
                source: crate::transactions::data::EntityReference::Existing(
                    crate::facade::identity::EntityId::new(PartitionId::main(), 10, 1),
                ),
                target: crate::transactions::data::EntityReference::Existing(
                    crate::facade::identity::EntityId::new(PartitionId::main(), 11, 1),
                ),
                fields: crate::transactions::data::AspectFieldPatch::default(),
            }))
            .into(),
        ],
    };

    let results = InvariantEngine::new(&runtime).execute(
        InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            &runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()).into(),
            runtime.current_version_id(),
            Some(&plan),
            None,
        ),
    );

    assert_eq!(results.results().len(), 1);
    assert!(matches!(
        results.results()[0].verdict,
        crate::validation::data::InvariantVerdict::Pass
    ));
    let counters = runtime.performance_access().counters();
    assert_eq!(counters.custom_invariant_preparation_count, 1);
    assert_eq!(counters.custom_invariant_execution_count, 1);
    assert!(counters.custom_invariant_traversal_frontier_count >= 2);
}

#[test]
fn engine_captures_custom_prepare_panics_as_typed_failures() {
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .custom_invariant(CustomInvariantRegistration::new(PanicDuringPrepareRule).unwrap())
        .build();
    runtime.performance_access().reset_counters();

    let results = InvariantEngine::new(&runtime).execute(
        InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            &runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()),
            runtime.current_version_id(),
            None,
            None,
        ),
    );

    assert_eq!(results.results().len(), 1);
    let crate::validation::data::InvariantVerdict::Violation(violation) =
        &results.results()[0].verdict
    else {
        panic!("expected captured prepare panic to produce a violation");
    };
    match &violation.fields {
        crate::validation::data::InvariantViolationFields::CustomInvariantFailure {
            identity,
            phase,
            failure,
            ..
        } => {
            assert_eq!(
                identity.semantic_identity().rule_id.as_str(),
                "test.custom.panic-prepare"
            );
            assert_eq!(
                phase,
                &crate::validation::data::CustomInvariantFailurePhase::Preparation
            );
            assert_eq!(
                failure,
                &crate::validation::data::ResultCustomInvariantFailureKind::Panic
            );
        }
        other => panic!("expected custom invariant failure fields, got {other:?}"),
    }
    assert_eq!(results.summary().custom_failure_count(), 1);
    assert_eq!(results.summary().custom_panic_count(), 1);
    let counters = runtime.performance_access().counters();
    assert_eq!(counters.custom_invariant_preparation_count, 1);
    assert_eq!(counters.custom_invariant_execution_count, 0);
    assert_eq!(counters.custom_invariant_panic_count, 1);
}

#[test]
fn engine_captures_custom_evaluate_panics_as_typed_failures() {
    let runtime = RelationalRuntimeApi::builder()
        .schema_registry(RelationalSchemaRegistry::new())
        .custom_invariant(CustomInvariantRegistration::new(PanicDuringEvaluateRule).unwrap())
        .build();
    runtime.performance_access().reset_counters();

    let results = InvariantEngine::new(&runtime).execute(
        InvariantExecutionRequest::from_profile_with_contract(
            InvariantRequestProfile::CommitBoundary,
            &runtime,
            InvariantObservation::committed(runtime.storage_access().current_state()),
            runtime.current_version_id(),
            None,
            None,
        ),
    );

    assert_eq!(results.results().len(), 1);
    let crate::validation::data::InvariantVerdict::Violation(violation) =
        &results.results()[0].verdict
    else {
        panic!("expected captured evaluate panic to produce a violation");
    };
    match &violation.fields {
        crate::validation::data::InvariantViolationFields::CustomInvariantFailure {
            identity,
            phase,
            failure,
            ..
        } => {
            assert_eq!(
                identity.semantic_identity().rule_id.as_str(),
                "test.custom.panic-evaluate"
            );
            assert_eq!(
                phase,
                &crate::validation::data::CustomInvariantFailurePhase::Execution
            );
            assert_eq!(
                failure,
                &crate::validation::data::ResultCustomInvariantFailureKind::Panic
            );
        }
        other => panic!("expected custom invariant failure fields, got {other:?}"),
    }
    assert_eq!(results.summary().custom_failure_count(), 1);
    assert_eq!(results.summary().custom_panic_count(), 1);
    let counters = runtime.performance_access().counters();
    assert_eq!(counters.custom_invariant_preparation_count, 1);
    assert_eq!(counters.custom_invariant_execution_count, 1);
    assert_eq!(counters.custom_invariant_panic_count, 1);
}
