use crate::runtime::WorthUiActiveApplicationGenerationIdentity;

use super::model::{
    next, UiActiveLocalRecipient, UiDraftRuntimeState, UiDraftSession, UiRecipientContext,
};
use super::{
    UiDraftFieldIdentity, UiDraftSessionIdentity, UiLocalInputRecipientAdmission,
    UiLocalInputRecipientBindingReceipt, UiLocalInputRecipientBindingStop,
    UiLocalInputRecipientBindingStopReason, UiLocalInputRecipientContract,
    UiLocalInputRecipientContractKind, UiLocalInputRecipientFamily, UI_DRAFT_SESSION_LIMIT,
};
use crate::runtime::interaction::UiActivateInteraction;

impl UiDraftRuntimeState {
    pub(crate) fn new() -> Self {
        Self {
            next_identity: Some(1),
            next_recipient_generation: Some(1),
            sessions: Default::default(),
            active: None,
            active_affinity: None,
            counters: Default::default(),
        }
    }

    pub(crate) fn bind<Install>(
        &mut self,
        activation: UiActivateInteraction,
        context: super::recipient_affinity::UiLocalInputRecipientBindingContext<'_>,
        contract: UiLocalInputRecipientContract,
        install: Install,
    ) -> Result<UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop>
    where
        Install: FnOnce(worth_ui_host_contract::UiHostInputRecipientBindingReceipt) -> bool,
    {
        let target = activation.target();
        if let Err(denial) =
            crate::runtime::interaction::targeting::require_current_target(context.mounted, target)
        {
            return Err(UiLocalInputRecipientBindingStop::new(
                activation,
                UiLocalInputRecipientBindingStopReason::TargetNoLongerCurrent(denial),
            ));
        }
        let prepared = match self.prepare_binding(target, context, contract) {
            Ok(prepared) => prepared,
            Err(reason) => return Err(UiLocalInputRecipientBindingStop::new(activation, reason)),
        };
        let (next_active, binding, host_binding, started_session) = prepared;
        if !install(host_binding) {
            if let Some(session) = started_session {
                self.sessions.remove(&session);
            }
            return Err(UiLocalInputRecipientBindingStop::new(
                activation,
                UiLocalInputRecipientBindingStopReason::HostAffinityInstallationDenied,
            ));
        }
        let displaced = self.suspend_active(super::UiLocalInputStopReason::RecipientReplaced);
        self.active = Some(next_active);
        self.active_affinity =
            Some(super::recipient_affinity::UiLocalInputRecipientAffinityLease::new(host_binding));
        if started_session.is_some() {
            self.counters.sessions_started = next(self.counters.sessions_started);
        }
        self.counters.recipients_bound = next(self.counters.recipients_bound);
        Ok(UiLocalInputRecipientAdmission::new(
            activation, binding, displaced,
        ))
    }

