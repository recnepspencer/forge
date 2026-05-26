use crate::application::{
    derive_signal_compatibility_support_report,
    forge_query_checked_declaration_signal_compatibility_on_handle,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanInput, ForgeQueryDeclarationSignalCompatibility,
    ForgeQueryDeclarationSignalCompatibilityChecked, ForgeQueryDeclarationSignalCompatibilityInput,
    ForgeQueryDeclarationSignalCompatibilitySupportReport,
    ForgeQueryDeclarationSignalCompatibilityTerminalError,
    ForgeQueryDeclarationSupportsSignalCompatibility, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext,
};

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn signal_compatibility<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
    ) -> Result<
        ForgeQueryDeclarationSignalCompatibility<D, I>,
        ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>,
    > {
        match self.signal_compatibility_checked(subject) {
            ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
                Ok(compatibility)
            }
            ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(compatibility) => {
                Err(ForgeQueryDeclarationSignalCompatibilityTerminalError::Deferred(compatibility))
            }
            ForgeQueryDeclarationSignalCompatibilityChecked::Denied(compatibility) => {
                Err(ForgeQueryDeclarationSignalCompatibilityTerminalError::Denied(compatibility))
            }
            ForgeQueryDeclarationSignalCompatibilityChecked::Failed(compatibility) => {
                Err(ForgeQueryDeclarationSignalCompatibilityTerminalError::Failed(compatibility))
            }
        }
    }

    pub fn signal_compatibility_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationSignalCompatibilityInput<D, I>,
    ) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I> {
        let support = self.signal_compatibility_support::<I>();
        forge_query_checked_declaration_signal_compatibility_on_handle(
            self.handle_identity_digest(),
            self.operating_context_identity_digest(),
            support.rows(),
            subject,
        )
    }

    pub fn signal_compatibility_support<I: ForgeQueryDeclarationInput<D>>(
        &self,
    ) -> ForgeQueryDeclarationSignalCompatibilitySupportReport<D, I> {
        derive_signal_compatibility_support_report::<D, C, I>(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn signal_compatibility_from_progressed<I>(
        &self,
        progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        ForgeQueryDeclarationSignalCompatibility<D, I>,
        ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsSignalCompatibility<D>,
    {
        self.signal_compatibility_from_progressed_with_intent(progressed, None)
    }

    pub fn signal_compatibility_from_progressed_with_intent<I>(
        &self,
        progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: impl Into<Option<ForgeQueryDeclarationRouteIntent>>,
    ) -> Result<
        ForgeQueryDeclarationSignalCompatibility<D, I>,
        ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsSignalCompatibility<D>,
    {
        let evidence = self
            .describe_foundational(
                ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                    progressed.clone(),
                ),
            )
            .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));
        let route_checked = match intent.into() {
            Some(intent) => self.plan_routes_checked(
                ForgeQueryDeclarationRoutePlanInput::with_intent(progressed, evidence, intent),
            ),
            None => self.plan_routes_checked(ForgeQueryDeclarationRoutePlanInput::admitted(
                progressed, evidence,
            )),
        };
        let receipt_checked = self.receipt_routes_checked(
            ForgeQueryDeclarationReceiptInput::route_checked(route_checked),
        );
        let envelope_checked = self.envelope_routes_checked(
            ForgeQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
        );
        self.signal_compatibility(
            ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
        )
    }

    pub fn declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility<
        I,
    >(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationSignalCompatibility<D, I>,
        crate::application::ForgeQueryDeclarationEntrySignalCompatibilityError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsSignalCompatibility<D>,
    {
        let progressed = self.declare_review_and_progress(input).map_err(|error| {
            crate::application::ForgeQueryDeclarationEntrySignalCompatibilityError::Entry(
                crate::application::ForgeQueryDeclarationEntryEnvelopeError::Entry(
                    crate::application::ForgeQueryDeclarationEntryReceiptError::Entry(error),
                ),
            )
        })?;
        self.signal_compatibility_from_progressed(progressed)
            .map_err(
            crate::application::ForgeQueryDeclarationEntrySignalCompatibilityError::Compatibility,
        )
    }
}
