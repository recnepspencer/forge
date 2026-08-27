pub(crate) struct UiPreparedIntentExecution {
    inner: Box<dyn UiPreparedIntentExecutionBinding>,
}

pub(super) trait UiPreparedIntentExecutionBinding: Send {
    fn retained_payload_count(&self) -> usize;
    fn destination(&self) -> crate::capability::UiIntentExecutionDestination;
    fn provider_version(&self) -> super::UiIntentProviderVersion;
    fn start(
        self: Box<Self>,
        context: super::UiManagedIntentExecutionStartContext,
    ) -> super::UiManagedIntentExecutionStart;
}

pub(super) struct UiTypedPreparedIntentExecution<I, Provider>
where
    I: crate::capability::UiIntent,
    Provider: super::UiIntentExecutionProvider<I>,
{
    payload: I::Payload,
    provider: std::sync::Arc<Provider>,
}

pub(super) struct UiTypedPreparedTransition<I: crate::capability::UiIntent> {
    payload: I::Payload,
    destination: crate::capability::UiIntentTransitionDestination,
}

pub(super) struct UiTypedPreparedUnsupportedCommand<I: crate::capability::UiIntent> {
    payload: I::Payload,
}

pub(super) struct UiTypedPreparedRuntimeService<I: crate::capability::UiIntent> {
    payload: I::Payload,
    destination: crate::capability::UiIntentRuntimeServiceDestination,
}

impl UiPreparedIntentExecution {
    pub(super) fn application<I, Provider>(
        payload: I::Payload,
        provider: std::sync::Arc<Provider>,
    ) -> Self
    where
        I: crate::capability::UiIntent,
        Provider: super::UiIntentExecutionProvider<I>,
    {
        Self {
            inner: Box::new(UiTypedPreparedIntentExecution::<I, Provider> { payload, provider }),
        }
    }

    pub(super) fn transition<I: crate::capability::UiIntent>(
        payload: I::Payload,
        destination: crate::capability::UiIntentTransitionDestination,
    ) -> Self
    where
        I::ProductOutcome: crate::capability::UiIntentTransitionOutcome,
    {
        Self {
            inner: Box::new(UiTypedPreparedTransition::<I> {
                payload,
                destination,
            }),
        }
    }

    pub(super) fn unsupported_command<I: crate::capability::UiIntent>(payload: I::Payload) -> Self {
        Self {
            inner: Box::new(UiTypedPreparedUnsupportedCommand::<I> { payload }),
        }
    }

    pub(super) fn runtime_service<I: crate::capability::UiIntent>(
        payload: I::Payload,
        destination: crate::capability::UiIntentRuntimeServiceDestination,
    ) -> Self {
        Self {
            inner: Box::new(UiTypedPreparedRuntimeService::<I> {
                payload,
                destination,
            }),
        }
    }

    pub(crate) fn retained_payload_count(&self) -> usize {
        self.inner.retained_payload_count()
    }

    pub(crate) fn reservation_basis(
        &self,
        intent: crate::capability::UiIntentId,
        retained_payload_bytes: usize,
    ) -> super::super::UiIntentExecutionReservationBasis {
        super::super::UiIntentExecutionReservationBasis::new(
            intent,
            self.inner.destination(),
            self.inner.provider_version(),
            retained_payload_bytes,
        )
    }

    pub(crate) fn start(
        self,
        context: super::UiManagedIntentExecutionStartContext,
    ) -> super::UiManagedIntentExecutionStart {
        self.inner.start(context)
    }
}