    fn prepare_binding(
        &mut self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        context: super::recipient_affinity::UiLocalInputRecipientBindingContext<'_>,
        contract: UiLocalInputRecipientContract,
    ) -> Result<
        (
            UiActiveLocalRecipient,
            UiLocalInputRecipientBindingReceipt,
            worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
            Option<UiDraftSessionIdentity>,
        ),
        UiLocalInputRecipientBindingStopReason,
    > {
        let recipient_generation = self.take_recipient_generation()?;
        Ok(match contract.kind() {
            UiLocalInputRecipientContractKind::Activation => {
                let active = UiActiveLocalRecipient::Activation(UiRecipientContext {
                    target,
                    generation: context.active_generation.clone(),
                });
                let receipt =
                    binding_receipt(UiLocalInputRecipientFamily::Activation, target, None, false);
                let host = super::recipient_affinity::host_binding_receipt(
                    context,
                    recipient_generation,
                    UiLocalInputRecipientFamily::Activation,
                    target,
                    None,
                )?;
                (active, receipt, host, None)
            }
            UiLocalInputRecipientContractKind::Submit => {
                let active = UiActiveLocalRecipient::Submit(UiRecipientContext {
                    target,
                    generation: context.active_generation.clone(),
                });
                let receipt =
                    binding_receipt(UiLocalInputRecipientFamily::Submit, target, None, false);
                let host = super::recipient_affinity::host_binding_receipt(
                    context,
                    recipient_generation,
                    UiLocalInputRecipientFamily::Submit,
                    target,
                    None,
                )?;
                (active, receipt, host, None)
            }
            UiLocalInputRecipientContractKind::Draft { field, budget } => {
                let (session, resumed, started) =
                    match self.find_session(target, context.active_generation, field) {
                        Some(session) => (session, true, None),
                        None => {
                            let session = self.create_session(
                                target,
                                context.active_generation,
                                field,
                                budget,
                            )?;
                            (session, false, Some(session))
                        }
                    };
                let receipt = binding_receipt(
                    UiLocalInputRecipientFamily::Draft,
                    target,
                    Some(session),
                    resumed,
                );
                let host = match super::recipient_affinity::host_binding_receipt(
                    context,
                    recipient_generation,
                    UiLocalInputRecipientFamily::Draft,
                    target,
                    Some(session),
                ) {
                    Ok(host) => host,
                    Err(reason) => {
                        if started.is_some() {
                            self.sessions.remove(&session);
                        }
                        return Err(reason);
                    }
                };
                (
                    UiActiveLocalRecipient::Draft(session),
                    receipt,
                    host,
                    started,
                )
            }
        })
    }

    pub(super) fn create_session(
        &mut self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        generation: &WorthUiActiveApplicationGenerationIdentity,
        field: UiDraftFieldIdentity,
        budget: super::UiDraftByteBudget,
    ) -> Result<UiDraftSessionIdentity, UiLocalInputRecipientBindingStopReason> {
        if self.sessions.len() >= UI_DRAFT_SESSION_LIMIT {
            return Err(
                UiLocalInputRecipientBindingStopReason::DraftCapacityExceeded {
                    limit: UI_DRAFT_SESSION_LIMIT,
                },
            );
        }
        let identity = self
            .take_identity()
            .ok_or(UiLocalInputRecipientBindingStopReason::IdentityExhausted)?;
        self.sessions.insert(
            identity,
            UiDraftSession {
                target,
                generation: generation.clone(),
                field,
                budget,
                committed: String::new(),
                preedit: None,
                last_input_revision: None,
                draft_revision: 0,
            },
        );
        Ok(identity)
    }

    fn find_session(
        &self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        generation: &WorthUiActiveApplicationGenerationIdentity,
        field: UiDraftFieldIdentity,
    ) -> Option<UiDraftSessionIdentity> {
        self.sessions.iter().find_map(|(identity, session)| {
            (session.target.mounted_instance() == target.mounted_instance()
                && session.target.binding() == target.binding()
                && &session.generation == generation
                && session.field == field)
                .then_some(*identity)
        })
    }

    fn take_identity(&mut self) -> Option<UiDraftSessionIdentity> {
        let value = self.next_identity?;
        self.next_identity = value.checked_add(1);
        Some(UiDraftSessionIdentity::mint(value))
    }

    fn take_recipient_generation(
        &mut self,
    ) -> Result<
        worth_ui_host_contract::UiHostInputRecipientGeneration,
        UiLocalInputRecipientBindingStopReason,
    > {
        let value = self
            .next_recipient_generation
            .ok_or(UiLocalInputRecipientBindingStopReason::RecipientGenerationExhausted)?;
        self.next_recipient_generation = value.checked_add(1);
        worth_ui_host_contract::UiHostInputRecipientGeneration::new(value)
            .ok_or(UiLocalInputRecipientBindingStopReason::RecipientGenerationExhausted)
    }
}

fn binding_receipt(
    family: UiLocalInputRecipientFamily,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    session: Option<UiDraftSessionIdentity>,
    resumed: bool,
) -> UiLocalInputRecipientBindingReceipt {
    UiLocalInputRecipientBindingReceipt::new(session, family, target, resumed)
}
