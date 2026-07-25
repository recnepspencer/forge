use crate::{
    WorthServerCompletedProductOperation, WorthServerProductIdempotencyConflict,
    WorthServerProductOperationDenialCode, WorthServerProductOperationExecutionBoundary,
    WorthServerProductOperationOutcome, WorthServerProductOperationRetryReceipt,
    WorthServerProductOperationSurfaceDenial, WorthServerProductOperationSurfaceDenialCode,
    WorthServerProductOperationSurfaceDenialFacts, WorthServerScheduledProductOperation,
};

use super::{
    WorthServerAdmittedDurableProductMutation, WorthServerDurableProductMutationCompletion,
    WorthServerDurableProductMutationConclusion, WorthServerDurableProductMutationExecutor,
};

pub(crate) fn execute_durable_product_mutation(
    executor: &dyn WorthServerDurableProductMutationExecutor,
    scheduled: &WorthServerScheduledProductOperation,
    contract: &super::WorthServerDurableProductMutationContract,
    counters: &crate::diagnostics::WorthServerCounters,
) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial> {
    let attempt = WorthServerAdmittedDurableProductMutation::from_scheduled(scheduled, contract)?;
    counters.increment_durable_product_mutation_attempts();
    let execution = executor.execute(&attempt);
    counters.record_durable_product_basis_comparisons(execution.basis_comparison_count());
    match execution.into_conclusion() {
        WorthServerDurableProductMutationConclusion::Committed(completion) => {
            validate_completion(&attempt, &completion)?;
            counters.increment_durable_product_commits();
            counters.record_product_result_artifact(
                completion.success().result_artifact().body().byte_len(),
            );
            completed_from_conclusion(scheduled, completion, false)
        }
        WorthServerDurableProductMutationConclusion::PreviouslyCommitted(completion) => {
            validate_completion(&attempt, &completion)?;
            counters.increment_durable_product_previously_committed();
            counters.record_product_result_artifact(
                completion.success().result_artifact().body().byte_len(),
            );
            completed_from_conclusion(scheduled, completion, true)
        }
        WorthServerDurableProductMutationConclusion::StaleBasis {
            observed_basis_digest,
        } => {
            counters.increment_durable_product_stale_bases();
            Err(durable_stale_basis_denial(&attempt, &observed_basis_digest))
        }
        WorthServerDurableProductMutationConclusion::IdempotencyConflict {
            bound_request_digest,
        } => {
            counters.increment_durable_product_idempotency_conflicts();
            Err(durable_idempotency_conflict(
                &attempt,
                &bound_request_digest,
            ))
        }
        WorthServerDurableProductMutationConclusion::Rejected(denial) => {
            let outcome = WorthServerProductOperationOutcome::Denied(
                denial.with_code(WorthServerProductOperationDenialCode::ProductSemantic),
            );
            let envelope = crate::product_adapter::build_envelope(scheduled, &outcome);
            Ok(WorthServerCompletedProductOperation::new(outcome, envelope)
                .with_durable_executor_attempt(scheduled))
        }
        WorthServerDurableProductMutationConclusion::InvalidResultArtifact(error) => {
            if error.code()
                == crate::WorthServerProductResultArtifactErrorCode::InlineBudgetExceeded
            {
                counters.increment_product_result_oversized_denials();
            }
            Err(WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::InvalidResultArtifact,
                error.detail().to_string(),
            )
            .with_facts(
                WorthServerProductOperationSurfaceDenialFacts::default().with_execution_boundary(
                    WorthServerProductOperationExecutionBoundary::DurableExecutorAttempted,
                ),
            ))
        }
        WorthServerDurableProductMutationConclusion::Indeterminate(recovery) => {
            counters.increment_durable_product_indeterminate();
            Err(WorthServerProductOperationSurfaceDenial::new(
                WorthServerProductOperationSurfaceDenialCode::Indeterminate,
                "durable product mutation has an indeterminate commit conclusion".to_string(),
            )
            .with_facts(
                WorthServerProductOperationSurfaceDenialFacts::default()
                    .with_execution_boundary(
                        WorthServerProductOperationExecutionBoundary::DurableExecutorAttempted,
                    )
                    .with_recovery_handle(recovery),
            ))
        }
        WorthServerDurableProductMutationConclusion::Failed { reason_key, detail } => {
            let outcome = WorthServerProductOperationOutcome::failed(reason_key, detail);
            let envelope = crate::product_adapter::build_envelope(scheduled, &outcome);
            Ok(WorthServerCompletedProductOperation::new(outcome, envelope)
                .with_durable_executor_attempt(scheduled))
        }
    }
}

