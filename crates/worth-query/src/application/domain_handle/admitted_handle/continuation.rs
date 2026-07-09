use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    WorthQueryContinuationBindingRequest, WorthQueryResolveContinuationFromTargetRequest,
};
use crate::continuation_pipeline::{
    execute_prepared_continuation_on_handle, ordinary_outcome_from_continuation_checked,
    ordinary_outcome_from_execution_checked, prepare_continuation_from_context_on_handle,
    prepare_continuation_from_target_on_handle, WorthQueryContinuationExecution,
    WorthQueryContinuationExecutionChecked, WorthQueryContinuationExecutionOutcome,
    WorthQueryContinuationExecutionTranscript, WorthQueryExecutePreparedContinuationRequest,
    WorthQueryPreparedContinuation, WorthQueryPreparedContinuationChecked,
    WorthQueryPreparedContinuationOutcome, WorthQueryPreparedContinuationTranscript,
};
use crate::ordinary_outcome::WorthQueryOrdinaryOutcome;

use super::WorthQueryAdmittedConfiguredDomainHandle;

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn prepare_continuation_from_target<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryPreparedContinuationOutcome<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        prepare_continuation_from_target_on_handle(self, request)
            .into_checked()
            .into_outcome()
    }

    pub fn prepare_continuation_from_target_outcome<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryPreparedContinuation<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_continuation_checked(
            prepare_continuation_from_target_on_handle(self, request).into_checked(),
        )
    }

    pub fn prepare_continuation_from_target_checked<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryPreparedContinuationChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        prepare_continuation_from_target_on_handle(self, request).into_checked()
    }

    pub fn prepare_continuation_from_target_proof<I>(
        &self,
        request: WorthQueryResolveContinuationFromTargetRequest<D, I>,
    ) -> WorthQueryPreparedContinuationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        prepare_continuation_from_target_on_handle(self, request)
    }

    pub fn prepare_continuation_from_context<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryPreparedContinuationOutcome<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        prepare_continuation_from_context_on_handle(self, request)
            .into_checked()
            .into_outcome()
    }

    pub fn prepare_continuation_from_context_outcome<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryPreparedContinuation<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_continuation_checked(
            prepare_continuation_from_context_on_handle(self, request).into_checked(),
        )
    }

    pub fn prepare_continuation_from_context_checked<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryPreparedContinuationChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        prepare_continuation_from_context_on_handle(self, request).into_checked()
    }

    pub fn prepare_continuation_from_context_proof<I>(
        &self,
        request: WorthQueryContinuationBindingRequest<D, I>,
    ) -> WorthQueryPreparedContinuationTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        prepare_continuation_from_context_on_handle(self, request)
    }

    pub fn execute_prepared_continuation<I>(
        &self,
        prepared: WorthQueryPreparedContinuation<D, I>,
    ) -> WorthQueryContinuationExecutionOutcome<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        execute_prepared_continuation_on_handle(
            self,
            WorthQueryExecutePreparedContinuationRequest::new(prepared),
        )
        .into_checked()
        .into_outcome()
    }

    pub fn execute_prepared_continuation_outcome<I>(
        &self,
        prepared: WorthQueryPreparedContinuation<D, I>,
    ) -> WorthQueryOrdinaryOutcome<WorthQueryContinuationExecution<D, I>>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        ordinary_outcome_from_execution_checked(
            execute_prepared_continuation_on_handle(
                self,
                WorthQueryExecutePreparedContinuationRequest::new(prepared),
            )
            .into_checked(),
        )
    }

    pub fn execute_prepared_continuation_checked<I>(
        &self,
        prepared: WorthQueryPreparedContinuation<D, I>,
    ) -> WorthQueryContinuationExecutionChecked<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        execute_prepared_continuation_on_handle(
            self,
            WorthQueryExecutePreparedContinuationRequest::new(prepared),
        )
        .into_checked()
    }

    pub fn execute_prepared_continuation_proof<I>(
        &self,
        prepared: WorthQueryPreparedContinuation<D, I>,
    ) -> WorthQueryContinuationExecutionTranscript<D, I>
    where
        I: WorthQueryDeclarationInput<D>,
    {
        execute_prepared_continuation_on_handle(
            self,
            WorthQueryExecutePreparedContinuationRequest::new(prepared),
        )
    }
}
