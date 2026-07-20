use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;

use worth_signal::facade::adapters::FrontierRouteEvidenceReceipt;

use crate::domain_installation::execution_index::WorthQueryWorkflowExecutionDescriptor;

use super::{
    WorthQueryWorkflowParallelAdmissionCall, WorthQueryWorkflowParallelAdmissionFailure,
    WorthQueryWorkflowParallelAdmissionProvider,
};

type ParallelAdmissionMarker<D, O, F> = fn() -> (D, O, F);

trait ErasedParallelAdmissionProvider: Send + Sync {
    fn admit(
        &self,
        call: &WorthQueryWorkflowParallelAdmissionCall,
    ) -> Result<FrontierRouteEvidenceReceipt, WorthQueryWorkflowParallelAdmissionFailure>;
}

struct TypedParallelAdmissionProvider<D, O, F, P> {
    provider: P,
    marker: PhantomData<ParallelAdmissionMarker<D, O, F>>,
}

impl<D, O, F, P> ErasedParallelAdmissionProvider for TypedParallelAdmissionProvider<D, O, F, P>
where
    P: WorthQueryWorkflowParallelAdmissionProvider<D, O, F>,
{
    fn admit(
        &self,
        call: &WorthQueryWorkflowParallelAdmissionCall,
    ) -> Result<FrontierRouteEvidenceReceipt, WorthQueryWorkflowParallelAdmissionFailure> {
        self.provider.admit_parallel_frontier(call)
    }
}

pub(crate) struct WorthQueryInstalledWorkflowParallelAdmissionProvider {
    provider: Arc<dyn ErasedParallelAdmissionProvider>,
}

impl WorthQueryInstalledWorkflowParallelAdmissionProvider {
    pub(crate) fn admit(
        &self,
        call: &WorthQueryWorkflowParallelAdmissionCall,
    ) -> Result<FrontierRouteEvidenceReceipt, WorthQueryWorkflowParallelAdmissionFailure> {
        self.provider.admit(call)
    }
}

#[derive(Default)]
pub(crate) struct WorthQueryPendingWorkflowParallelAdmissionProviders {
    registrations: HashMap<(TypeId, TypeId, TypeId), Arc<dyn ErasedParallelAdmissionProvider>>,
    duplicate: bool,
}

impl WorthQueryPendingWorkflowParallelAdmissionProviders {
    pub(crate) fn register<D: 'static, O: 'static, F: 'static, P>(mut self, provider: P) -> Self
    where
        P: WorthQueryWorkflowParallelAdmissionProvider<D, O, F>,
    {
        let key = (TypeId::of::<D>(), TypeId::of::<O>(), TypeId::of::<F>());
        self.duplicate |= self
            .registrations
            .insert(
                key,
                Arc::new(TypedParallelAdmissionProvider::<D, O, F, P> {
                    provider,
                    marker: PhantomData,
                }),
            )
            .is_some();
        self
    }

    pub(crate) fn install(
        self,
        workflow_operations: &[WorthQueryWorkflowExecutionDescriptor],
    ) -> Result<WorthQueryWorkflowParallelAdmissionProviderRegistry, &'static str> {
        if self.duplicate {
            return Err("duplicate exact workflow parallel-admission provider registration");
        }
        let expected = workflow_operations
            .iter()
            .filter(|descriptor| descriptor.has_parallel_frontier)
            .map(|descriptor| (descriptor.domain, descriptor.operation, descriptor.family))
            .collect::<HashSet<_>>();
        let actual = self.registrations.keys().copied().collect::<HashSet<_>>();
        if expected != actual {
            return Err("parallel workflow operation and parallel-admission provider sets differ");
        }
        Ok(WorthQueryWorkflowParallelAdmissionProviderRegistry {
            registrations: self
                .registrations
                .into_iter()
                .map(|(key, provider)| {
                    (
                        key,
                        Arc::new(WorthQueryInstalledWorkflowParallelAdmissionProvider { provider }),
                    )
                })
                .collect(),
        })
    }
}

pub(crate) struct WorthQueryWorkflowParallelAdmissionProviderRegistry {
    registrations: HashMap<
        (TypeId, TypeId, TypeId),
        Arc<WorthQueryInstalledWorkflowParallelAdmissionProvider>,
    >,
}

impl WorthQueryWorkflowParallelAdmissionProviderRegistry {
    pub(crate) fn get<D: 'static, O: 'static, F: 'static>(
        &self,
    ) -> Option<Arc<WorthQueryInstalledWorkflowParallelAdmissionProvider>> {
        self.registrations
            .get(&(TypeId::of::<D>(), TypeId::of::<O>(), TypeId::of::<F>()))
            .map(Arc::clone)
    }
}
