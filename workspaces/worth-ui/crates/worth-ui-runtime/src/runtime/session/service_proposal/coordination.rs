/// Cross-family requirement emitted by Focus and consumed by Scroll through
/// proposal compilation. Focus names the semantic target; Scroll owns how the
/// nearest lawful reveal is computed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiFocusRevealRequirement {
    target: worth_ui_host_contract::UiMountedInstanceIdentity,
    application_item_anchor: Option<crate::runtime::UiApplicationItemKey>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiSelectionInvocationCause {
    #[cfg(test)]
    Pointer,
    #[cfg(test)]
    Keyboard,
    Intent,
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
        Self {
            target,
            application_item_anchor: None,
        }
    }

    pub(in crate::runtime) const fn with_application_item_anchor(
        mut self,
        anchor: crate::runtime::UiApplicationItemKey,
    ) -> Self {
        self.application_item_anchor = Some(anchor);
        self
    }

    pub(in crate::runtime) const fn target(
        self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.target
    }

    pub(in crate::runtime) const fn application_item_anchor(
        self,
    ) -> Option<crate::runtime::UiApplicationItemKey> {
        self.application_item_anchor
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

    #[cfg(test)]
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
            crate::runtime::selection::UiSelectionRequest::SelectSingle(
                crate::runtime::selection::UiSelectionStableKey::new(
                    crate::runtime::UiApplicationItemKey::new(
                        crate::runtime::UiApplicationItemKeyFamily::new(
                            core::num::NonZeroU64::new(1).expect("recorded item-key family"),
                        ),
                        core::num::NonZeroU64::new(1).expect("recorded item-key value"),
                    ),
                ),
            ),
            UiSelectionInvocationCause::Keyboard,
        )
    }
}
