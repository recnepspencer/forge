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
