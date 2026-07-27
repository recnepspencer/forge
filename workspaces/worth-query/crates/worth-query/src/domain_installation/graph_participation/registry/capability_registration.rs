use std::any::TypeId;
use std::sync::Arc;

use worth_query_execution::facade::integration::WorthQueryGraphProviderAnchor;

use super::WorthQueryPendingGraphParticipations;
use crate::domain_installation::WorthQueryGraphParticipationProvider;

impl WorthQueryPendingGraphParticipations {
    pub(crate) fn session_provider<G: 'static, P>(
        self,
        provider: P,
        commit_marker: Option<(TypeId, &'static str)>,
    ) -> Self
    where
        P: WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle,
    {
        let provider =
            Arc::new(WorthQueryGraphProviderAnchor::install_session_capable::<G, P>(provider));
        self.register_provider::<G>(provider, commit_marker)
    }

    pub(crate) fn decision_provider<G: 'static, P>(
        self,
        provider: P,
        commit_marker: Option<(TypeId, &'static str)>,
    ) -> Self
    where
        P: WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider,
    {
        let provider = Arc::new(WorthQueryGraphProviderAnchor::install_decision_capable::<
            G,
            P,
        >(provider));
        self.register_provider::<G>(provider, commit_marker)
    }

    pub(crate) fn provisional_provider<G: 'static, P>(
        self,
        provider: P,
        commit_marker: Option<(TypeId, &'static str)>,
    ) -> Self
    where
        P: WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider,
    {
        let provider =
            Arc::new(WorthQueryGraphProviderAnchor::install_provisional_capable::<G, P>(provider));
        self.register_provider::<G>(provider, commit_marker)
    }

    pub(crate) fn invariant_provider<G: 'static, P>(
        self,
        provider: P,
        commit_marker: Option<(TypeId, &'static str)>,
    ) -> Self
    where
        P: WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider
            + worth_query_execution::facade::provider_session::WorthQueryInvariantExecutionProvider,
    {
        let provider = Arc::new(WorthQueryGraphProviderAnchor::install_invariant_capable::<
            G,
            P,
        >(provider));
        self.register_provider::<G>(provider, commit_marker)
    }

    pub(crate) fn convergent_invariant_provider<G: 'static, P>(
        self,
        provider: P,
        commit_marker: Option<(TypeId, &'static str)>,
    ) -> Self
    where
        P: WorthQueryGraphParticipationProvider<G>
            + worth_query_execution::facade::provider_session::WorthQueryProviderSessionLifecycle
            + worth_query_execution::facade::provider_session::WorthQueryDecisionFactProvider
            + worth_query_execution::facade::provider_session::WorthQueryProvisionalGraphProvider
            + worth_query_execution::facade::provider_session::WorthQueryInvariantExecutionProvider
            + worth_query_execution::facade::convergence_epoch::WorthQueryConvergenceDomainProvider,
    {
        let provider = Arc::new(
            WorthQueryGraphProviderAnchor::install_convergent_invariant_capable::<G, P>(provider),
        );
        self.register_provider::<G>(provider, commit_marker)
    }
}
