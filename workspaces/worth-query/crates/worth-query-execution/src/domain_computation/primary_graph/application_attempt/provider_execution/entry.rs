use worth_query_installation::facade::ApplicationSchema;

use super::super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitOutcome,
    WorthQueryApplicationEffectProgram, WorthQueryApplicationIdempotencyBinding,
};
use super::elevation_currentness::WorthQueryElevationCommitCurrentness;
use super::phase::{
    finish_application_commit, prepare_application_commit, progress_application_commit,
    start_managed_application_commit, WorthQueryApplicationCommitPreparation,
    WorthQueryApplicationCommitPreparationRequest,
};
use crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn compare_and_commit_application<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> WorthQueryApplicationCommitOutcome
    where
        Input: Clone + Send + Sync + 'static,
    {
        if program.read_set.admission.has_elevation_lifecycle_binding() {
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::elevation_transition_required(),
            );
        }
        if program
            .read_set
            .admission
            .allowed_graph_contract()
            .execution_posture()
            .requires_delegation_activation()
        {
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::delegation_activation_required(),
            );
        }
        if program
            .read_set
            .admission
            .allowed_graph_contract()
            .execution_posture()
            .requires_capability_revocation()
        {
            return WorthQueryApplicationCommitOutcome::Denied(
                WorthQueryApplicationCommitDenial::capability_revocation_required(),
            );
        }
        self.compare_and_commit_application_inner(program, idempotency)
    }

    pub(super) fn compare_and_commit_application_inner<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
    ) -> WorthQueryApplicationCommitOutcome
    where
        Input: Clone + Send + Sync + 'static,
    {
        self.compare_and_commit_application_inner_with_currentness(program, idempotency, None)
    }

    pub(super) fn compare_and_commit_application_inner_with_currentness<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        elevation_currentness: Option<WorthQueryElevationCommitCurrentness>,
    ) -> WorthQueryApplicationCommitOutcome
    where
        Input: Clone + Send + Sync + 'static,
    {
        self.compare_and_commit_application_inner_with_currentness_and_aftermath(
            program,
            idempotency,
            elevation_currentness,
            None,
        )
    }

    pub(crate) fn compare_and_commit_application_with_aftermath<Operation, Input, Scope>(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        aftermath_causality: WorthQueryPendingAftermathCausality,
    ) -> WorthQueryApplicationCommitOutcome
    where
        Input: Clone + Send + Sync + 'static,
    {
        self.compare_and_commit_application_inner_with_currentness_and_aftermath(
            program,
            idempotency,
            None,
            Some(aftermath_causality),
        )
    }

    fn compare_and_commit_application_inner_with_currentness_and_aftermath<
        Operation,
        Input,
        Scope,
    >(
        &self,
        program: WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope>,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        elevation_currentness: Option<WorthQueryElevationCommitCurrentness>,
        aftermath_causality: Option<WorthQueryPendingAftermathCausality>,
    ) -> WorthQueryApplicationCommitOutcome
    where
        Input: Clone + Send + Sync + 'static,
    {
        let prepared = match prepare_application_commit(
            self,
            WorthQueryApplicationCommitPreparationRequest::new(
                program,
                idempotency,
                elevation_currentness,
                aftermath_causality,
            ),
        ) {
            WorthQueryApplicationCommitPreparation::Ready(prepared) => prepared,
            WorthQueryApplicationCommitPreparation::Terminal(outcome) => return outcome,
        };
        let running = match start_managed_application_commit(self, prepared) {
            Ok(running) => running,
            Err(outcome) => return outcome,
        };
        finish_application_commit(self, progress_application_commit(self, running))
    }
}
