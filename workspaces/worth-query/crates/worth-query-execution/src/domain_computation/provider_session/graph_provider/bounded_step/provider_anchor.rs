use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport;

use super::{WorthQueryGraphProviderExecution, WorthQueryGraphProviderExecutionStart};
use crate::domain_computation::{
    WorthQueryGraphParticipationProvider, WorthQueryGraphProviderCall,
    WorthQueryGraphProviderFailure,
};

trait WorthQueryErasedGraphParticipationProvider: Send + Sync {
    fn begin(
        &self,
        call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure>;
}

pub(crate) enum WorthQueryGraphProviderStartInvocation {
    Returned(Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure>),
    Panicked,
}

static NEXT_GRAPH_PROVIDER_GENERATION: AtomicU64 = AtomicU64::new(1);

struct WorthQueryTypedGraphParticipationProvider<G, P> {
    provider: Arc<P>,
    _graph: PhantomData<fn() -> G>,
}

impl<G: 'static, P: WorthQueryGraphParticipationProvider<G>>
    WorthQueryErasedGraphParticipationProvider for WorthQueryTypedGraphParticipationProvider<G, P>
{
    fn begin(
        &self,
        call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure> {
        self.provider
            .begin(call, start)
            .and_then(|admitted| start.validate_returned_execution(admitted))
            .map(|execution| Box::new(execution) as Box<dyn WorthQueryGraphProviderExecution>)
    }
}

pub struct WorthQueryGraphProviderAnchor {
    provider: Arc<dyn WorthQueryErasedGraphParticipationProvider>,
    convergence_provider:
        Option<Arc<dyn crate::domain_computation::WorthQueryConvergenceDomainProvider>>,
    provider_identity: &'static str,
    provider_generation: u64,
    resource_support: WorthQueryExecutionResourceSupport,
}

impl WorthQueryGraphProviderAnchor {
    #[doc(hidden)]
    pub fn install<G: 'static, P: WorthQueryGraphParticipationProvider<G>>(provider: P) -> Self {
        let provider = Arc::new(provider);
        let resource_support = provider.execution_resource_support();
        Self {
            provider: Arc::new(WorthQueryTypedGraphParticipationProvider::<G, P> {
                provider: Arc::clone(&provider),
                _graph: PhantomData,
            }),
            convergence_provider: None,
            provider_identity: std::any::type_name::<P>(),
            provider_generation: NEXT_GRAPH_PROVIDER_GENERATION.fetch_add(1, Ordering::Relaxed),
            resource_support,
        }
    }

    #[doc(hidden)]
    pub fn install_convergent<
        G: 'static,
        P: WorthQueryGraphParticipationProvider<G>
            + crate::domain_computation::WorthQueryConvergenceDomainProvider,
    >(
        provider: P,
    ) -> Self {
        let provider = Arc::new(provider);
        let resource_support = provider.execution_resource_support();
        let convergence_provider: Arc<
            dyn crate::domain_computation::WorthQueryConvergenceDomainProvider,
        > = provider.clone();
        Self {
            provider: Arc::new(WorthQueryTypedGraphParticipationProvider::<G, P> {
                provider,
                _graph: PhantomData,
            }),
            convergence_provider: Some(convergence_provider),
            provider_identity: std::any::type_name::<P>(),
            provider_generation: NEXT_GRAPH_PROVIDER_GENERATION.fetch_add(1, Ordering::Relaxed),
            resource_support,
        }
    }

    #[doc(hidden)]
    pub fn provider_identity(&self) -> &'static str {
        self.provider_identity
    }

    pub(crate) const fn provider_generation(&self) -> u64 {
        self.provider_generation
    }

    #[doc(hidden)]
    pub fn resource_support(&self) -> &WorthQueryExecutionResourceSupport {
        &self.resource_support
    }

    pub(crate) fn begin(
        &self,
        call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> WorthQueryGraphProviderStartInvocation {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.provider.begin(call, start)
        })) {
            Ok(result) => WorthQueryGraphProviderStartInvocation::Returned(result),
            Err(_) => WorthQueryGraphProviderStartInvocation::Panicked,
        }
    }

    pub(crate) fn convergence_provider(
        &self,
    ) -> Option<Arc<dyn crate::domain_computation::WorthQueryConvergenceDomainProvider>> {
        self.convergence_provider.as_ref().map(Arc::clone)
    }
}
