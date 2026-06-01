use crate::application::{
    derive_bridge_routing_support_report, forge_query_checked_declaration_bridge_routing_on_handle,
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAuthorityAspectMismatch, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingInput,
    ForgeQueryDeclarationBridgeRoutingSupportReport,
    ForgeQueryDeclarationBridgeRoutingSupportStatus,
    ForgeQueryDeclarationBridgeRoutingTerminalError, ForgeQueryDeclarationEnvelopeInput,
    ForgeQueryDeclarationFoundationalEvidenceInput, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanInput, ForgeQueryDeclarationSupportsBridgeContinuation,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn route_bridge_continuation<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
    ) -> Result<
        ForgeQueryDeclarationBridgeRouting<D, I>,
        ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>,
    > {
        match self.route_bridge_continuation_checked(subject) {
            ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => Ok(routing),
            ForgeQueryDeclarationBridgeRoutingChecked::Deferred(routing) => Err(
                ForgeQueryDeclarationBridgeRoutingTerminalError::Deferred(routing),
            ),
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(routing) => Err(
                ForgeQueryDeclarationBridgeRoutingTerminalError::Denied(routing),
            ),
            ForgeQueryDeclarationBridgeRoutingChecked::Failed(routing) => Err(
                ForgeQueryDeclarationBridgeRoutingTerminalError::Failed(routing),
            ),
        }
    }

    pub fn route_bridge_continuation_checked<I: ForgeQueryDeclarationInput<D>>(
        &self,
        subject: ForgeQueryDeclarationBridgeRoutingInput<D, I>,
    ) -> ForgeQueryDeclarationBridgeRoutingChecked<D, I> {
        let support_status = self
            .bridge_continuation_support::<I>()
            .rows()
            .first()
            .map(|row| {
                if row.status() == ForgeQueryDeclarationBridgeRoutingSupportStatus::Unsupported
                    && (matches!(
                        row.aspect_mismatch(),
                        Some(
                            ForgeQueryDeclarationAuthorityAspectMismatch::MissingRequiredAspect
                                | ForgeQueryDeclarationAuthorityAspectMismatch::AspectConflict
                                | ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap
                                | ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectAmbiguity
                        )
                    ) || !matches!(
                        row.mapping_fit(),
                        ForgeQueryDeclarationAspectFit::Exact
                            | ForgeQueryDeclarationAspectFit::CompatibleSuperset
                    ))
                {
                    ForgeQueryDeclarationBridgeRoutingSupportStatus::Admitted
                } else {
                    row.status()
                }
            })
            .unwrap_or(ForgeQueryDeclarationBridgeRoutingSupportStatus::Unsupported);
        forge_query_checked_declaration_bridge_routing_on_handle(
            self.handle_identity_digest(),
            self.operating_context_identity_digest(),
            support_status,
            subject,
        )
    }

    pub fn bridge_continuation_support<I: ForgeQueryDeclarationInput<D>>(
        &self,
    ) -> ForgeQueryDeclarationBridgeRoutingSupportReport<D, I> {
        derive_bridge_routing_support_report::<D, C, I>(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>
{
    pub fn route_bridge_continuation_from_progressed<I>(
        &self,
        progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Result<
        ForgeQueryDeclarationBridgeRouting<D, I>,
        ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsBridgeContinuation<D>,
    {
        self.route_bridge_continuation_from_progressed_with_intent(progressed, None)
    }

    pub fn route_bridge_continuation_from_progressed_with_intent<I>(
        &self,
        progressed: crate::application::ForgeQueryAdmittedDeclarationProgression<D, I>,
        intent: impl Into<Option<ForgeQueryDeclarationRouteIntent>>,
    ) -> Result<
        ForgeQueryDeclarationBridgeRouting<D, I>,
        ForgeQueryDeclarationBridgeRoutingTerminalError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsBridgeContinuation<D>,
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
        match self.route_bridge_continuation_checked(
            ForgeQueryDeclarationBridgeRoutingInput::envelope_checked(envelope_checked),
        ) {
            ForgeQueryDeclarationBridgeRoutingChecked::Routed(routing) => Ok(routing),
            ForgeQueryDeclarationBridgeRoutingChecked::Deferred(routing) => Err(
                ForgeQueryDeclarationBridgeRoutingTerminalError::Deferred(routing),
            ),
            ForgeQueryDeclarationBridgeRoutingChecked::Denied(routing) => Err(
                ForgeQueryDeclarationBridgeRoutingTerminalError::Denied(routing),
            ),
            ForgeQueryDeclarationBridgeRoutingChecked::Failed(routing) => Err(
                ForgeQueryDeclarationBridgeRoutingTerminalError::Failed(routing),
            ),
        }
    }

    pub fn declare_review_progress_describe_plan_receipt_envelope_and_route_bridge_continuation<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryDeclarationBridgeRouting<D, I>,
        crate::application::ForgeQueryDeclarationEntryBridgeRoutingError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryDeclarationSupportsBridgeContinuation<D>,
    {
        let progressed = self.declare_review_and_progress(input).map_err(|error| {
            crate::application::ForgeQueryDeclarationEntryBridgeRoutingError::Entry(
                crate::application::ForgeQueryDeclarationEntryEnvelopeError::Entry(
                    crate::application::ForgeQueryDeclarationEntryReceiptError::Entry(error),
                ),
            )
        })?;
        self.route_bridge_continuation_from_progressed(progressed)
            .map_err(crate::application::ForgeQueryDeclarationEntryBridgeRoutingError::Routing)
    }
}
