use super::{
    WorthQueryGraphCommitCall, WorthQueryGraphProviderCall, WorthQueryGraphProviderFailure,
    WorthQueryGraphProviderReceipt,
};

pub trait WorthQueryGraphParticipationProvider<G>: Send + Sync + 'static {
    fn execution_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupport;

    fn observe(
        &self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;

    fn project(
        &self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;

    fn touch_effect(
        &self,
        call: &WorthQueryGraphProviderCall,
    ) -> Result<WorthQueryGraphProviderReceipt, WorthQueryGraphProviderFailure>;
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
