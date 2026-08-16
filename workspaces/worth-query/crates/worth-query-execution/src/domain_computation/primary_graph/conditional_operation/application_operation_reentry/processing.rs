use std::collections::BTreeMap;

use worth_query_installation::facade::{
    ApplicationFieldUnit, ApplicationSchema, OperationReads, OperationWrites,
    TypedApplicationReadableValue, TypedApplicationValue, WorthQueryInstalledApplicationOperation,
    WorthQueryTemporalIntentCandidate, WorthQueryTemporalIntentRevisionValue, WritableCapability,
    WritePosture,
};
use worth_runtime_bridge::facade::{
    BridgeConditionalDecisionEvidence, BridgeManagedClockBinding,
    BridgeManagedTemporalIntentLifecycle, BridgeManagedTemporalIntentReconciliation,
    BridgeManagedTemporalIntentReconciliationParts, BridgeOwnedSignalRuntime,
};

use super::{reenter_temporal_operation, WorthQueryTemporalReentryOutcome};
use crate::domain_computation::primary_graph::conditional_operation::{
    operation_invocation::{
        WorthQueryTemporalOperationExecution, WorthQueryTemporalOperationInvoker,
    },
    reconstruction_authority::{
        WorthQueryTemporalPrincipalSource, WorthQueryTemporalReconstructionAccess,
    },
    signal_decision_reentry::{
        WorthQueryRetainedConditionalDecision, WorthQueryRetainedConditionalWake,
    },
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

#[derive(Clone, Copy, Default)]
pub(in crate::domain_computation::primary_graph::conditional_operation) struct WorthQueryTemporalReentryCounts
{
    pub(in crate::domain_computation::primary_graph::conditional_operation) committed: usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) already_committed:
        usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) failed: usize,
    pub(in crate::domain_computation::primary_graph::conditional_operation) indeterminate: usize,
}

#[allow(clippy::too_many_arguments)]
pub(in crate::domain_computation::primary_graph::conditional_operation) fn reenter_retained_wakes<
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
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
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
    candidates: &mut BTreeMap<
        String,
        super::super::temporal_reconstruction::WorthQueryReconstructedTemporalIntent<Clock, Input>,
    >,
    wakes: &mut Vec<WorthQueryRetainedConditionalWake>,
    runtime_binding: &crate::domain_computation::primary_graph::conditional_operation::canonical_identity::WorthQueryTemporalRuntimeBindingIdentity,
) -> WorthQueryTemporalReentryCounts
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
    Authorization:
        super::super::WorthQueryTemporalOperationAuthorization<Schema, Operation, Input, Scope>,
{
    let mut counts = WorthQueryTemporalReentryCounts::default();
    for wake in wakes.iter_mut() {
        let decision = std::mem::replace(
            &mut wake.decision,
            WorthQueryRetainedConditionalDecision::Failed(
                "temporal operation re-entry was interrupted".to_string(),
            ),
        );
        let evidence = match decision {
            WorthQueryRetainedConditionalDecision::Eligible(evidence)
            | WorthQueryRetainedConditionalDecision::OperationRetryable(evidence, _)
            | WorthQueryRetainedConditionalDecision::OperationIndeterminate(evidence, _) => {
                evidence
            }
            other => {
                wake.decision = other;
                continue;
            }
        };
        let identity = wake.due.intent_identity().as_str();
        let Some(candidate) = candidates.get(identity) else {
            wake.decision =
                WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted(evidence);
            continue;
        };
        if !wake_matches_candidate(wake, candidate.candidate()) {
            wake.decision =
                WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted(evidence);
            continue;
        }
        wake.application_attempted = true;
        let attempt = reenter_temporal_operation(
            runtime,
            operation,
            access,
            execution,
            candidate.candidate(),
            runtime_binding,
        );
        wake.application_admission_canonical_work = attempt.admission_canonical_work;
        apply_reentry_outcome(
            bridge,
            clock,
            candidates,
            wake,
            identity.to_string(),
            evidence,
            attempt.outcome,
            &mut counts,
        );
    }
    counts
}

