#[must_use]
#[derive(Debug)]
pub(in crate::runtime) struct UiServiceProposalOccupancyLease {
    pub(super) key: UiServiceProposalOccupancyKey,
    pub(super) proposal: super::super::UiServiceProposalIdentity,
    pub(super) slot_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(in crate::runtime) enum UiServiceProposalOccupancyScopeIdentity {
    MountedOwner(worth_ui_host_contract::UiMountedInstanceIdentity),
    #[cfg(any(test, feature = "certification-support"))]
    Test(core::num::NonZeroU64),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiServiceProposalOccupancyKey {
    pub(super) application: crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity,
    pub(super) semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(super) family: crate::capability::UiRuntimeServiceFamily,
    pub(super) scope: UiServiceProposalOccupancyScopeIdentity,
}

impl UiServiceProposalOccupancyScopeIdentity {
    pub(in crate::runtime) const fn for_mounted_owner(
        value: worth_ui_host_contract::UiMountedInstanceIdentity,
    ) -> Self {
        Self::MountedOwner(value)
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(in crate::runtime) fn for_test(value: u64) -> Self {
        Self::Test(
            core::num::NonZeroU64::new(value).expect("test occupancy scope must be non-zero"),
        )
    }
}

impl UiServiceProposalOccupancyLease {
    #[cfg(test)]
    pub(in crate::runtime) const fn proposal(&self) -> super::super::UiServiceProposalIdentity {
        self.proposal
    }

    #[cfg(test)]
    pub(in crate::runtime) fn application(
        &self,
    ) -> &crate::runtime::intent::WorthUiActiveApplicationGenerationIdentity {
        &self.key.application
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn semantic_surface(
        &self,
    ) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.key.semantic_surface
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn family(&self) -> crate::capability::UiRuntimeServiceFamily {
        self.key.family
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn scope(&self) -> UiServiceProposalOccupancyScopeIdentity {
        self.key.scope
    }

    #[cfg(test)]
    pub(in crate::runtime) const fn slot_generation(&self) -> u64 {
        self.slot_generation
    }
}
