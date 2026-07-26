use std::sync::Arc;

use worth_query_admission::facade::resource_admission::{
    WorthQueryExecutionResourceSupport, WorthQueryFixedExecutionCapacity,
};
use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode, WorthQueryResourceLimitRequest,
    WorthQuerySemanticScaleRequest,
};
use worth_query_execution::facade::{
    convergence_epoch::{
        WorthQueryConvergenceAssessment, WorthQueryConvergenceComparison,
        WorthQueryConvergenceDomainFailure, WorthQueryConvergenceDomainProvider,
        WorthQueryConvergenceProgress, WorthQueryConvergenceProviderFamilies,
        WorthQueryConvergenceRepeatedState,
    },
    provider_session::{
        WorthQueryCooperativeGraphProviderExecution, WorthQueryGraphParticipationProvider,
        WorthQueryGraphProviderCall, WorthQueryGraphProviderExecution,
        WorthQueryGraphProviderExecutionStart, WorthQueryGraphProviderFailure,
        WorthQueryGraphProviderStep, WorthQueryGraphProviderStepDisposition,
    },
};
use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionResourceEnvelope,
};

use super::WorthQueryPendingGraphParticipations;

struct TestGraph;
struct TestProvider;
struct TestExecution;

impl WorthQueryGraphProviderExecution for TestExecution {
    fn advance(
        &mut self,
        _step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        unimplemented!()
    }

    fn dispose(&mut self) -> Result<(), WorthQueryGraphProviderFailure> {
        Ok(())
    }
}

impl WorthQueryGraphParticipationProvider<TestGraph> for TestProvider {
    type Execution = TestExecution;

    fn execution_resource_support(&self) -> WorthQueryExecutionResourceSupport {
        execution_support()
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<
        WorthQueryCooperativeGraphProviderExecution<Self::Execution>,
        WorthQueryGraphProviderFailure,
    > {
        unimplemented!()
    }
}

impl WorthQueryConvergenceDomainProvider for TestProvider {
    fn convergence_families(&self) -> &WorthQueryConvergenceProviderFamilies {
        unimplemented!()
    }

    fn compare(
        &self,
        _assessment: &WorthQueryConvergenceAssessment<'_>,
    ) -> Result<WorthQueryConvergenceComparison, WorthQueryConvergenceDomainFailure> {
        unimplemented!()
    }

    fn measure_progress(
        &self,
        _assessment: &WorthQueryConvergenceAssessment<'_>,
        _comparison: &WorthQueryConvergenceComparison,
    ) -> Result<WorthQueryConvergenceProgress, WorthQueryConvergenceDomainFailure> {
        unimplemented!()
    }

    fn detect_repeated_state(
        &self,
        _assessment: &WorthQueryConvergenceAssessment<'_>,
        _comparison: &WorthQueryConvergenceComparison,
        _progress: WorthQueryConvergenceProgress,
    ) -> Result<WorthQueryConvergenceRepeatedState, WorthQueryConvergenceDomainFailure> {
        unimplemented!()
    }
}

#[test]
fn production_registry_accepts_explicit_convergent_provider_registration() {
    let pending = WorthQueryPendingGraphParticipations::default()
        .convergent_provider::<TestGraph, TestProvider>(TestProvider, None);

    let registration = pending
        .providers
        .get(&std::any::TypeId::of::<TestGraph>())
        .expect("convergent provider should enter the production registry");

    assert_eq!(
        registration.provider_identity,
        std::any::type_name::<TestProvider>()
    );
}

fn execution_support() -> WorthQueryExecutionResourceSupport {
    let envelope = WorthQueryExecutionResourceEnvelope::new(
        WorthQuerySemanticScaleRequest::bounded(1),
        WorthQueryResourceLimitRequest::bounded(1),
        WorthQueryExecutionMode::Synchronous,
        None,
        WorthQueryCancellationSafePointFamily::new("registry-test-step").unwrap(),
    );
    WorthQueryExecutionResourceSupport::new(
        WorthQueryExecutionProviderFamily::new("registry-test-provider").unwrap(),
        WorthQueryExecutionAccessProductFamily::new("registry-test-access").unwrap(),
        WorthQueryExecutionAllocatorFamily::new("registry-test-allocator").unwrap(),
        envelope,
        Arc::new(WorthQueryFixedExecutionCapacity::new("registry-test", 1).unwrap()),
    )
}
