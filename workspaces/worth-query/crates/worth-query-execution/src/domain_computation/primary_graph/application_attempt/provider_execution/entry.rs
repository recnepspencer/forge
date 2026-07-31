use worth_query_admission::facade::basis::basis_lifecycle;
use worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceAdmissionCounters;
use worth_query_admission::integration::admit_execution_resource_plan;
use worth_query_installation::facade::ApplicationSchema;
use worth_relational::facade::bridge::bridge_snapshot_identity_for_handle;
use worth_runtime_bridge::facade::{
    BridgeDeliveryIntent, BridgeDiagnosticsTier, BridgeReplayMode, BridgeTruthViewSelector,
    HistoricalEvaluationDeclaration, SnapshotReadPacket, TruthBranchIdentity,
};

use super::super::provider_binding::prepare_provider_attempt;
use super::super::{
    provider_recomparison::recover_equivalent_commit_evidence, WorthQueryApplicationCommitDenial,
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationEffectProgram,
    WorthQueryApplicationIdempotencyBinding,
};
use super::progression::execute_provider_progression;
use super::support::{application_resource_request, denied};
use crate::domain_computation::operation_binding::WorthQueryApplicationOperationBindingInput;
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::{
    WorthQueryExecutionBoundOperationAuthority, WorthQueryManagedRunTerminalKind,
    WorthQueryManagedTruthReadRequest,
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
            read_set,
            effects,
            emission_retained_bytes,
            emission_retained_bytes_ceiling,
        } = program;
        let mut admission = read_set.admission;
        let idempotency =
            idempotency.bind_preconditions(admission.mutation_preconditions().identity());
        if admission.validate_current_authority().is_err() {
            return WorthQueryApplicationCommitOutcome::Cancelled;
        }
        let admission_identity = admission.admission_identity();
        {
            let serialization = self.primary_provider.serialize_application_commit();
            let Some(authorization) = admission.authorization_mut() else {
                return denied(DenialStage::DecisionReadSet);
            };
            let proof = match self.authorize_retained_idempotency(
                authorization,
                admission_identity,
                &serialization,
            ) {
                Ok(proof) => proof,
                Err(_) => return denied(DenialStage::DecisionReadSet),
            };
            let resolution = proof.govern(admission_identity, || {
                self.primary_provider
                    .resolve_idempotency_binding(idempotency)
            });
            match resolution {
                Err(()) => return denied(DenialStage::DecisionReadSet),
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Absent)) => {}
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
                    return WorthQueryApplicationCommitOutcome::AlreadyCommitted(
                        WorthQueryApplicationCommitReceipt::from_provider(
                            receipt,
                            recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                            admission.canonical_work(),
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
        let (authorization, commit_authorization) =
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
                installed_operation_fingerprint: admission.retain_installed_operation_fingerprint(),
                resource_binding_identity: admission.retain_resource_binding_identity(),
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
            self,
            &mut running,
            &self.primary_graph_authority,
            &self.primary_provider,
            &admission,
            prepared,
            authorization,
            commit_authorization,
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