#[allow(clippy::too_many_arguments)]
fn apply_reentry_outcome<Clock, Input>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    candidates: &mut BTreeMap<
        String,
        super::super::temporal_reconstruction::WorthQueryReconstructedTemporalIntent<Clock, Input>,
    >,
    wake: &mut WorthQueryRetainedConditionalWake,
    identity: String,
    evidence: BridgeConditionalDecisionEvidence,
    outcome: WorthQueryTemporalReentryOutcome,
    counts: &mut WorthQueryTemporalReentryCounts,
) {
    match outcome {
        WorthQueryTemporalReentryOutcome::Committed => complete_wake(
            bridge, clock, candidates, wake, identity, evidence, counts, true,
        ),
        WorthQueryTemporalReentryOutcome::AlreadyCommitted => complete_wake(
            bridge, clock, candidates, wake, identity, evidence, counts, false,
        ),
        WorthQueryTemporalReentryOutcome::Obsolete => {
            retire_obsolete(bridge, clock, candidates, wake, identity, evidence, counts)
        }
        WorthQueryTemporalReentryOutcome::RetryableFailure(detail) => {
            wake.decision =
                WorthQueryRetainedConditionalDecision::OperationRetryable(evidence, detail);
            counts.failed += 1;
        }
        WorthQueryTemporalReentryOutcome::Indeterminate(detail) => {
            wake.decision =
                WorthQueryRetainedConditionalDecision::OperationIndeterminate(evidence, detail);
            counts.indeterminate += 1;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn complete_wake<Clock, Input>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    candidates: &mut BTreeMap<
        String,
        super::super::temporal_reconstruction::WorthQueryReconstructedTemporalIntent<Clock, Input>,
    >,
    wake: &mut WorthQueryRetainedConditionalWake,
    identity: String,
    evidence: BridgeConditionalDecisionEvidence,
    counts: &mut WorthQueryTemporalReentryCounts,
    committed: bool,
) {
    if let Err(detail) = retire_committed_wake(bridge, clock, wake) {
        wake.decision = WorthQueryRetainedConditionalDecision::OperationRetryable(evidence, detail);
        counts.failed += 1;
        return;
    }
    candidates.remove(&identity);
    if committed {
        wake.decision = WorthQueryRetainedConditionalDecision::OperationCommitted(evidence);
        counts.committed += 1;
    } else {
        wake.decision = WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted(evidence);
        counts.already_committed += 1;
    }
}

#[allow(clippy::too_many_arguments)]
fn retire_obsolete<Clock, Input>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    candidates: &mut BTreeMap<
        String,
        super::super::temporal_reconstruction::WorthQueryReconstructedTemporalIntent<Clock, Input>,
    >,
    wake: &mut WorthQueryRetainedConditionalWake,
    identity: String,
    evidence: BridgeConditionalDecisionEvidence,
    counts: &mut WorthQueryTemporalReentryCounts,
) {
    match retire_obsolete_wake(bridge, clock, wake) {
        Ok(()) => {
            candidates.remove(&identity);
            wake.decision =
                WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted(evidence);
        }
        Err(detail) => {
            wake.decision =
                WorthQueryRetainedConditionalDecision::OperationRetryable(evidence, detail);
            counts.failed += 1;
        }
    }
}

fn wake_matches_candidate<Clock, Input>(
    wake: &WorthQueryRetainedConditionalWake,
    candidate: &WorthQueryTemporalIntentCandidate<Clock, Input>,
) -> bool {
    wake.due.revision() == candidate.revision()
        && wake.due.due_coordinate() == candidate.due().nanoseconds()
        && wake.due.idempotency_identity() == candidate.idempotency().as_str()
        && wake.due.intent_identity().as_str() == candidate.identity().as_str()
}

fn retire_committed_wake(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    wake: &WorthQueryRetainedConditionalWake,
) -> Result<(), String> {
    let revision = wake
        .due
        .revision()
        .checked_add(1)
        .ok_or_else(|| "committed temporal intent revision overflowed".to_string())?;
    let outcome = bridge
        .reconcile_managed_temporal_intent(BridgeManagedTemporalIntentReconciliationParts {
            binding: clock,
            identity: wake.due.intent_identity().clone(),
            revision,
            due_coordinate: wake.due.due_coordinate(),
            idempotency_identity: std::sync::Arc::from(wake.due.idempotency_identity()),
            source_record_identity: wake.due.source_record_identity(),
            lifecycle: BridgeManagedTemporalIntentLifecycle::Completed,
        })
        .map_err(|denial| denial.detail().to_string())?;
    if outcome == BridgeManagedTemporalIntentReconciliation::Retired {
        Ok(())
    } else {
        Err("committed temporal intent did not retire its exact managed wake".to_string())
    }
}

fn retire_obsolete_wake(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    wake: &WorthQueryRetainedConditionalWake,
) -> Result<(), String> {
    retire_committed_wake(bridge, clock, wake)
}