impl<I, Provider> UiPreparedIntentExecutionBinding for UiTypedPreparedIntentExecution<I, Provider>
where
    I: crate::capability::UiIntent,
    Provider: super::UiIntentExecutionProvider<I>,
{
    fn retained_payload_count(&self) -> usize {
        let _ = (&self.payload, &self.provider);
        1
    }

    fn destination(&self) -> crate::capability::UiIntentExecutionDestination {
        crate::capability::UiIntentExecutionDestination::ApplicationEffect
    }

    fn provider_version(&self) -> super::UiIntentProviderVersion {
        Provider::VERSION
    }

    fn start(
        self: Box<Self>,
        context: super::UiManagedIntentExecutionStartContext,
    ) -> super::UiManagedIntentExecutionStart {
        let Self { payload, provider } = *self;
        let request = super::UiIntentExecutionRequest::new(
            context.attempt(),
            context.idempotency(),
            payload,
            context.deadline(),
        );
        match provider.begin(request) {
            super::UiIntentProviderStart::Started(attempt) => {
                super::UiManagedIntentExecutionStart::Running(Box::new(
                    super::managed::UiTypedManagedIntentExecution::<I, Provider>::new(
                        attempt, provider,
                    ),
                ))
            }
            super::UiIntentProviderStart::RejectedBeforeEffect(detail) => {
                super::UiManagedIntentExecutionStart::Settled(
                    super::UiManagedIntentSettlement::RejectedBeforeEffect(detail),
                )
            }
        }
    }
}

impl<I> UiPreparedIntentExecutionBinding for UiTypedPreparedTransition<I>
where
    I: crate::capability::UiIntent,
    I::ProductOutcome: crate::capability::UiIntentTransitionOutcome,
{
    fn retained_payload_count(&self) -> usize {
        let _ = (&self.payload, self.destination);
        1
    }

    fn destination(&self) -> crate::capability::UiIntentExecutionDestination {
        crate::capability::UiIntentExecutionDestination::UiTransition(self.destination)
    }

    fn provider_version(&self) -> super::UiIntentProviderVersion {
        super::UiIntentProviderVersion::stable(1)
    }

    fn start(
        self: Box<Self>,
        _context: super::UiManagedIntentExecutionStartContext,
    ) -> super::UiManagedIntentExecutionStart {
        let Self {
            payload: _,
            destination,
        } = *self;
        let outcome = <I::ProductOutcome as crate::capability::UiIntentTransitionOutcome>::from_completed_transition(destination);
        super::UiManagedIntentExecutionStart::Settled(super::UiManagedIntentSettlement::Completed(
            super::managed::outcome_material(outcome),
        ))
    }
}

impl<I: crate::capability::UiIntent> UiPreparedIntentExecutionBinding
    for UiTypedPreparedUnsupportedCommand<I>
{
    fn retained_payload_count(&self) -> usize {
        let _ = &self.payload;
        1
    }

    fn destination(&self) -> crate::capability::UiIntentExecutionDestination {
        crate::capability::UiIntentExecutionDestination::RuntimeService(
            crate::capability::UiIntentRuntimeServiceDestination::InvokeCommand,
        )
    }

    fn provider_version(&self) -> super::UiIntentProviderVersion {
        super::UiIntentProviderVersion::stable(1)
    }

    fn start(
        self: Box<Self>,
        _context: super::UiManagedIntentExecutionStartContext,
    ) -> super::UiManagedIntentExecutionStart {
        let _ = *self;
        super::UiManagedIntentExecutionStart::Settled(
            super::UiManagedIntentSettlement::RejectedBeforeEffect(
                super::UiIntentProviderStop::stable("worth_ui.command_routing.unsupported"),
            ),
        )
    }
}

impl<I: crate::capability::UiIntent> UiPreparedIntentExecutionBinding
    for UiTypedPreparedRuntimeService<I>
{
    fn retained_payload_count(&self) -> usize {
        let _ = (&self.payload, self.destination);
        1
    }

    fn destination(&self) -> crate::capability::UiIntentExecutionDestination {
        crate::capability::UiIntentExecutionDestination::RuntimeService(self.destination)
    }

    fn provider_version(&self) -> super::UiIntentProviderVersion {
        super::UiIntentProviderVersion::stable(1)
    }

    fn start(
        self: Box<Self>,
        _context: super::UiManagedIntentExecutionStartContext,
    ) -> super::UiManagedIntentExecutionStart {
        let Self {
            payload: _,
            destination,
        } = *self;
        super::UiManagedIntentExecutionStart::Settled(super::UiManagedIntentSettlement::Completed(
            super::managed::runtime_service_material::<I>(destination),
        ))
    }
}
