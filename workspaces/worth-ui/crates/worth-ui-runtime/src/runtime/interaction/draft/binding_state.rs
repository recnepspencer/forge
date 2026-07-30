use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;

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
            sessions: Default::default(),
            active: None,
            counters: Default::default(),
        }
    }

    pub(crate) fn bind(
        &mut self,
        activation: UiActivateInteraction,
        generation: &WorthUiPreparedApplicationGenerationIdentity,
        contract: UiLocalInputRecipientContract,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> Result<UiLocalInputRecipientAdmission, UiLocalInputRecipientBindingStop> {
        let target = activation.target();
        if let Err(denial) =
            crate::runtime::interaction::targeting::require_current_target(mounted, target)
        {
            return Err(UiLocalInputRecipientBindingStop::new(
                activation,
                UiLocalInputRecipientBindingStopReason::TargetNoLongerCurrent(denial),
            ));
        }
        let (next_active, binding) = match self.prepare_binding(target, generation, contract) {
            Ok(prepared) => prepared,
            Err(reason) => return Err(UiLocalInputRecipientBindingStop::new(activation, reason)),
        };
        let displaced = self.suspend_active(super::UiLocalInputStopReason::RecipientReplaced);
        self.active = Some(next_active);
        self.counters.recipients_bound = next(self.counters.recipients_bound);
        Ok(UiLocalInputRecipientAdmission::new(
            activation, binding, displaced,
        ))
    }

    fn prepare_binding(
        &mut self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        generation: &WorthUiPreparedApplicationGenerationIdentity,
        contract: UiLocalInputRecipientContract,
    ) -> Result<
        (UiActiveLocalRecipient, UiLocalInputRecipientBindingReceipt),
        UiLocalInputRecipientBindingStopReason,
    > {
        Ok(match contract.kind() {
            UiLocalInputRecipientContractKind::Activation => {
                let active = UiActiveLocalRecipient::Activation(UiRecipientContext {
                    target,
                    generation: generation.clone(),
                });
                let receipt =
                    binding_receipt(UiLocalInputRecipientFamily::Activation, target, None, false);
                (active, receipt)
            }
            UiLocalInputRecipientContractKind::Submit => {
                let active = UiActiveLocalRecipient::Submit(UiRecipientContext {
                    target,
                    generation: generation.clone(),
                });
                let receipt =
                    binding_receipt(UiLocalInputRecipientFamily::Submit, target, None, false);
                (active, receipt)
            }
            UiLocalInputRecipientContractKind::Draft { field, budget } => {
                let (session, resumed) = match self.find_session(target, generation, field) {
                    Some(session) => (session, true),
                    None => {
                        let session = self.create_session(target, generation, field, budget)?;
                        (session, false)
                    }
                };
                let receipt = binding_receipt(
                    UiLocalInputRecipientFamily::Draft,
                    target,
                    Some(session),
                    resumed,
                );
                (UiActiveLocalRecipient::Draft(session), receipt)
            }
        })
    }

    pub(super) fn create_session(
        &mut self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        generation: &WorthUiPreparedApplicationGenerationIdentity,
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
        self.counters.sessions_started = next(self.counters.sessions_started);
        Ok(identity)
    }

    fn find_session(
        &self,
        target: crate::runtime::interaction::UiPresentedInteractionTargetView,
        generation: &WorthUiPreparedApplicationGenerationIdentity,
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
}

fn binding_receipt(
    family: UiLocalInputRecipientFamily,
    target: crate::runtime::interaction::UiPresentedInteractionTargetView,
    session: Option<UiDraftSessionIdentity>,
    resumed: bool,
) -> UiLocalInputRecipientBindingReceipt {
    UiLocalInputRecipientBindingReceipt::new(session, family, target, resumed)
}
