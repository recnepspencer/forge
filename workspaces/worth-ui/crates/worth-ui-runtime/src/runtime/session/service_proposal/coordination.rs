/// Cross-family requirement emitted by Focus and consumed by Scroll through
/// proposal compilation. Focus names the semantic target; Scroll owns how the
/// nearest lawful reveal is computed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiFocusRevealRequirement {
    target: worth_ui_host_contract::UiMountedInstanceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiSelectionInvocationCause {
    DeclaredPointerActivation,
    DeclaredKeyboardActivation,
    DeclaredIntentActivation,
}

/// Declared action that may accompany focus only inside one compiled proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiDeclaredFocusSelectionAction {
    mounted_target: worth_ui_host_contract::UiMountedInstanceIdentity,
    owner: crate::runtime::selection::UiSelectionOwnerIdentity,
    incarnation: crate::runtime::selection::UiSelectionOwnerIncarnation,
    request: crate::runtime::selection::UiSelectionRequest,
    cause: UiSelectionInvocationCause,
}

impl UiFocusRevealRequirement {
    pub(in crate::runtime) const fn new(
        target: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> Self {
        Self { target }
    }

    pub(in crate::runtime) const fn target(
        self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.target
    }

    #[cfg(test)]
    pub(in crate::runtime) fn recorded_fixture() -> Self {
        Self::new(
            worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound()
                .expect("recorded mounted identity"),
        )
    }
}

impl UiDeclaredFocusSelectionAction {
    pub(in crate::runtime) const fn new(
        mounted_target: worth_ui_host_contract::UiMountedInstanceIdentity,
        owner: crate::runtime::selection::UiSelectionOwnerIdentity,
        incarnation: crate::runtime::selection::UiSelectionOwnerIncarnation,
        request: crate::runtime::selection::UiSelectionRequest,
        cause: UiSelectionInvocationCause,
    ) -> Self {
        Self {
            mounted_target,
            owner,
            incarnation,
            request,
            cause,
        }
    }

    pub(in crate::runtime) const fn mounted_target(
        self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.mounted_target
    }

    pub(in crate::runtime) const fn owner(
        self,
    ) -> crate::runtime::selection::UiSelectionOwnerIdentity {
        self.owner
    }

    pub(in crate::runtime) const fn incarnation(
        self,
    ) -> crate::runtime::selection::UiSelectionOwnerIncarnation {
        self.incarnation
    }

    pub(in crate::runtime) const fn request(self) -> crate::runtime::selection::UiSelectionRequest {
        self.request
    }

    pub(in crate::runtime) const fn cause(self) -> UiSelectionInvocationCause {
        self.cause
    }

    #[cfg(test)]
    pub(in crate::runtime) fn recorded_fixture(
        mounted_target: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> Self {
        Self::new(
            mounted_target,
            crate::runtime::selection::UiSelectionOwnerIdentity::new(
                worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound()
                    .expect("recorded semantic surface"),
                crate::graph::UiGraphNodeIdentity::new(1),
                crate::runtime::UiApplicationItemKeyFamily::new(
                    core::num::NonZeroU64::new(1).expect("recorded item-key family"),
                ),
            ),
            crate::runtime::selection::UiSelectionOwnerIncarnation::new(1)
                .expect("recorded selection incarnation"),
            crate::runtime::selection::UiSelectionRequest::Clear,
            UiSelectionInvocationCause::DeclaredKeyboardActivation,
        )
    }
}
