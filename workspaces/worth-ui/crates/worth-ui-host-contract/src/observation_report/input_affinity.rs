#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiHostApplicationGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiHostInputRecipientGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiHostInputDraftSessionIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiHostInputRecipientFamily {
    Activation,
    Draft,
    Submit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostInputRecipientBindingInput {
    pub host_session: u64,
    pub application_generation: UiHostApplicationGeneration,
    pub recipient_generation: UiHostInputRecipientGeneration,
    pub family: UiHostInputRecipientFamily,
    pub draft_session: Option<UiHostInputDraftSessionIdentity>,
    pub surface: crate::UiSemanticSurfaceIdentity,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub mounted_instance: crate::UiMountedInstanceIdentity,
    pub node_receipt: crate::UiMountedNodeReceiptIdentity,
    pub text_profile: Option<crate::UiTextProfileGeneration>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostInputRecipientBindingReceipt(UiHostInputRecipientBindingInput);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostInputRecipientAffinityReceipt {
    binding: UiHostInputRecipientBindingReceipt,
    presentation: super::UiHostObservationPresentationBasis,
}

macro_rules! nonzero_identity {
    ($name:ident) => {
        impl $name {
            pub const fn new(value: u64) -> Option<Self> {
                if value == 0 {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

nonzero_identity!(UiHostApplicationGeneration);
nonzero_identity!(UiHostInputRecipientGeneration);
nonzero_identity!(UiHostInputDraftSessionIdentity);

impl UiHostInputRecipientBindingReceipt {
    pub const fn new(input: UiHostInputRecipientBindingInput) -> Self {
        Self(input)
    }

    pub const fn host_session(self) -> u64 {
        self.0.host_session
    }

    pub const fn application_generation(self) -> UiHostApplicationGeneration {
        self.0.application_generation
    }

    pub const fn recipient_generation(self) -> UiHostInputRecipientGeneration {
        self.0.recipient_generation
    }

    pub const fn family(self) -> UiHostInputRecipientFamily {
        self.0.family
    }

    pub const fn draft_session(self) -> Option<UiHostInputDraftSessionIdentity> {
        self.0.draft_session
    }

    pub const fn surface(self) -> crate::UiSemanticSurfaceIdentity {
        self.0.surface
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.0.binding
    }

    pub const fn mounted_instance(self) -> crate::UiMountedInstanceIdentity {
        self.0.mounted_instance
    }

    pub const fn node_receipt(self) -> crate::UiMountedNodeReceiptIdentity {
        self.0.node_receipt
    }

    pub const fn text_profile(self) -> Option<crate::UiTextProfileGeneration> {
        self.0.text_profile
    }
}

impl UiHostInputRecipientAffinityReceipt {
    pub const fn at_event_time(
        binding: UiHostInputRecipientBindingReceipt,
        presentation: super::UiHostObservationPresentationBasis,
    ) -> Self {
        Self {
            binding,
            presentation,
        }
    }

    pub const fn binding(self) -> UiHostInputRecipientBindingReceipt {
        self.binding
    }

    pub const fn presentation(self) -> super::UiHostObservationPresentationBasis {
        self.presentation
    }
}
