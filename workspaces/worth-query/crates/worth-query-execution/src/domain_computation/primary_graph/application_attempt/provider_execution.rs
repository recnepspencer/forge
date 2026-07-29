mod decision_facts;
mod support;

use worth_query_admission::facade::basis::basis_lifecycle;
use worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceAdmissionCounters;
use worth_query_admission::integration::admit_execution_resource_plan;
use worth_query_installation::facade::{ApplicationSchema, APPLICATION_INVARIANT_SLOT};
use worth_relational::facade::bridge::bridge_snapshot_identity_for_handle;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, BridgeTruthViewSelector,
    HistoricalEvaluationDeclaration, SnapshotReadPacket, TruthBranchIdentity,
};

use super::provider_binding::{
    prepare_provider_attempt, WorthQueryPreparedApplicationProviderAttempt,
};
use super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitDenialStage as DenialStage,
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationEffectProgram,
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationStaleAttempt,
};
use crate::domain_computation::operation_binding::WorthQueryApplicationOperationBindingInput;
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::{
    WorthQueryDecisionReadSetFreshnessOutcome, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryInvariantStateLocator, WorthQueryManagedRunTerminalKind,
    WorthQueryManagedTruthReadRequest, WorthQueryProviderCompareAndCommitOutcome,
};

use decision_facts::bind_provider_decision_facts;
use support::{application_resource_request, denied, parse_provider_receipt};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn compare_and_commit_application<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> WorthQueryApplicationCommitOutcome {
        let WorthQueryApplicationEffectProgram {
            read_set,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        } = program;
        let mut admission = read_set.admission;
        if admission.validate_current_authority().is_err() {
            return WorthQueryApplicationCommitOutcome::Cancelled;
        }
        {
            let _serialization = self.primary_provider.serialize_application_commit();
            match self
                .primary_provider
                .resolve_idempotency_binding(idempotency)
            {
                Ok(WorthQueryProviderIdempotencyResolution::Absent) => {}
                Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
                    return WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt);
                }
                Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
                    return WorthQueryApplicationCommitOutcome::Denied(
                        WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                    );
                }
                Err(_) => return denied(DenialStage::Idempotency),
            }
        }
        let authorization =
            match admission.take_authorization_dependencies(self.authorization.bridge()) {
                Ok(authorization) => authorization,
                Err(_) => return denied(DenialStage::DecisionReadSet),
            };
        let prepared = match prepare_provider_attempt(
            read_set.facts,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        ) {
            Ok(prepared) => prepared,
            Err(_) => return denied(DenialStage::ProposalBinding),
        };
        let snapshot = read_set.lease.snapshot();
        let branch = TruthBranchIdentity::from_relational_branch_id("main");
        let bridge_snapshot = bridge_snapshot_identity_for_handle(snapshot);
        if self
            .bridge
            .plan_truth_view_packet(
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::branch_snapshot(
                        branch.clone(),
                        bridge_snapshot.clone(),
                    ),
                    BridgeReplayMode::Disabled,
                    BridgeDiagnosticsTier::Standard,
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
                SnapshotReadPacket::new(Vec::new()),
            )
            .is_err()
        {
            return denied(DenialStage::BridgePlanning);
        }
        let basis_path = match basis_lifecycle()
            .branch_snapshot(
                "main",
                format!(
                    "relational-snapshot:{}:{}",
                    snapshot.snapshot_id.0, snapshot.version_id.0
                ),
            )
            .for_mutation_preparation()
        {
            Ok(path) => path,
            Err(_) => return denied(DenialStage::BasisAdmission),
        };
        let basis = match basis_path.admit() {
            Ok(basis) => basis,
            Err(_) => return denied(DenialStage::BasisAdmission),
        };
        let operation = WorthQueryExecutionBoundOperationAuthority::bind_application(
            WorthQueryApplicationOperationBindingInput {
                runtime: &self.runtime,
                owner: self.installed_schema.owner(),
                installed_operation_fingerprint: admission.installed_operation_fingerprint(),
                operation_scope_fingerprint: admission.operation_scope_fingerprint().bytes(),
                basis: &basis,
                contracts: admission.allowed_graph_contract(),
                graph: &self.primary_graph_authority,
                support: self.primary_provider.application_resource_support(),
            },
        );
        let request = match application_resource_request(admission.allowed_graph_contract()) {
            Some(request) => request,
            None => return denied(DenialStage::ResourceAdmission),
        };
        let plan = match admit_execution_resource_plan(
            operation.binding_identity(),
            admission.allowed_graph_contract().resources(),
            &request,
            self.primary_provider.application_resource_support(),
            WorthQueryExecutionResourceAdmissionCounters::default(),
        ) {
            Ok(plan) => plan,
            Err(_) => return denied(DenialStage::ResourceAdmission),
        };
        let attempt = match self.runtime.start_direct_resource_attempt(&operation, plan) {
            Ok(attempt) => attempt,
            Err(_) => return denied(DenialStage::ResourceAdmission),
        };
        let read_request = WorthQueryManagedTruthReadRequest::new(
            snapshot.version_id,
            branch,
            SnapshotReadPacket::new(Vec::new()),
        );
        let request_bridge = self.bridge.fork_managed_request_lane();
        let mut running = match self
            .runtime
            .managed_run_admission(&request_bridge, &self.relational_source)
            .admit_direct(&operation, attempt, read_request)
        {
            Ok(admitted) => admitted.start(),
            Err(_) => return denied(DenialStage::ManagedRunAdmission),
        };
        let outcome = execute_provider_progression(
            &mut running,
            &self.primary_graph_authority,
            &self.primary_provider,
            &admission,
            prepared,
            authorization,
            idempotency,
        );
        let terminal = if matches!(
            outcome,
            WorthQueryApplicationCommitOutcome::Committed(_)
                | WorthQueryApplicationCommitOutcome::AlreadyCommitted(_)
        ) {
            WorthQueryManagedRunTerminalKind::Completed
        } else {
            WorthQueryManagedRunTerminalKind::Failed
        };
        if running
            .terminate_for_convergence(terminal)
            .cleanup()
            .is_err()
        {
            return WorthQueryApplicationCommitOutcome::Indeterminate;
        }
        outcome
    }
}

