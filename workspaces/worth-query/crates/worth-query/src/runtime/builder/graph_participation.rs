use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn graph_participation<G: 'static>(
        mut self,
        definition: crate::domain_installation::WorthQueryGraphParticipationDefinition<G>,
    ) -> Self {
        self.pending_graph_participations =
            self.pending_graph_participations.definition(definition);
        self
    }

    pub fn graph_participation_provider<
        G: 'static,
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>,
    >(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self {
        self.pending_graph_participations = self
            .pending_graph_participations
            .provider::<G, P>(provider, None);
        self
    }

    pub fn session_graph_participation_provider<G: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .session_provider::<G, P>(provider, None);
        self
    }

    pub fn decision_graph_participation_provider<G: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .decision_provider::<G, P>(provider, None);
        self
    }

    pub fn provisional_graph_participation_provider<G: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .provisional_provider::<G, P>(provider, None);
        self
    }

    pub fn invariant_graph_participation_provider<G: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider
            + worth_query_execution::facade::provider_session::WorthQueryInvariantExecutionProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .invariant_provider::<G, P>(provider, None);
        self
    }

    pub fn atomic_invariant_graph_participation_provider<G: 'static, C: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
        _commit: C,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider
            + worth_query_execution::facade::provider_session::WorthQueryInvariantExecutionProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .invariant_provider::<G, P>(
                provider,
                Some((std::any::TypeId::of::<C>(), std::any::type_name::<C>())),
            );
        self
    }

    pub fn convergent_invariant_graph_participation_provider<G: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider
            + worth_query_execution::facade::provider_session::WorthQueryInvariantExecutionProvider
            + worth_query_execution::facade::convergence_epoch::WorthQueryConvergenceDomainProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .convergent_invariant_provider::<G, P>(provider, None);
        self
    }

    pub fn atomic_convergent_invariant_graph_participation_provider<G: 'static, C: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
        _commit: C,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider
            + worth_query_execution::facade::provider_session::WorthQueryInvariantExecutionProvider
            + worth_query_execution::facade::convergence_epoch::WorthQueryConvergenceDomainProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .convergent_invariant_provider::<G, P>(
                provider,
                Some((std::any::TypeId::of::<C>(), std::any::type_name::<C>())),
            );
        self
    }

    pub fn convergent_graph_participation_provider<G: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::convergence_epoch::WorthQueryConvergenceDomainProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .convergent_provider::<G, P>(provider, None);
        self
    }

    pub fn atomic_graph_participation_provider<G: 'static, C: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
        _commit: C,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>,
    {
        self.pending_graph_participations = self.pending_graph_participations.provider::<G, P>(
            provider,
            Some((std::any::TypeId::of::<C>(), std::any::type_name::<C>())),
        );
        self
    }

    pub fn atomic_convergent_graph_participation_provider<G: 'static, C: 'static, P>(
        mut self,
        _marker: G,
        provider: P,
        _commit: C,
    ) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::convergence_epoch::WorthQueryConvergenceDomainProvider,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .convergent_provider::<G, P>(
                provider,
                Some((std::any::TypeId::of::<C>(), std::any::type_name::<C>())),
            );
        self
    }

    pub fn graph_commit_provider<C: 'static, P>(mut self, _commit: C, provider: P) -> Self
    where
        P: crate::domain_installation::WorthQueryGraphCommitProvider<C>,
    {
        self.pending_graph_participations = self
            .pending_graph_participations
            .commit_provider::<C, P>(provider);
        self
    }
}
