mod digest_projection;
mod execute;
mod installed_authority;
mod outcome;
mod prepare;
mod readmission;
#[cfg(test)]
mod readmission_test_support;
mod readmission_transcript;
mod support;

use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationSignalExecutionFamily,
    WorthQueryDomainEntryMarker,
};

use super::WorthQueryPreparedContinuation;

pub(crate) use execute::execute_prepared_continuation_on_handle;
pub(crate) use prepare::{
    prepare_continuation_from_context_on_handle,
    prepare_continuation_from_signal_checked_on_handle, prepare_continuation_from_target_on_handle,
};
#[cfg(test)]
pub(crate) use readmission_test_support::drifted_observation_from_retained;

pub struct WorthQueryContinuationExecution<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    prepared: WorthQueryPreparedContinuation<D, I>,
    signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
    bridge_binding_surface: String,
    execution_digest: String,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryContinuationExecution<D, I>
{
    pub(crate) fn new(
        prepared: WorthQueryPreparedContinuation<D, I>,
        signal_execution_family: Option<WorthQueryDeclarationSignalExecutionFamily>,
        bridge_binding_surface: String,
        execution_digest: String,
    ) -> Self {
        Self {
            prepared,
            signal_execution_family,
            bridge_binding_surface,
            execution_digest,
        }
    }

    pub fn prepared(&self) -> &WorthQueryPreparedContinuation<D, I> {
        &self.prepared
    }

    pub fn family(&self) -> super::WorthQueryPreparedContinuationFamily {
        self.prepared.family()
    }

    pub fn signal_execution_family(&self) -> Option<WorthQueryDeclarationSignalExecutionFamily> {
        self.signal_execution_family
    }

    pub fn bridge_binding_surface(&self) -> &str {
        &self.bridge_binding_surface
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }
}
