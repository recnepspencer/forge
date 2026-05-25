use crate::application::{
    forge_query_checked_declaration_relational_routing_on_handle,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRelationalRouting,
    ForgeQueryDeclarationRelationalRoutingChecked, ForgeQueryDeclarationRelationalRoutingInput,
    ForgeQueryDeclarationRelationalRoutingSupportReport,
    ForgeQueryDeclarationRelationalRoutingTerminalError,
    ForgeQueryDeclarationRelationalTruthRoutingSupportStatus, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanInput, ForgeQueryDeclarationSupportsRelationalTruth,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn route_relational_truth<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationRelationalRoutingInput<D, I>,
    ) -> Result<
        ForgeQueryDeclarationRelationalRouting<D, I>,
        ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>,
    > {
        match self.route_relational_truth_checked(subject) {
            ForgeQueryDeclarationRelationalRoutingChecked::Routed(routing) => Ok(routing),
            ForgeQueryDeclarationRelationalRoutingChecked::Deferred(routing) => {
                Err(ForgeQueryDeclarationRelationalRoutingTerminalError::Deferred(routing))
            }
            ForgeQueryDeclarationRelationalRoutingChecked::Denied(routing) => Err(
                ForgeQueryDeclarationRelationalRoutingTerminalError::Denied(routing),
            ),
            ForgeQueryDeclarationRelationalRoutingChecked::Failed(routing) => Err(
                ForgeQueryDeclarationRelationalRoutingTerminalError::Failed(routing),
            ),
        }
    }

    pub fn route_relational_truth_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationRelationalRoutingInput<D, I>,
    ) -> ForgeQueryDeclarationRelationalRoutingChecked<D, I> {
        let support_status = self
            .relational_truth_support::<I>()
            .rows()
            .first()
            .map(|row| row.status())
            .unwrap_or(ForgeQueryDeclarationRelationalTruthRoutingSupportStatus::Unsupported);
        forge_query_checked_declaration_relational_routing_on_handle(
            self.handle_identity_digest(),
            self.operating_context_identity_digest(),
            support_status,
            subject,
        )
    }

    pub fn relational_truth_support<I: ForgeQueryDeclarationInput<D>>(
        &self,
    ) -> ForgeQueryDeclarationRelationalRoutingSupportReport<D, I> {
        crate::application::derive_relational_routing_support_report::<D, C, I>(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn route_relational_truth_from_progressed<I>(
        &self,
        progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        ForgeQueryDeclarationRelationalRouting<D, I>,
        ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsRelationalTruth<D>,
    {
        self.route_relational_truth_from_progressed_with_intent(progressed, None)
    }

    pub fn route_relational_truth_from_progressed_with_intent<I>(
        &self,
        progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: impl Into<Option<ForgeQueryDeclarationRouteIntent>>,
    ) -> Result<
        ForgeQueryDeclarationRelationalRouting<D, I>,
        ForgeQueryDeclarationRelationalRoutingTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsRelationalTruth<D>,
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
        match self.route_relational_truth_checked(
            ForgeQueryDeclarationRelationalRoutingInput::envelope_checked(envelope_checked),
        ) {
            ForgeQueryDeclarationRelationalRoutingChecked::Routed(routing) => Ok(routing),
            ForgeQueryDeclarationRelationalRoutingChecked::Deferred(routing) => {
                Err(ForgeQueryDeclarationRelationalRoutingTerminalError::Deferred(routing))
            }
            ForgeQueryDeclarationRelationalRoutingChecked::Denied(routing) => Err(
                ForgeQueryDeclarationRelationalRoutingTerminalError::Denied(routing),
            ),
            ForgeQueryDeclarationRelationalRoutingChecked::Failed(routing) => Err(
                ForgeQueryDeclarationRelationalRoutingTerminalError::Failed(routing),
            ),
        }
    }

    pub fn declare_review_progress_describe_plan_receipt_envelope_and_route_relational_truth<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationRelationalRouting<D, I>,
        crate::application::ForgeQueryDeclarationEntryRelationalRoutingError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsRelationalTruth<D>,
    {
        let progressed = self.declare_review_and_progress(input).map_err(|error| {
            crate::application::ForgeQueryDeclarationEntryRelationalRoutingError::Entry(
                crate::application::ForgeQueryDeclarationEntryEnvelopeError::Entry(
                    crate::application::ForgeQueryDeclarationEntryReceiptError::Entry(error),
                ),
            )
        })?;
        self.route_relational_truth_from_progressed(progressed)
            .map_err(crate::application::ForgeQueryDeclarationEntryRelationalRoutingError::Routing)
    }
}