fn completed_from_conclusion(
    scheduled: &WorthServerScheduledProductOperation,
    completion: WorthServerDurableProductMutationCompletion,
    previously_committed: bool,
) -> Result<WorthServerCompletedProductOperation, WorthServerProductOperationSurfaceDenial> {
    let outcome = WorthServerProductOperationOutcome::Success(completion.success().clone());
    let envelope = crate::product_adapter::build_durable_envelope(
        scheduled,
        &outcome,
        completion.canonical_digest(),
    );
    let request = scheduled.plan().operation_admission().operation_request();
    let receipt = if previously_committed {
        WorthServerProductOperationRetryReceipt::previously_committed(
            request
                .identity()
                .idempotency_key()
                .expect("durable attempt key"),
            completion.request_digest(),
            completion.canonical_digest(),
        )
    } else {
        WorthServerProductOperationRetryReceipt::executed(
            request
                .identity()
                .idempotency_key()
                .expect("durable attempt key"),
            completion.request_digest(),
        )
    };
    let durable_disposition = if previously_committed {
        super::WorthServerDurableProductMutationDisposition::PreviouslyCommitted
    } else {
        super::WorthServerDurableProductMutationDisposition::Committed
    };
    let durable_receipt = super::WorthServerDurableProductMutationReceipt::from_completion(
        &completion,
        durable_disposition,
    );
    Ok(WorthServerCompletedProductOperation::new(outcome, envelope)
        .with_durable_executor_attempt(scheduled)
        .with_durable_mutation_receipt(durable_receipt)
        .with_retry_receipt(receipt))
}

fn validate_completion(
    attempt: &WorthServerAdmittedDurableProductMutation,
    completion: &WorthServerDurableProductMutationCompletion,
) -> Result<(), WorthServerProductOperationSurfaceDenial> {
    if completion.matches_attempt(attempt) {
        return Ok(());
    }
    Err(WorthServerProductOperationSurfaceDenial::new(
        WorthServerProductOperationSurfaceDenialCode::InvalidResultArtifact,
        "durable product executor returned a completion outside the admitted attempt".to_string(),
    )
    .with_facts(
        WorthServerProductOperationSurfaceDenialFacts::default().with_execution_boundary(
            WorthServerProductOperationExecutionBoundary::DurableExecutorAttempted,
        ),
    ))
}

fn durable_stale_basis_denial(
    attempt: &WorthServerAdmittedDurableProductMutation,
    observed_basis_digest: &str,
) -> WorthServerProductOperationSurfaceDenial {
    WorthServerProductOperationSurfaceDenial::new(
        WorthServerProductOperationSurfaceDenialCode::PreconditionDenied,
        format!(
            "durable product basis `{}` did not match current product basis `{observed_basis_digest}`",
            attempt.expected_basis().base_digest().value(),
        ),
    )
    .with_facts(
        WorthServerProductOperationSurfaceDenialFacts::default()
            .with_execution_boundary(
                WorthServerProductOperationExecutionBoundary::DurableExecutorAttempted,
            )
            .with_basis_mismatch(crate::WorthServerProductStaleBasisDenial::new(
                attempt.expected_basis().base_digest().value(),
                observed_basis_digest,
            )),
    )
}

fn durable_idempotency_conflict(
    attempt: &WorthServerAdmittedDurableProductMutation,
    bound_request_digest: &str,
) -> WorthServerProductOperationSurfaceDenial {
    let conflict = WorthServerProductIdempotencyConflict::new(
        attempt.idempotency_key().value(),
        attempt.request_digest(),
        bound_request_digest,
    );
    WorthServerProductOperationSurfaceDenial::new(
        WorthServerProductOperationSurfaceDenialCode::IdempotencyConflict,
        "durable product idempotency key is already bound to different request meaning".to_string(),
    )
    .with_facts(
        WorthServerProductOperationSurfaceDenialFacts::default()
            .with_execution_boundary(
                WorthServerProductOperationExecutionBoundary::DurableExecutorAttempted,
            )
            .with_idempotency_conflict(conflict),
    )
}
