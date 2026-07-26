use worth_query_execution::facade::domain_computation::{
    WorthQueryGraphProviderExecution, WorthQueryGraphProviderFailure, WorthQueryGraphProviderStep,
    WorthQueryGraphProviderStepDisposition,
};

struct MissingDisposalContract;

impl WorthQueryGraphProviderExecution for MissingDisposalContract {
    fn advance(
        &mut self,
        _step: &mut WorthQueryGraphProviderStep,
    ) -> Result<WorthQueryGraphProviderStepDisposition, WorthQueryGraphProviderFailure> {
        unreachable!()
    }
}

fn main() {}
