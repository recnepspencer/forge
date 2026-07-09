use crate::application::{
    derive_signal_compatibility_support_report,
    worth_query_checked_declaration_signal_compatibility_on_handle,
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationEnvelopeInput,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptInput, WorthQueryDeclarationRouteIntent,
    WorthQueryDeclarationRoutePlanInput, WorthQueryDeclarationSignalCompatibility,
    WorthQueryDeclarationSignalCompatibilityChecked, WorthQueryDeclarationSignalCompatibilityInput,
    WorthQueryDeclarationSignalCompatibilitySupportReport,
    WorthQueryDeclarationSignalCompatibilityTerminalError,
    WorthQueryDeclarationSupportsSignalCompatibility, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn signal_compatibility<I: WorthQueryDeclarationInput<D>>(
        &self,
        subject: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
    ) -> Result<
        WorthQueryDeclarationSignalCompatibility<D, I>,
        WorthQueryDeclarationSignalCompatibilityTerminalError<D, I>,
    > {
        match self.signal_compatibility_checked(subject) {
            WorthQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
                Ok(compatibility)
            }
            WorthQueryDeclarationSignalCompatibilityChecked::Deferred(compatibility) => {
                Err(WorthQueryDeclarationSignalCompatibilityTerminalError::Deferred(compatibility))
            }
            WorthQueryDeclarationSignalCompatibilityChecked::Denied(compatibility) => {
                Err(WorthQueryDeclarationSignalCompatibilityTerminalError::Denied(compatibility))
            }
            WorthQueryDeclarationSignalCompatibilityChecked::Failed(compatibility) => {
                Err(WorthQueryDeclarationSignalCompatibilityTerminalError::Failed(compatibility))
            }
        }
    }

    pub fn signal_compatibility_checked<I: WorthQueryDeclarationInput<D>>(
        &self,
        subject: WorthQueryDeclarationSignalCompatibilityInput<D, I>,
    ) -> WorthQueryDeclarationSignalCompatibilityChecked<D, I> {
        let support = self.signal_compatibility_support::<I>();
        worth_query_checked_declaration_signal_compatibility_on_handle(
            self.handle_identity_digest(),
            self.operating_context_identity_digest(),
            support.rows(),
            subject,
        )
    }

    pub fn signal_compatibility_support<I: WorthQueryDeclarationInput<D>>(
        &self,
    ) -> WorthQueryDeclarationSignalCompatibilitySupportReport<D, I> {
        derive_signal_compatibility_support_report::<D, C, I>(self)
    }
}

impl<D: WorthQueryDomainEntryMarker, C: WorthQueryDomainOperatingContext<D>>
    WorthQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn signal_compatibility_from_progressed<I>(
        &self,
        progressed: crate::application::WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        WorthQueryDeclarationSignalCompatibility<D, I>,
        WorthQueryDeclarationSignalCompatibilityTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
        I::Family: WorthQueryDeclarationSupportsSignalCompatibility<D>,
    {
        self.signal_compatibility_from_progressed_with_intent(progressed, None)
    }

    pub fn signal_compatibility_from_progressed_with_intent<I>(
        &self,
        progressed: crate::application::WorthQueryAdmittedDeclarationProgression<D, I>,
        intent: impl Into<Option<WorthQueryDeclarationRouteIntent>>,
    ) -> Result<
        WorthQueryDeclarationSignalCompatibility<D, I>,
        WorthQueryDeclarationSignalCompatibilityTerminalError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
        I::Family: WorthQueryDeclarationSupportsSignalCompatibility<D>,
    {
        let evidence = self
            .describe_foundational(
                WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                    progressed.clone(),
                ),
            )
            .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));
        let route_checked = match intent.into() {
            Some(intent) => self.plan_routes_checked(
                WorthQueryDeclarationRoutePlanInput::with_intent(progressed, evidence, intent),
            ),
            None => self.plan_routes_checked(WorthQueryDeclarationRoutePlanInput::admitted(
                progressed, evidence,
            )),
        };
        let receipt_checked = self.receipt_routes_checked(
            WorthQueryDeclarationReceiptInput::route_checked(route_checked),
        );
        let envelope_checked = self.envelope_routes_checked(
            WorthQueryDeclarationEnvelopeInput::receipt_checked(receipt_checked),
        );
        self.signal_compatibility(
            WorthQueryDeclarationSignalCompatibilityInput::envelope_checked(envelope_checked),
        )
    }

    pub fn declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility<
        I,
    >(
        &self,
        input: I,
    ) -> Result<
        WorthQueryDeclarationSignalCompatibility<D, I>,
        crate::application::WorthQueryDeclarationEntrySignalCompatibilityError<D, I>,
    >
    where
        I: WorthQueryDeclarationInput<D>,
        I::Family: WorthQueryDeclarationSupportsSignalCompatibility<D>,
    {
        let progressed = self.declare_review_and_progress(input).map_err(|error| {
            crate::application::WorthQueryDeclarationEntrySignalCompatibilityError::Entry(
                crate::application::WorthQueryDeclarationEntryEnvelopeError::Entry(
                    crate::application::WorthQueryDeclarationEntryReceiptError::Entry(error),
                ),
            )
        })?;
        self.signal_compatibility_from_progressed(progressed)
            .map_err(
            crate::application::WorthQueryDeclarationEntrySignalCompatibilityError::Compatibility,
        )
    }
}
