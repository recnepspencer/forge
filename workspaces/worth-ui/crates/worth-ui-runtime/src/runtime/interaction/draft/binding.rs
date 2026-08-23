use super::UiLocalInputRecipientBindingReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiLocalInputRecipientBindingStopReason {
    TargetNoLongerCurrent(crate::runtime::interaction::UiInteractionTargetingDenial),
    DraftCapacityExceeded { limit: usize },
    IdentityExhausted,
    RecipientGenerationExhausted,
    MissingTextProfile,
    HostAffinityInstallationDenied,
}

#[derive(Debug)]
pub struct UiLocalInputRecipientAdmission {
    activation: crate::runtime::interaction::UiActivateInteraction,
    binding: UiLocalInputRecipientBindingReceipt,
    displaced: Option<super::UiLocalInputStop>,
}

#[derive(Debug)]
pub struct UiLocalInputRecipientBindingStop {
    activation: Box<crate::runtime::interaction::UiActivateInteraction>,
    reason: UiLocalInputRecipientBindingStopReason,
}

impl UiLocalInputRecipientAdmission {
    pub(super) const fn new(
        activation: crate::runtime::interaction::UiActivateInteraction,
        binding: UiLocalInputRecipientBindingReceipt,
        displaced: Option<super::UiLocalInputStop>,
    ) -> Self {
        Self {
            activation,
            binding,
            displaced,
        }
    }

    pub const fn activation(&self) -> &crate::runtime::interaction::UiActivateInteraction {
        &self.activation
    }

    pub const fn binding(&self) -> UiLocalInputRecipientBindingReceipt {
        self.binding
    }

    pub const fn displaced_recipient(&self) -> Option<&super::UiLocalInputStop> {
        self.displaced.as_ref()
    }
}

impl UiLocalInputRecipientBindingStop {
    pub(super) fn new(
        activation: crate::runtime::interaction::UiActivateInteraction,
        reason: UiLocalInputRecipientBindingStopReason,
    ) -> Self {
        Self {
            activation: Box::new(activation),
            reason,
        }
    }

    pub fn activation(&self) -> &crate::runtime::interaction::UiActivateInteraction {
        self.activation.as_ref()
    }

    pub const fn reason(&self) -> UiLocalInputRecipientBindingStopReason {
        self.reason
    }

    pub fn into_activation(self) -> crate::runtime::interaction::UiActivateInteraction {
        *self.activation
    }
}
