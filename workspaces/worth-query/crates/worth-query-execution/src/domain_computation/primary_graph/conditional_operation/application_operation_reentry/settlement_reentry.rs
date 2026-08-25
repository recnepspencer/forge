use crate::domain_computation::primary_graph::{
    WorthQueryApplicationSettlementDeferred, WorthQueryApplicationSettlementRecoveryError,
    WorthQueryPrimaryGraphApplicationRuntime,
};

pub(super) enum WorthQuerySettlementReentry {
    RetryApplicationPublication(WorthQueryApplicationSettlementDeferred),
    AlreadyCommitted,
    Indeterminate(String),
    Deferred(WorthQueryApplicationSettlementDeferred),
}

pub(super) fn repair<Schema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    deferred: WorthQueryApplicationSettlementDeferred,
) -> WorthQuerySettlementReentry
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    if deferred.requires_idempotency_readmission() {
        return WorthQuerySettlementReentry::RetryApplicationPublication(deferred);
    }
    match runtime.recover_deferred_application_settlement(&deferred) {
        Ok(_) => WorthQuerySettlementReentry::AlreadyCommitted,
        Err(WorthQueryApplicationSettlementRecoveryError::IdempotencyAbsent) => {
            WorthQuerySettlementReentry::Indeterminate(
                "settled temporal commit has no exact idempotency evidence".to_string(),
            )
        }
        Err(WorthQueryApplicationSettlementRecoveryError::IdempotencyDrift) => {
            WorthQuerySettlementReentry::Indeterminate(
                "settled temporal commit has drifting idempotency evidence".to_string(),
            )
        }
        Err(
            WorthQueryApplicationSettlementRecoveryError::Durability(_)
            | WorthQueryApplicationSettlementRecoveryError::Publication(_),
        ) => WorthQuerySettlementReentry::Deferred(deferred),
    }
}

pub(super) fn retry_application_publication<Clock, Input, Schema>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    candidate: &worth_query_installation::facade::WorthQueryTemporalIntentCandidate<Clock, Input>,
    runtime_binding: &super::super::canonical_identity::WorthQueryTemporalRuntimeBindingIdentity,
    deferred: WorthQueryApplicationSettlementDeferred,
) -> super::WorthQueryTemporalReentryAttempt
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    let prepared =
        match super::temporal_idempotency::prepare_temporal_idempotency(runtime_binding, candidate)
        {
            Ok(prepared) => prepared,
            Err(denial) => {
                return retry_attempt(
                    super::WorthQueryTemporalReentryOutcome::RetryableFailure(format!(
                        "temporal publication retry idempotency denied: {denial:?}"
                    )),
                    worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
                );
            }
        };
    let canonical_work = prepared.canonical_work();
    let retained = deferred.idempotency_binding();
    if prepared.binding().key_identity() != retained.key_identity()
        || prepared.binding().intent_identity() != retained.intent_identity()
    {
        return retry_attempt(
            super::WorthQueryTemporalReentryOutcome::Indeterminate(
                "settled publication retry does not match the retained temporal candidate"
                    .to_string(),
            ),
            canonical_work,
        );
    }
    let outcome = match runtime.recover_deferred_application_settlement(&deferred) {
        Ok(_) => super::WorthQueryTemporalReentryOutcome::AlreadyCommitted,
        Err(WorthQueryApplicationSettlementRecoveryError::IdempotencyAbsent) => {
            super::WorthQueryTemporalReentryOutcome::Indeterminate(
                "settled publication retry has no exact idempotency evidence".to_string(),
            )
        }
        Err(WorthQueryApplicationSettlementRecoveryError::IdempotencyDrift) => {
            super::WorthQueryTemporalReentryOutcome::Indeterminate(
                "settled publication retry has drifting idempotency evidence".to_string(),
            )
        }
        Err(
            WorthQueryApplicationSettlementRecoveryError::Durability(_)
            | WorthQueryApplicationSettlementRecoveryError::Publication(_),
        ) => super::WorthQueryTemporalReentryOutcome::SettlementDeferred(deferred),
    };
    retry_attempt(outcome, canonical_work)
}

fn retry_attempt(
    outcome: super::WorthQueryTemporalReentryOutcome,
    admission_canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
) -> super::WorthQueryTemporalReentryAttempt {
    super::WorthQueryTemporalReentryAttempt {
        outcome,
        admission_canonical_work,
    }
}
