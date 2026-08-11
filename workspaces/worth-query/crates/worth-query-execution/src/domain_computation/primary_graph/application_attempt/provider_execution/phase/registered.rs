use super::super::super::WorthQueryApplicationIdempotencyBinding;

pub(in crate::domain_computation::primary_graph::application_attempt) struct WorthQueryRegisteredProviderAttempt<
    'run,
> {
    pub(super) staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
    pub(super) requests: Vec<crate::domain_computation::WorthQueryDecisionFactRequest>,
    pub(super) steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
    pub(super) dispatch_outbox:
        Option<crate::domain_computation::application_aftermath::WorthQueryDispatchOutboxRecord>,
}

pub(in crate::domain_computation::primary_graph::application_attempt) struct WorthQueryProviderAttemptRegistrationContext<
    'a,
    Schema,
    Operation,
    Input,
    Scope,
> {
    provider:
        &'a std::sync::Arc<super::super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    aftermath_causality: Option<
        &'a crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

impl<'a, Schema, Operation, Input, Scope>
    WorthQueryProviderAttemptRegistrationContext<'a, Schema, Operation, Input, Scope>
{
    pub(super) fn from_progression(
        authority: &'a super::advance::WorthQueryApplicationCommitProgressionAuthority<
            'a,
            '_,
            Schema,
            Operation,
            Input,
            Scope,
        >,
    ) -> Self {
        Self {
            provider: authority.provider(),
            admission: authority.admission(),
            idempotency: authority.idempotency(),
            aftermath_causality: authority.aftermath_causality(),
        }
    }

    pub(in crate::domain_computation::primary_graph::application_attempt) const fn provider(
        &self,
    ) -> &std::sync::Arc<super::super::super::super::provider::WorthQueryPrimaryGraphProvider> {
        self.provider
    }

    pub(in crate::domain_computation::primary_graph::application_attempt) const fn admission(
        &self,
    ) -> &crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    > {
        self.admission
    }

    pub(in crate::domain_computation::primary_graph::application_attempt) const fn idempotency(
        &self,
    ) -> WorthQueryApplicationIdempotencyBinding {
        self.idempotency
    }

    pub(in crate::domain_computation::primary_graph::application_attempt) const fn aftermath_causality(
        &self,
    ) -> Option<
        &crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    > {
        self.aftermath_causality
    }
}

impl<'run> WorthQueryRegisteredProviderAttempt<'run> {
    pub(in crate::domain_computation::primary_graph::application_attempt) fn from_registration(
        staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
        requests: Vec<crate::domain_computation::WorthQueryDecisionFactRequest>,
        steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
        dispatch_outbox: Option<
            crate::domain_computation::application_aftermath::WorthQueryDispatchOutboxRecord,
        >,
    ) -> Self {
        Self {
            staged,
            requests,
            steps,
            dispatch_outbox,
        }
    }
}
