mod execute;
mod prepare;
mod support;

use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationSignalExecutionFamily,
    ForgeQueryDomainEntryMarker,
};

use super::ForgeQueryPreparedContinuation;

pub(crate) use execute::execute_prepared_continuation_on_handle;
pub(crate) use prepare::{
    prepare_continuation_from_context_on_handle, prepare_continuation_from_target_on_handle,
};

pub struct ForgeQueryContinuationExecution<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    prepared: ForgeQueryPreparedContinuation<D, I>,
    signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
    bridge_binding_surface: String,
    execution_digest: String,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryContinuationExecution<D, I>
{
    pub(crate) fn new(
        prepared: ForgeQueryPreparedContinuation<D, I>,
        signal_execution_family: Option<ForgeQueryDeclarationSignalExecutionFamily>,
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

    pub fn prepared(&self) -> &ForgeQueryPreparedContinuation<D, I> {
        &self.prepared
    }

    pub fn family(&self) -> super::ForgeQueryPreparedContinuationFamily {
        self.prepared.family()
    }

    pub fn signal_execution_family(&self) -> Option<ForgeQueryDeclarationSignalExecutionFamily> {
        self.signal_execution_family
    }

    pub fn bridge_binding_surface(&self) -> &str {
        &self.bridge_binding_surface
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }
}
