use crate::commit_strategies::data::{
    CanonicalStrategyCommitRequest, LoweredStrategyCommitPlan, StrategyExecutionDraft,
    StrategyLoweringError, StrategyLoweringProvenance, StrategyLoweringSummary,
};
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::TransactionOptions;

pub(crate) fn lower_execution(
    runtime: &mut RelationalRuntime,
    request: &CanonicalStrategyCommitRequest,
    execution: &StrategyExecutionDraft,
    options: TransactionOptions,
) -> Result<LoweredStrategyCommitPlan, StrategyLoweringError> {
    validate_execution_binding(request, execution)?;

    let mut transaction = runtime.begin_transaction(options.clone());
    for worker_batch in execution
        .mutation_program()
        .worker_batches()
        .iter()
        .cloned()
    {
        transaction.push_batch(worker_batch);
    }

    let transaction_id = transaction.transaction_id();
    let bulk_mutation_batch = transaction
        .admit_provenance_complete_bulk_mutation_batch()
        .map_err(StrategyLoweringError::mutation_conflict)?;
    let merged_plan = transaction
        .merged_plan()
        .map_err(StrategyLoweringError::mutation_conflict)?
        .clone();
    let lowering_provenance =
        StrategyLoweringProvenance::from_request_and_execution(request, execution);
    let lowering_summary = build_lowering_summary(execution, bulk_mutation_batch.as_ref());

    Ok(LoweredStrategyCommitPlan::new(
        request.clone(),
        execution.clone(),
        transaction_id,
        options,
        bulk_mutation_batch,
        merged_plan,
        lowering_provenance,
        lowering_summary,
    ))
}

fn validate_execution_binding(
    request: &CanonicalStrategyCommitRequest,
    execution: &StrategyExecutionDraft,
) -> Result<(), StrategyLoweringError> {
    let binding = execution.request_binding();
    if binding.strategy_id() != request.strategy_id() {
        return Err(StrategyLoweringError::RequestExecutionMismatch {
            detail: format!(
                "execution draft strategy id {} does not match request strategy id {}",
                binding.strategy_id().0,
                request.strategy_id().0
            ),
        });
    }
    if binding.descriptor_digest() != request.descriptor_digest() {
        return Err(StrategyLoweringError::RequestExecutionMismatch {
            detail: "execution draft descriptor digest does not match canonical request descriptor digest"
                .to_string(),
        });
    }
    if binding.input_digest() != request.canonical_input().digest() {
        return Err(StrategyLoweringError::RequestExecutionMismatch {
            detail: "execution draft input digest does not match canonical request input digest"
                .to_string(),
        });
    }
    Ok(())
}

fn build_lowering_summary(
    execution: &StrategyExecutionDraft,
    bulk_mutation_batch: Option<&crate::transactions::data::ProvenanceCompleteBulkMutationBatch>,
) -> StrategyLoweringSummary {
    let planned = bulk_mutation_batch.map(|batch| batch.planned());
    StrategyLoweringSummary::new(
        execution.mutation_program().worker_batches().len(),
        execution.mutation_program().total_intent_count(),
        planned
            .map(|planned| planned.locality.touched_partitions.len())
            .unwrap_or(0),
        planned
            .map(|planned| planned.locality.cross_partition_relation_count)
            .unwrap_or(0),
        planned
            .map(|planned| planned.naming.normalized_client_keys.len())
            .unwrap_or(0),
        planned
            .map(|planned| planned.lineage.transitions.len())
            .unwrap_or(0),
        execution.summary().projected_entity_record_reads,
        execution.summary().projected_relation_record_reads,
        execution.summary().projected_partition_reads,
    )
}

