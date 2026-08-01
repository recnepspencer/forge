use worth_query_installation::facade::ApplicationSchema;
use worth_relational::facade::bridge::bridge_snapshot_identity_for_handle;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, BridgeTruthViewSelector,
    HistoricalEvaluationDeclaration, SnapshotReadPacket,
};

use super::super::provider_binding::prepare_provider_attempt;
use super::super::{
    provider_recomparison::recover_equivalent_commit_evidence, WorthQueryApplicationCommitDenial,
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationEffectProgram,
    WorthQueryApplicationIdempotencyBinding,
};
use super::progression::execute_provider_progression;
use super::support::denied;
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::{
    WorthQueryManagedRunTerminalKind, WorthQueryManagedTruthReadRequest,
};

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
            operation,
            facts,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        } = program;
        let mut progressing = Some(operation);
        let mut managed_progressing = None;
        let mut managed_cleanup = None;
        let branch = progressing
            .as_ref()
            .expect("an application attempt begins in graph-work progression")
            .graph_work()
            .session()
            .branch_affinity()
            .truth_branch()
            .clone();
        let relational_branch = progressing
            .as_ref()
            .expect("an application attempt begins in graph-work progression")
            .graph_work()
            .session()
            .branch_affinity()
            .relational_branch()
            .clone();
        let idempotency = idempotency.bind_preconditions(
            progressing
                .as_ref()
                .expect("an application attempt begins in graph-work progression")
                .admission()
                .mutation_preconditions()
                .identity(),
        );
        let mut outcome = (|| {
            let progressing_attempt = progressing
                .as_mut()
                .expect("pre-managed work remains present until the handoff");
            if progressing_attempt
                .admission()
                .validate_current_authority()
                .is_err()
            {
                return WorthQueryApplicationCommitOutcome::Cancelled;
            }
            {
                let serialization = self.primary_provider.serialize_application_commit();
                let (admission, session) = progressing_attempt.authorization_revalidation_parts();
                let proof =
                    match self.authorize_retained_idempotency(admission, session, &serialization) {
                        Ok(proof) => proof,
                        Err(_) => return denied(DenialStage::DecisionReadSet),
                    };
                let resolution = proof.govern((), |()| {
                    self.primary_provider
                        .resolve_idempotency_binding(idempotency, &relational_branch)
                });
                match resolution {
                    Err(()) => return denied(DenialStage::DecisionReadSet),
                    Ok(Ok(WorthQueryProviderIdempotencyResolution::Absent)) => {}
                    Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
                        return WorthQueryApplicationCommitOutcome::AlreadyCommitted(
                            WorthQueryApplicationCommitReceipt::from_provider(
                                receipt,
                                recover_equivalent_commit_evidence(
                                    progressing_attempt.admission().mutation_preconditions(),
                                ),
                                progressing_attempt.admission().canonical_work(),
                            ),
                        );
                    }
                    Ok(Ok(WorthQueryProviderIdempotencyResolution::Drift)) => {
                        return WorthQueryApplicationCommitOutcome::Denied(
                            WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                        );
                    }
                    Ok(Err(_)) => return denied(DenialStage::Idempotency),
                }
            }
            let (authorization, commit_authorization) = match progressing_attempt
                .admission_mut()
                .take_authorization_dependencies(self.authorization.bridge())
            {
                Ok(authorization) => authorization,
                Err(_) => return denied(DenialStage::DecisionReadSet),
            };
            let prepared = match prepare_provider_attempt(
                facts,
                effects,
                emission_retained_bytes,
                emission_retained_bytes_ceiling,
            ) {
                Ok(prepared) => prepared,
                Err(_) => return denied(DenialStage::ProposalBinding),
            };
            let snapshot = progressing_attempt
                .graph_work()
                .session()
                .basis()
                .snapshot();
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
            let read_request = WorthQueryManagedTruthReadRequest::new(
                snapshot.version_id,
                branch,
                SnapshotReadPacket::new(Vec::new()),
            );
            let request_bridge = self.bridge.fork_managed_request_lane();
            let pre_managed = progressing
                .take()
                .expect("the managed handoff consumes pre-managed progression exactly once");
            let (managed, admitted) = match pre_managed.enter_managed_run(
                &self
                    .runtime
                    .managed_run_admission(&request_bridge, &self.relational_source),
                read_request,
            ) {
                Ok(transition) => transition,
                Err(failure) => {
                    progressing = Some(failure.into_progressing());
                    return denied(DenialStage::ManagedRunAdmission);
                }
            };
            managed_progressing = Some(managed);
            let mut running = admitted.start();
            let outcome = {
                let (admission, graph_work) = managed_progressing
                    .as_mut()
                    .expect("managed progression owns provider execution")
                    .parts_mut();
                execute_provider_progression(
                    self,
                    &mut running,
                    &self.primary_graph_authority,
                    &self.primary_provider,
                    admission,
                    graph_work,
                    prepared,
                    authorization,
                    commit_authorization,
                    idempotency,
                )
            };
            let terminal = if matches!(
                outcome,
                WorthQueryApplicationCommitOutcome::Committed(_)
                    | WorthQueryApplicationCommitOutcome::AlreadyCommitted(_)
            ) {
                WorthQueryManagedRunTerminalKind::Completed
            } else {
                WorthQueryManagedRunTerminalKind::Failed
            };
            managed_cleanup = match running.terminate_for_convergence(terminal).cleanup() {
                Ok(cleanup) => Some(cleanup),
                Err(_) => return WorthQueryApplicationCommitOutcome::Indeterminate,
            };
            outcome
        })();
        let release = match (managed_progressing, managed_cleanup) {
            (Some(managed), Some(cleanup)) => {
                let released =
                    if matches!(outcome, WorthQueryApplicationCommitOutcome::Committed(_)) {
                        managed.finish_mutation(&cleanup)
                    } else {
                        managed.abort(&cleanup)
                    };
                match released {
                    Ok(release) => release,
                    Err(_) => return WorthQueryApplicationCommitOutcome::Indeterminate,
                }
            }
            (Some(_), None) => return WorthQueryApplicationCommitOutcome::Indeterminate,
            (None, _) => progressing
                .expect("a pre-managed outcome retains graph-work progression")
                .abort(),
        };
        if !release.basis_released() || release.capacity().released_reservation_count() == 0 {
            return WorthQueryApplicationCommitOutcome::Indeterminate;
        }
        outcome.attach_graph_work(&release);
        outcome
    }
}