fn execute_provider_progression<Schema, Operation, Input, Scope>(
    running: &mut crate::domain_computation::WorthQueryRunningDirectRun,
    graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    provider: &std::sync::Arc<super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    prepared: WorthQueryPreparedApplicationProviderAttempt,
    authorization: Vec<
        crate::domain_computation::primary_graph::authorization::WorthQueryAuthorizationCommitDependency,
    >,
    idempotency: WorthQueryApplicationIdempotencyBinding,
) -> WorthQueryApplicationCommitOutcome {
    let staged = match running
        .admit_provider_execution_plan(graph)
        .and_then(|plan| plan.readmit())
        .and_then(|session| session.prepare())
    {
        Ok(prepared_session) => prepared_session.bind_reads_and_effects(),
        Err(_) => return denied(DenialStage::ProviderPlan),
    };
    let session_identity = staged.token_identity().to_owned();
    let WorthQueryPreparedApplicationProviderAttempt {
        facts,
        steps,
        batch,
        emissions,
    } = prepared;
    let (facts, requests) = match bind_provider_decision_facts(facts, authorization) {
        Ok(bound) => bound,
        Err(()) => return denied(DenialStage::DecisionReadSet),
    };
    if provider
        .register_application_attempt(
            staged.token_identity(),
            facts,
            steps.clone(),
            batch,
            emissions,
            idempotency,
        )
        .is_err()
    {
        let _ = staged.abort();
        return denied(DenialStage::ProviderPlan);
    }
    let receipt = match staged.read_authority().capture_decision_read_set(requests) {
        Ok(receipt) => receipt,
        Err(_) => {
            let _ = staged.abort();
            return denied(DenialStage::DecisionReadSet);
        }
    };
    let fresh = match staged.read_authority().compare_decision_read_set(receipt) {
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Fresh(fresh)) => fresh,
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale)) => {
            let resolution = {
                let _serialization = provider.serialize_application_commit();
                provider.resolve_application_idempotency(&session_identity)
            };
            match resolution {
                Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
                    let _ = staged.abort();
                    return WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt);
                }
                Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
                    let _ = staged.abort();
                    return WorthQueryApplicationCommitOutcome::Denied(
                        WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                    );
                }
                Ok(WorthQueryProviderIdempotencyResolution::Absent) => {}
                Err(_) => {
                    let _ = staged.abort();
                    return denied(DenialStage::Idempotency);
                }
            }
            let count = stale.stale_fact_count();
            let _ = staged.abort();
            return WorthQueryApplicationCommitOutcome::Stale(
                WorthQueryApplicationStaleAttempt::new(count),
            );
        }
        Err(_) => {
            let _ = staged.abort();
            return denied(DenialStage::DecisionReadSet);
        }
    };
    let lowered = match staged
        .effect_authority()
        .lower_provisional_program(&fresh, steps)
    {
        Ok(lowered) => lowered,
        Err(_) => {
            let _ = staged.abort();
            return denied(DenialStage::EffectLowering);
        }
    };
    let inspection = match staged.begin_provisional_attempt(fresh, lowered) {
        Ok(attempt) => attempt.materialize_proposed_state().inspect(),
        Err(_) => return denied(DenialStage::ProvisionalState),
    };
    let locators = inspection
        .facts()
        .iter()
        .map(|fact| {
            WorthQueryInvariantStateLocator::new("application-proposed-state", fact.identity())
        })
        .collect::<Result<Vec<_>, _>>();
    let receipt = match locators.and_then(|locators| {
        inspection
            .select_installed_invariant(APPLICATION_INVARIANT_SLOT)?
            .admit_state_load_plan(locators)?
            .execute()
    }) {
        Ok(receipt) => receipt,
        Err(_) => {
            inspection.discard();
            return denied(DenialStage::InvariantExecution);
        }
    };
    let progression = match inspection.admit_invariant_progression([receipt]) {
        Ok(progression) => progression,
        Err(_) => {
            inspection.discard();
            return denied(DenialStage::InvariantExecution);
        }
    };
    let candidate = match inspection.bind_invariant_progression(progression) {
        Ok(candidate) => candidate,
        Err((_, inspection)) => {
            inspection.discard();
            return denied(DenialStage::InvariantExecution);
        }
    };
    let _serialization = provider.serialize_application_commit();
    if admission.validate_current_authority().is_err() {
        candidate.discard();
        return WorthQueryApplicationCommitOutcome::Cancelled;
    }
    match provider.resolve_application_idempotency(&session_identity) {
        Ok(WorthQueryProviderIdempotencyResolution::Absent) => {}
        Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
            candidate.discard();
            return WorthQueryApplicationCommitOutcome::AlreadyCommitted(receipt);
        }
        Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
            candidate.discard();
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
            );
        }
        Err(_) => {
            candidate.discard();
            return denied(DenialStage::Idempotency);
        }
    }
    match candidate.compare_and_commit() {
        WorthQueryProviderCompareAndCommitOutcome::Committed {
            provider_receipt, ..
        } => parse_provider_receipt(&provider_receipt).map_or_else(
            || WorthQueryApplicationCommitOutcome::Indeterminate,
            WorthQueryApplicationCommitOutcome::Committed,
        ),
        WorthQueryProviderCompareAndCommitOutcome::Stale(stale) => {
            WorthQueryApplicationCommitOutcome::Stale(WorthQueryApplicationStaleAttempt::new(
                stale.stale_fact_count(),
            ))
        }
        WorthQueryProviderCompareAndCommitOutcome::Denied(_) => denied(DenialStage::ProviderCommit),
        WorthQueryProviderCompareAndCommitOutcome::Indeterminate(_) => {
            resolve_indeterminate_commit(provider, idempotency)
        }
    }
}

fn resolve_indeterminate_commit(
    provider: &super::super::provider::WorthQueryPrimaryGraphProvider,
    idempotency: WorthQueryApplicationIdempotencyBinding,
) -> WorthQueryApplicationCommitOutcome {
    match provider.resolve_idempotency_binding(idempotency) {
        Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
            WorthQueryApplicationCommitOutcome::Committed(receipt)
        }
        Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
            WorthQueryApplicationCommitOutcome::Aborted
        }
        Ok(WorthQueryProviderIdempotencyResolution::Drift) | Err(_) => {
            WorthQueryApplicationCommitOutcome::Indeterminate
        }
    }
}