#[cfg(test)]
mod tests {
    use super::lower_execution;
    use crate::commit_strategies::data::{
        CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact,
        CanonicalStrategyInputDigest, CanonicalStrategyOutputArtifact,
        CommitStrategyDescriptorDigest, CommitStrategyId, PersistentArtifactName,
        StrategyCallerProvenance, StrategyExecutionDraft, StrategyExecutionResult,
        StrategyExecutionSummary, StrategyInputSchemaName, StrategyInputSchemaVersion,
        StrategyMutationProgram, StrategyOutputSchemaName, StrategyRequestOrigin,
    };
    use crate::facade::transactions::{
        CreateIntent, MutationIntent, TransactionOptions, WorkerIntentBatch,
    };
    use crate::identity::data::{KindId, PartitionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::symbols::data::ClientKey;
    use crate::transactions::data::{AspectFieldPatch, EntitySpec};
    use worth_foundational::facade::{AspectKey, AspectValue, FieldKey, InternedString};

    fn canonical_request() -> CanonicalStrategyCommitRequest {
        CanonicalStrategyCommitRequest::new(
            CommitStrategyId(41),
            CommitStrategyDescriptorDigest([7; 32]),
            CanonicalStrategyInputArtifact::new(
                StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                StrategyInputSchemaVersion(1),
                b"replicas=3".to_vec().into(),
                CanonicalStrategyInputDigest([9; 32]),
                PersistentArtifactName::new("strategy.intent.reconcile.input"),
            ),
            StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            },
        )
    }

    fn execution_draft(request: &CanonicalStrategyCommitRequest) -> StrategyExecutionDraft {
        let batch = WorkerIntentBatch::new("reconcile-deployment").push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId(1),
                kind_id: KindId(1),
                client_key: ClientKey::from("deployment-a"),
                fields: AspectFieldPatch::from_locator(
                    crate::transactions::data::planned_single_field_locator(
                        AspectKey::new("name").expect("valid name aspect key"),
                        FieldKey::new("name").expect("valid name field key"),
                    ),
                    AspectValue::String(InternedString::Raw("deployment-a".to_string())),
                ),
            }),
        ));

        StrategyExecutionDraft::from_measured_result(
            request,
            StrategyExecutionResult::new(
                CanonicalStrategyOutputArtifact::new(
                    StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
                    b"status=planned".to_vec(),
                    PersistentArtifactName::new("strategy.intent.reconcile.output"),
                ),
                StrategyMutationProgram::new(vec![batch]),
            ),
            StrategyExecutionSummary::default(),
        )
    }

    #[test]
    fn lower_execution_routes_strategy_batches_through_transaction_admission() {
        let mut runtime = RelationalRuntimeBuilder::new()
            .schema_registry(crate::tests::support::test_schema_registry())
            .build();
        let request = canonical_request();
        let execution = execution_draft(&request);

        let lowered = lower_execution(
            &mut runtime,
            &request,
            &execution,
            TransactionOptions::default(),
        )
        .expect("lowered strategy plan");

        assert_eq!(lowered.request().strategy_id(), CommitStrategyId(41));
        assert_eq!(lowered.merged_plan().merged_intents.len(), 1);
        assert_eq!(
            lowered.execution().mutation_program().total_intent_count(),
            1
        );
        assert_eq!(
            lowered.lowering_provenance().mutation_program_digest(),
            execution.mutation_program().digest()
        );
        assert_eq!(lowered.lowering_summary().worker_batch_count(), 1);
        assert_eq!(lowered.lowering_summary().total_intent_count(), 1);
        assert_eq!(lowered.lowering_summary().normalized_client_key_count(), 1);
        assert_eq!(lowered.lowering_summary().lineage_transition_count(), 1);
        assert_eq!(lowered.lowering_summary().touched_partition_count(), 1);
        assert!(lowered.bulk_mutation_batch().is_some());
    }

    #[test]
    fn lower_execution_rejects_request_execution_mismatch() {
        let mut runtime = RelationalRuntimeBuilder::new().build();
        let request = canonical_request();
        let other_request = CanonicalStrategyCommitRequest::new(
            CommitStrategyId(42),
            request.descriptor_digest(),
            request.canonical_input().clone(),
            request.caller_provenance().clone(),
        );
        let execution = execution_draft(&request);

        let error = lower_execution(
            &mut runtime,
            &other_request,
            &execution,
            TransactionOptions::default(),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            crate::commit_strategies::data::StrategyLoweringError::RequestExecutionMismatch { .. }
        ));
    }
}
