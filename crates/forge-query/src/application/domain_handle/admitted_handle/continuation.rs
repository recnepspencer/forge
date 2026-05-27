use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    ForgeQueryContinuationBindingRequest, ForgeQueryResolveContinuationFromTargetRequest,
};
use crate::continuation_pipeline::{
    execute_prepared_continuation_on_handle, ordinary_outcome_from_continuation_checked,
    ordinary_outcome_from_execution_checked, prepare_continuation_from_context_on_handle,
    prepare_continuation_from_target_on_handle, ForgeQueryContinuationExecution,
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionOutcome,
    ForgeQueryContinuationExecutionTranscript, ForgeQueryExecutePreparedContinuationRequest,
    ForgeQueryPreparedContinuation, ForgeQueryPreparedContinuationChecked,
    ForgeQueryPreparedContinuationOutcome, ForgeQueryPreparedContinuationTranscript,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::ForgeQueryAdmittedConfiguredDomainHandle;

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn prepare_continuation_from_target<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryPreparedContinuationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        prepare_continuation_from_target_on_handle(self, request)
            .into_checked()
            .into_outcome()
    }

    pub fn prepare_continuation_from_target_outcome<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryPreparedContinuation<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_continuation_checked(
            prepare_continuation_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub fn prepare_continuation_from_target_checked<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryPreparedContinuationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        prepare_continuation_from_target_on_handle(self, request).into_checked()
    }

    pub fn prepare_continuation_from_target_proof<I>(
        &self,
        request: ForgeQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> ForgeQueryPreparedContinuationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        prepare_continuation_from_target_on_handle(self, request)
    }

    pub fn prepare_continuation_from_context<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryPreparedContinuationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        prepare_continuation_from_context_on_handle(self, request)
            .into_checked()
            .into_outcome()
    }

    pub fn prepare_continuation_from_context_outcome<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryPreparedContinuation<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_continuation_checked(
            prepare_continuation_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn prepare_continuation_from_context_checked<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryPreparedContinuationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        prepare_continuation_from_context_on_handle(self, request).into_checked()
    }

    pub fn prepare_continuation_from_context_proof<I>(
        &self,
        request: ForgeQueryContinuationBindingRequest<D, I>,
    ) -> ForgeQueryPreparedContinuationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        prepare_continuation_from_context_on_handle(self, request)
    }

    pub fn execute_prepared_continuation<I>(
        &self,
        prepared: ForgeQueryPreparedContinuation<D, I>,
    ) -> ForgeQueryContinuationExecutionOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        execute_prepared_continuation_on_handle(
            self,
            ForgeQueryExecutePreparedContinuationRequest::new(prepared),
        )
        .into_checked()
        .into_outcome()
    }

    pub fn execute_prepared_continuation_outcome<I>(
        &self,
        prepared: ForgeQueryPreparedContinuation<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryContinuationExecution<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_execution_checked(
            execute_prepared_continuation_on_handle(
                self,
                ForgeQueryExecutePreparedContinuationRequest::new(prepared),
            )
            .into_checked(),
        )
    }

    pub fn execute_prepared_continuation_checked<I>(
        &self,
        prepared: ForgeQueryPreparedContinuation<D, I>,
    ) -> ForgeQueryContinuationExecutionChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        execute_prepared_continuation_on_handle(
            self,
            ForgeQueryExecutePreparedContinuationRequest::new(prepared),
        )
        .into_checked()
    }

    pub fn execute_prepared_continuation_proof<I>(
        &self,
        prepared: ForgeQueryPreparedContinuation<D, I>,
    ) -> ForgeQueryContinuationExecutionTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
    {
        execute_prepared_continuation_on_handle(
            self,
            ForgeQueryExecutePreparedContinuationRequest::new(prepared),
        )
    }
}
