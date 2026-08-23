#[derive(Clone, Copy)]
pub(crate) struct UiLocalInputRecipientBindingContext<'world> {
    pub(super) host_session: u64,
    pub(super) application_generation: worth_ui_host_contract::UiHostApplicationGeneration,
    pub(super) active_generation:
        &'world crate::runtime::WorthUiActiveApplicationGenerationIdentity,
    pub(super) mounted: &'world crate::mounting::WorthUiMountedSessionState,
}

#[derive(Clone, Copy)]
pub(super) struct UiLocalInputRecipientAffinityLease {
    binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
}

impl<'world> UiLocalInputRecipientBindingContext<'world> {
    pub(crate) const fn new(
        host_session: u64,
        application_generation: worth_ui_host_contract::UiHostApplicationGeneration,
        active_generation: &'world crate::runtime::WorthUiActiveApplicationGenerationIdentity,
        mounted: &'world crate::mounting::WorthUiMountedSessionState,
    ) -> Self {
        Self {
            host_session,
            application_generation,
            active_generation,
            mounted,
        }
    }
}

impl UiLocalInputRecipientAffinityLease {
    pub(super) const fn new(
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> Self {
        Self { binding }
    }

    pub(super) const fn binding(
        self,
    ) -> worth_ui_host_contract::UiHostInputRecipientBindingReceipt {
        self.binding
    }

    pub(super) fn admits_report(
        self,
        report: &worth_ui_host_contract::UiHostObservationReport,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> bool {
        report.input_affinity().is_some_and(|affinity| {
            affinity.binding() == self.binding && affinity.presentation() == presentation
        })
    }

    pub(super) fn reported_text_profile_mismatch(
        self,
        report: &worth_ui_host_contract::UiHostObservationReport,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Option<(
        worth_ui_host_contract::UiTextProfileGeneration,
        Option<worth_ui_host_contract::UiTextProfileGeneration>,
    )> {
        let affinity = report.input_affinity()?;
        if affinity.presentation() != presentation {
            return None;
        }
        let reported = affinity.binding();
        let expected_profile = self.binding.text_profile()?;
        if !same_recipient_except_text_profile(self.binding, reported)
            || reported.text_profile() == Some(expected_profile)
        {
            return None;
        }
        Some((expected_profile, reported.text_profile()))
    }
}

fn same_recipient_except_text_profile(
    expected: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    reported: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
) -> bool {
    expected.host_session() == reported.host_session()
        && expected.application_generation() == reported.application_generation()
        && expected.recipient_generation() == reported.recipient_generation()
        && expected.family() == reported.family()
        && expected.draft_session() == reported.draft_session()
        && expected.surface() == reported.surface()
        && expected.binding() == reported.binding()
        && expected.mounted_instance() == reported.mounted_instance()
        && expected.node_receipt() == reported.node_receipt()
}

pub(super) fn host_binding_receipt(
    context: UiLocalInputRecipientBindingContext<'_>,
    recipient_generation: worth_ui_host_contract::UiHostInputRecipientGeneration,
    family: super::UiLocalInputRecipientFamily,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    session: Option<super::UiDraftSessionIdentity>,
) -> Result<
    worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    super::UiLocalInputRecipientBindingStopReason,
> {
    let text_profile = if family == super::UiLocalInputRecipientFamily::Draft {
        Some(
            context
                .mounted
                .input_text_profile(target)
                .ok_or(super::UiLocalInputRecipientBindingStopReason::MissingTextProfile)?,
        )
    } else {
        None
    };
    Ok(
        worth_ui_host_contract::UiHostInputRecipientBindingReceipt::new(
            worth_ui_host_contract::UiHostInputRecipientBindingInput {
                host_session: context.host_session,
                application_generation: context.application_generation,
                recipient_generation,
                family,
                draft_session: session.map(|identity| {
                    worth_ui_host_contract::UiHostInputDraftSessionIdentity::new(
                        identity.diagnostic_value(),
                    )
                    .expect("runtime draft identities are nonzero")
                }),
                surface: target.surface(),
                binding: target.binding(),
                mounted_instance: target.mounted_instance(),
                node_receipt: target.node_receipt(),
                text_profile,
            },
        ),
    )
}
