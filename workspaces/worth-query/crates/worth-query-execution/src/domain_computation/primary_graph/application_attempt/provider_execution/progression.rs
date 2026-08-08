use super::application_attempt_registration::{
    register_provider_attempt, WorthQueryProviderAttemptRegistrationContext,
    WorthQueryRegisteredProviderAttempt,
};
use super::authorized_progression::{
    resolve_authorized_provider_commit, WorthQueryAuthorizedProviderCommitContext,
};
use super::commit_completion::WorthQueryProgressedApplicationCommit;
use super::invariant_progression::progress_invariant_candidate;
use super::managed_commit_run::WorthQueryRunningApplicationCommit;
use super::outcome::WorthQueryProviderProgressionOutcome;
use super::provider_session_admission::admit_provider_session;
use super::read_set_progression::{
    compare_provider_read_set, WorthQueryProviderReadSetContext,
    WorthQueryProviderReadSetProgression,
};

struct WorthQueryProviderProgression<'a, 'provider, Schema, Operation, Input, Scope> {
    application:
        &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    running: &'a mut crate::domain_computation::WorthQueryRunningDirectRun,
    graph: &'a worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    provider: &'a std::sync::Arc<super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    prepared: super::super::provider_binding::WorthQueryPreparedApplicationProviderAttempt,
    authorization:
        crate::domain_computation::authorization::WorthQueryProviderAuthorizationDecisionFacts,
    commit_authorization:
        crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    idempotency: super::super::WorthQueryApplicationIdempotencyBinding,
    mutation_run: &'a crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
    serialization:
        &'a super::super::super::provider::WorthQueryApplicationCommitSerialization<'provider>,
    aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

struct WorthQueryRegisteredProgressionContext<'a, 'provider, Schema, Operation, Input, Scope> {
    application:
        &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    provider: &'a std::sync::Arc<super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    commit_authorization:
        crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    idempotency: super::super::WorthQueryApplicationIdempotencyBinding,
    serialization:
        &'a super::super::super::provider::WorthQueryApplicationCommitSerialization<'provider>,
    aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

pub(super) fn execute_provider_progression<Schema, Operation, Input, Scope>(
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
    let outcome = progress_provider_application(WorthQueryProviderProgression {
        application,
        running: &mut running,
        graph: &application.primary_graph_authority,
        provider: &application.primary_provider,
        admission: &admission,
        prepared: provider_attempt,
        authorization,
        commit_authorization,
        idempotency,
        mutation_run: &mutation_run,
        serialization: &serialization,
        aftermath_causality,
    });
    WorthQueryProgressedApplicationCommit {
        outcome,
        lease,
        running,
        mutation_run,
    }
}

fn progress_provider_application<Schema, Operation, Input, Scope>(
    progression: WorthQueryProviderProgression<'_, '_, Schema, Operation, Input, Scope>,
) -> WorthQueryProviderProgressionOutcome
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
        mutation_run,
        serialization,
        aftermath_causality,
    } = progression;
    let staged = match admit_provider_session(running, graph, mutation_run) {
        Ok(staged) => staged,
        Err(outcome) => return outcome,
    };
    let registered = match register_provider_attempt(
        staged,
        prepared,
        authorization,
        WorthQueryProviderAttemptRegistrationContext {
            provider,
            admission,
            idempotency,
            aftermath_causality: aftermath_causality.as_ref(),
        },
    ) {
        Ok(registered) => registered,
        Err(outcome) => return outcome,
    };
    progress_registered_attempt(
        WorthQueryRegisteredProgressionContext {
            application,
            provider,
            admission,
            commit_authorization,
            idempotency,
            serialization,
            aftermath_causality,
        },
        registered,
    )
}

fn progress_registered_attempt<Schema, Operation, Input, Scope>(
    progression: WorthQueryRegisteredProgressionContext<'_, '_, Schema, Operation, Input, Scope>,
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
        session_identity,
        dispatch_outbox,
    } = registered;
    let read_set = compare_provider_read_set(
        staged,
        requests,
        WorthQueryProviderReadSetContext {
            application: progression.application,
            provider: progression.provider,
            admission: progression.admission,
            commit_authorization: &progression.commit_authorization,
            session_identity: &session_identity,
            serialization: progression.serialization,
            idempotency: progression.idempotency,
        },
    );
    let (staged, fresh) = match read_set {
        WorthQueryProviderReadSetProgression::Fresh { staged, read_set } => (staged, read_set),
        WorthQueryProviderReadSetProgression::Terminal(outcome) => return outcome,
    };
    let candidate = match progress_invariant_candidate(staged, fresh, steps) {
        Ok(candidate) => candidate,
        Err(outcome) => return outcome,
    };
    resolve_authorized_provider_commit(
        candidate,
        WorthQueryAuthorizedProviderCommitContext {
            application: progression.application,
            provider: progression.provider,
            admission: progression.admission,
            commit_authorization: &progression.commit_authorization,
            serialization: progression.serialization,
            idempotency: progression.idempotency,
            session_identity: &session_identity,
            dispatch_outbox,
            aftermath_causality: progression.aftermath_causality.as_ref(),
        },
    )
}
