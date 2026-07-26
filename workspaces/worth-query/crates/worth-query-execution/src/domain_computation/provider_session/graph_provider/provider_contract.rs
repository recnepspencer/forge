use super::{
    WorthQueryGraphCommitCall, WorthQueryGraphProviderCall, WorthQueryGraphProviderExecution,
    WorthQueryGraphProviderExecutionStart, WorthQueryGraphProviderFailure,
    WorthQueryGraphProviderReceipt,
};

pub trait WorthQueryGraphParticipationProvider<G>: Send + Sync + 'static {
    type Execution: WorthQueryGraphProviderExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport;

    fn begin(
        &self,
        call: &WorthQueryGraphProviderCall,
        start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure>;
}

pub trait WorthQueryGraphCommitProvider<C>: Send + Sync + 'static {
    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport;

    fn admit_commit(
        &self,
        call: &WorthQueryGraphCommitCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;
}
