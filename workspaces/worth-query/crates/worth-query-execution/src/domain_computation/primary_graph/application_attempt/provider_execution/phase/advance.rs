use super::super::outcome::WorthQueryProviderProgressionOutcome;
use super::authorized::{authorize_provider_commit, resolve_authorized_provider_commit};
use super::fresh::{compare_provider_read_set, WorthQueryProviderReadSetProgression};
use super::invariant::progress_invariant_candidate;
use super::progressed::WorthQueryProgressedApplicationCommit;
use super::registered::{
    WorthQueryProviderAttemptRegistrationContext, WorthQueryRegisteredProviderAttempt,
};
use super::running::WorthQueryRunningApplicationCommit;
use super::session_admission::admit_provider_session;

struct WorthQueryProviderProgression<'a, 'provider, Schema, Operation, Input, Scope> {
    application:
        &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    running: &'a mut crate::domain_computation::WorthQueryRunningDirectRun,
    graph: &'a worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    provider:
        &'a std::sync::Arc<super::super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    prepared: super::super::super::provider_binding::WorthQueryPreparedApplicationProviderAttempt,
    authorization:
        crate::domain_computation::authorization::WorthQueryProviderAuthorizationDecisionFacts,
    commit_authorization:
        crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    idempotency: super::super::super::WorthQueryApplicationIdempotencyBinding,
    serialization:
        &'a super::super::super::super::provider::WorthQueryApplicationCommitSerialization<
            'provider,
        >,
    aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

pub(super) struct WorthQueryApplicationCommitProgressionAuthority<
    'a,
    'provider,
    Schema,
    Operation,
    Input,
    Scope,
> {
    application:
        &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    provider:
        &'a std::sync::Arc<super::super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    commit_authorization:
        crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    idempotency: super::super::super::WorthQueryApplicationIdempotencyBinding,
    serialization:
        &'a super::super::super::super::provider::WorthQueryApplicationCommitSerialization<
            'provider,
        >,
    aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

impl<'a, 'provider, Schema, Operation, Input, Scope>
    WorthQueryApplicationCommitProgressionAuthority<'a, 'provider, Schema, Operation, Input, Scope>
{
    pub(super) fn application(
        &self,
    ) -> &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    > {
        self.application
    }

    pub(super) fn provider(
        &self,
    ) -> &'a std::sync::Arc<super::super::super::super::provider::WorthQueryPrimaryGraphProvider>
    {
        self.provider
    }

    pub(super) fn admission(
        &self,
    ) -> &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    > {
        self.admission
    }

    pub(super) fn commit_authorization(
        &self,
    ) -> &crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis {
        &self.commit_authorization
    }

    pub(super) fn idempotency(
        &self,
    ) -> super::super::super::WorthQueryApplicationIdempotencyBinding {
        self.idempotency
    }

    pub(super) fn serialization(
        &self,
    ) -> &'a super::super::super::super::provider::WorthQueryApplicationCommitSerialization<'provider>
    {
        self.serialization
    }

    pub(super) fn aftermath_causality(
        &self,
    ) -> Option<
        &'_ crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    > {
        self.aftermath_causality.as_ref()
    }
}

pub(in super::super) fn progress_application_commit<Schema, Operation, Input, Scope>(
    application: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    running_commit: WorthQueryRunningApplicationCommit<Schema, Operation, Input, Scope>,
) -> WorthQueryProgressedApplicationCommit
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let WorthQueryRunningApplicationCommit {
        admission,
        lease,
        provider_attempt,
        authorization,
        commit_authorization,
        idempotency,
        mut running,
        mutation_run,
        aftermath_causality,
    } = running_commit;
    let serialization = application.primary_provider.serialize_application_commit();
    let (outcome, cleanup) = progress_provider_application(
        WorthQueryProviderProgression {
            application,
            running: &mut running,
            graph: &application.primary_graph_authority,
            provider: &application.primary_provider,
            admission: &admission,
            prepared: provider_attempt,
            authorization,
            commit_authorization,
            idempotency,
            serialization: &serialization,
            aftermath_causality,
        },
        mutation_run,
    );
    WorthQueryProgressedApplicationCommit::new(outcome, lease, running, cleanup)
}

fn progress_provider_application<Schema, Operation, Input, Scope>(
    progression: WorthQueryProviderProgression<'_, '_, Schema, Operation, Input, Scope>,
    mutation_run: crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
) -> (
    WorthQueryProviderProgressionOutcome,
    super::mutation_cleanup::WorthQueryApplicationMutationCleanupOwner,
)
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let WorthQueryProviderProgression {
        application,
        running,
        graph,
        provider,
        admission,
        prepared,
        authorization,
        commit_authorization,
        idempotency,
        serialization,
        aftermath_causality,
    } = progression;
    let (staged, mutation_run) = match admit_provider_session(running, graph, mutation_run) {
        Ok(admitted) => admitted,
        Err((outcome, mutation_run)) => {
            return (
                outcome,
                super::mutation_cleanup::WorthQueryApplicationMutationCleanupOwner::Unbound(
                    mutation_run,
                ),
            )
        }
    };
    let authority = WorthQueryApplicationCommitProgressionAuthority {
        application,
        provider,
        admission,
        commit_authorization,
        idempotency,
        serialization,
        aftermath_causality,
    };
    let registered =
        match prepared.register(
            staged,
            authorization,
            WorthQueryProviderAttemptRegistrationContext::from_progression(&authority),
        ) {
            Ok(registered) => registered,
            Err(outcome) => return (
                outcome,
                super::mutation_cleanup::WorthQueryApplicationMutationCleanupOwner::ProviderBound(
                    mutation_run,
                ),
            ),
        };
    (
        progress_registered_attempt(&authority, registered),
        super::mutation_cleanup::WorthQueryApplicationMutationCleanupOwner::ProviderBound(
            mutation_run,
        ),
    )
}

fn progress_registered_attempt<Schema, Operation, Input, Scope>(
    authority: &WorthQueryApplicationCommitProgressionAuthority<
        '_,
        '_,
        Schema,
        Operation,
        Input,
        Scope,
    >,
    registered: WorthQueryRegisteredProviderAttempt<'_>,
) -> WorthQueryProviderProgressionOutcome
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let WorthQueryRegisteredProviderAttempt {
        staged,
        requests,
        steps,
        dispatch_outbox,
    } = registered;
    let read_set = compare_provider_read_set(staged, requests, authority);
    let (staged, fresh) = match read_set {
        WorthQueryProviderReadSetProgression::Fresh(fresh) => (fresh.staged, fresh.read_set),
        WorthQueryProviderReadSetProgression::Terminal(outcome) => return outcome,
    };
    let candidate = match progress_invariant_candidate(staged, fresh, steps) {
        Ok(candidate) => candidate,
        Err(outcome) => return outcome,
    };
    resolve_authorized_provider_commit(authorize_provider_commit(
        candidate,
        authority,
        dispatch_outbox,
    ))
}
