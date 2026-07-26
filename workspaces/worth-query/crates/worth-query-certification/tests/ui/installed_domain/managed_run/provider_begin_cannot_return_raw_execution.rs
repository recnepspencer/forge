use worth_query_execution::facade::provider_session::{
    WorthQueryGraphParticipationProvider, WorthQueryGraphProviderCall,
    WorthQueryGraphProviderExecution, WorthQueryGraphProviderExecutionStart,
    WorthQueryGraphProviderFailure, WorthQueryGraphProviderStep,
    WorthQueryGraphProviderStepDisposition,
};

struct Graph;
struct RawProvider;
struct RawExecution;

impl WorthQueryGraphProviderExecution for RawExecution {
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

impl WorthQueryGraphParticipationProvider<Graph> for RawProvider {
    type Execution = RawExecution;

    fn execution_resource_support(
        &self,
    ) -> worth_query_host::facade::admission::resource_admission::WorthQueryExecutionResourceSupport
    {
        unimplemented!()
    }

    fn begin(
        &self,
        _call: &WorthQueryGraphProviderCall,
        _start: &mut WorthQueryGraphProviderExecutionStart,
    ) -> Result<Self::Execution, WorthQueryGraphProviderFailure> {
        Ok(RawExecution)
    }
}

fn main() {}
