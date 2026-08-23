mod admitted_projection;
mod invoker_isolation;
mod processing;
mod sealed_commit;
mod temporal_idempotency;
pub(in crate::domain_computation::primary_graph::conditional_operation) use invoker_isolation::isolate_invoker;
pub(super) use processing::{reenter_retained_wakes, WorthQueryTemporalReentryCounts};
#[cfg(test)]
mod tests;

use worth_query_installation::facade::{
    ApplicationFieldUnit, ApplicationSchema, OperationReads, OperationWrites,
    TypedApplicationReadableValue, TypedApplicationValue, WorthQueryInstalledApplicationOperation,
    WorthQueryTemporalIntentCandidate, WorthQueryTemporalIntentRevisionValue, WritableCapability,
    WritePosture,
};

use super::operation_invocation::{
    WorthQueryTemporalOperationExecution, WorthQueryTemporalOperationInvoker,
};
use super::reconstruction_authority::{
    WorthQueryTemporalPrincipalSource, WorthQueryTemporalReconstructionAccess,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

pub(super) enum WorthQueryTemporalReentryOutcome {
    Committed,
    AlreadyCommitted,
    Obsolete,
    RetryableFailure(String),
    Indeterminate(String),
}

pub(super) struct WorthQueryTemporalReentryAttempt {
    pub(super) outcome: WorthQueryTemporalReentryOutcome,
    pub(super) admission_canonical_work:
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn reenter_temporal_operation<
    Schema,
    Operation,
    Input,
    Scope,
    PrincipalBinding,
    PrincipalMapping,
    Principal,
    PrincipalIdentity,
    ScopeAspect,
    ScopeField,
    ScopeValue,
    ScopeWrite,
    ScopeUnit,
    PrincipalSource,
    QueryAuthorization,
    Invoker,
    IntentEntity,
    IdentityAspect,
    IdentityField,
    IdentityValue,
    IdentityWrite,
    IdentityUnit,
    RevisionAspect,
    RevisionField,
    RevisionValue,
    RevisionWrite,
    RevisionEquality,
    RevisionUnit,
    LifecycleAspect,
    LifecycleField,
    LifecycleValue,
    LifecycleWrite,
    LifecycleEquality,
    LifecycleUnit,
    Authorization,
    Clock,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    access: &WorthQueryTemporalReconstructionAccess<
        Schema,
        PrincipalBinding,
        PrincipalMapping,
        Principal,
        PrincipalIdentity,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeValue,
        ScopeWrite,
        ScopeUnit,
        PrincipalSource,
        QueryAuthorization,
    >,
    execution: &WorthQueryTemporalOperationExecution<
        Schema,
        Operation,
        Input,
        Scope,
        Invoker,
        IntentEntity,
        IdentityAspect,
        IdentityField,
        IdentityValue,
        IdentityWrite,
        IdentityUnit,
        RevisionAspect,
        RevisionField,
        RevisionValue,
        RevisionWrite,
        RevisionEquality,
        RevisionUnit,
        LifecycleAspect,
        LifecycleField,
        LifecycleValue,
        LifecycleWrite,
        LifecycleEquality,
        LifecycleUnit,
        Authorization,
    >,
    candidate: &WorthQueryTemporalIntentCandidate<Clock, Input>,
    runtime_binding: &super::canonical_identity::WorthQueryTemporalRuntimeBindingIdentity,
) -> WorthQueryTemporalReentryAttempt
where
    Schema: ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
    PrincipalIdentity: worth_query_installation::facade::TypedApplicationIdentityValue,
    ScopeValue: TypedApplicationValue + Clone,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
    PrincipalSource: WorthQueryTemporalPrincipalSource<Schema>,
    Invoker: WorthQueryTemporalOperationInvoker<Schema, Operation, Input, Scope>,
    IdentityField: OperationReads<Operation>,
    IdentityValue: TypedApplicationReadableValue + Clone,
    IdentityWrite: WritePosture,
    IdentityUnit: ApplicationFieldUnit,
    RevisionField: OperationReads<Operation> + OperationWrites<Operation>,
    RevisionValue: WorthQueryTemporalIntentRevisionValue + TypedApplicationReadableValue + Clone,
    RevisionWrite: WritableCapability,
    RevisionUnit: ApplicationFieldUnit,
    LifecycleField: OperationReads<Operation> + OperationWrites<Operation>,
    LifecycleValue: TypedApplicationReadableValue + Clone,
    LifecycleWrite: WritableCapability,
    LifecycleUnit: ApplicationFieldUnit,
    Authorization: super::WorthQueryTemporalOperationAuthorization<Schema, Operation, Input, Scope>,
{
    let idempotency =
        match temporal_idempotency::prepare_temporal_idempotency(runtime_binding, candidate) {
            Ok(idempotency) => idempotency,
            Err(denial) => {
                return WorthQueryTemporalReentryAttempt {
                    outcome: WorthQueryTemporalReentryOutcome::RetryableFailure(format!(
                        "temporal idempotency admission denied: {denial:?}"
                    )),
                    admission_canonical_work:
                        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
                }
            }
        };
    let admission_canonical_work = idempotency.canonical_work();
    let result = try_reentry(
        runtime,
        operation,
        access,
        execution,
        candidate,
        &idempotency,
    );
    let outcome = match result {
        Ok(outcome) => outcome,
        Err(detail) => WorthQueryTemporalReentryOutcome::RetryableFailure(detail),
    };
    WorthQueryTemporalReentryAttempt {
        outcome,
        admission_canonical_work,
    }
}

#[allow(clippy::too_many_arguments)]
fn try_reentry<
    Schema,
    Operation,
    Input,
    Scope,
    PrincipalBinding,
    PrincipalMapping,
    Principal,
    PrincipalIdentity,
    ScopeAspect,
    ScopeField,
    ScopeValue,
    ScopeWrite,
    ScopeUnit,
    PrincipalSource,
    QueryAuthorization,
    Invoker,
    IntentEntity,
    IdentityAspect,
    IdentityField,
    IdentityValue,
    IdentityWrite,
    IdentityUnit,
    RevisionAspect,
    RevisionField,
    RevisionValue,
    RevisionWrite,
    RevisionEquality,
    RevisionUnit,
    LifecycleAspect,
    LifecycleField,
    LifecycleValue,
    LifecycleWrite,
    LifecycleEquality,
    LifecycleUnit,
    Authorization,
    Clock,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    access: &WorthQueryTemporalReconstructionAccess<
        Schema,
        PrincipalBinding,
        PrincipalMapping,
        Principal,
        PrincipalIdentity,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeValue,
        ScopeWrite,
        ScopeUnit,
        PrincipalSource,
        QueryAuthorization,
    >,
    execution: &WorthQueryTemporalOperationExecution<
        Schema,
        Operation,
        Input,
        Scope,
        Invoker,
        IntentEntity,
        IdentityAspect,
        IdentityField,
        IdentityValue,
        IdentityWrite,
        IdentityUnit,
        RevisionAspect,
        RevisionField,
        RevisionValue,
        RevisionWrite,
        RevisionEquality,
        RevisionUnit,
        LifecycleAspect,
        LifecycleField,
        LifecycleValue,
        LifecycleWrite,
        LifecycleEquality,
        LifecycleUnit,
        Authorization,
    >,
    candidate: &WorthQueryTemporalIntentCandidate<Clock, Input>,
    idempotency: &temporal_idempotency::WorthQueryPreparedTemporalIdempotency,
) -> Result<WorthQueryTemporalReentryOutcome, String>
where
    Schema: ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
    PrincipalIdentity: worth_query_installation::facade::TypedApplicationIdentityValue,
    ScopeValue: TypedApplicationValue + Clone,
    ScopeWrite: WritePosture,
    ScopeUnit: ApplicationFieldUnit,
    PrincipalSource: WorthQueryTemporalPrincipalSource<Schema>,
    Invoker: WorthQueryTemporalOperationInvoker<Schema, Operation, Input, Scope>,
    IdentityField: OperationReads<Operation>,
    IdentityValue: TypedApplicationReadableValue + Clone,
    IdentityWrite: WritePosture,
    IdentityUnit: ApplicationFieldUnit,
    RevisionField: OperationReads<Operation> + OperationWrites<Operation>,
    RevisionValue: WorthQueryTemporalIntentRevisionValue + TypedApplicationReadableValue + Clone,
    RevisionWrite: WritableCapability,
    RevisionUnit: ApplicationFieldUnit,
    LifecycleField: OperationReads<Operation> + OperationWrites<Operation>,
    LifecycleValue: TypedApplicationReadableValue + Clone,
    LifecycleWrite: WritableCapability,
    LifecycleUnit: ApplicationFieldUnit,
    Authorization: super::WorthQueryTemporalOperationAuthorization<Schema, Operation, Input, Scope>,
{
    let fresh = access.resolve_fresh_operation_access(runtime)?;
    let Some(current) = execution.resolve_current_intent(
        runtime,
        candidate.record_identity(),
        candidate.revision(),
        &fresh.request,
    )?
    else {
        return Ok(WorthQueryTemporalReentryOutcome::Obsolete);
    };
    let Some(projected) =
        execution.admit_current_projection(runtime, operation, candidate, &fresh, &current)?
    else {
        return Ok(WorthQueryTemporalReentryOutcome::Obsolete);
    };
    execution.commit_projected_temporal_effect(runtime, candidate, current, projected, idempotency)
}
