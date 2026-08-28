use core::num::NonZeroU64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceProducedFactReference {
    identity: NonZeroU64,
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::UiServiceProposalOccupancyScopeIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) struct UiServiceMountedWorkReference {
    identity: NonZeroU64,
    family: crate::capability::UiRuntimeServiceFamily,
    scope: super::super::UiServiceProposalOccupancyScopeIdentity,
}

macro_rules! reference_impl {
    ($name:ident) => {
        impl $name {
            #[cfg(test)]
            pub(in crate::runtime) fn recorded_fixture(
                identity: u64,
                family: crate::capability::UiRuntimeServiceFamily,
                scope: super::super::UiServiceProposalOccupancyScopeIdentity,
            ) -> Self {
                Self {
                    identity: NonZeroU64::new(identity)
                        .expect("recorded reference identity must be non-zero"),
                    family,
                    scope,
                }
            }

            pub(in crate::runtime) const fn diagnostic_value(self) -> u64 {
                self.identity.get()
            }

            pub(in crate::runtime) const fn family(
                self,
            ) -> crate::capability::UiRuntimeServiceFamily {
                self.family
            }

            pub(in crate::runtime) const fn scope(
                self,
            ) -> super::super::UiServiceProposalOccupancyScopeIdentity {
                self.scope
            }
        }
    };
}

reference_impl!(UiServiceProducedFactReference);
reference_impl!(UiServiceMountedWorkReference);

impl UiServiceProducedFactReference {
    pub(in crate::runtime) fn for_focus_proposal(
        proposal: super::UiServiceProposalIdentity,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            identity: NonZeroU64::new(proposal.diagnostic_value())
                .expect("proposal identity is derived from a non-zero request identity"),
            family: crate::capability::UiRuntimeServiceFamily::Focus,
            scope,
        }
    }

    pub(in crate::runtime) fn for_motion_proposal(
        proposal: super::UiServiceProposalIdentity,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            identity: NonZeroU64::new(proposal.diagnostic_value())
                .expect("proposal identity is derived from a non-zero request identity"),
            family: crate::capability::UiRuntimeServiceFamily::Motion,
            scope,
        }
    }

    pub(in crate::runtime) fn for_scroll_proposal(
        proposal: super::UiServiceProposalIdentity,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self::for_family(
            proposal,
            crate::capability::UiRuntimeServiceFamily::Scroll,
            scope,
        )
    }

    pub(in crate::runtime) fn for_selection_proposal(
        proposal: super::UiServiceProposalIdentity,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self::for_family(
            proposal,
            crate::capability::UiRuntimeServiceFamily::Selection,
            scope,
        )
    }

    fn for_family(
        proposal: super::UiServiceProposalIdentity,
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            identity: NonZeroU64::new(proposal.diagnostic_value())
                .expect("proposal identity is derived from a non-zero request identity"),
            family,
            scope,
        }
    }
}

impl UiServiceMountedWorkReference {
    pub(in crate::runtime) fn for_portal_proposal(
        proposal: super::UiServiceProposalIdentity,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    ) -> Self {
        Self {
            identity: NonZeroU64::new(proposal.diagnostic_value())
                .expect("proposal identity is derived from a non-zero request identity"),
            family: crate::capability::UiRuntimeServiceFamily::Portal,
            scope,
        }
    }
}
