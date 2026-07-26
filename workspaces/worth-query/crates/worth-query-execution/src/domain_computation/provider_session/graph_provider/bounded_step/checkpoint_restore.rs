use super::WorthQueryGraphProviderExecution;
use crate::domain_computation::WorthQueryGraphProviderFailure;

pub(crate) enum WorthQueryProviderCheckpointRestoreInvocation {
    Returned(Result<Box<dyn WorthQueryGraphProviderExecution>, WorthQueryGraphProviderFailure>),
    Panicked,
}
