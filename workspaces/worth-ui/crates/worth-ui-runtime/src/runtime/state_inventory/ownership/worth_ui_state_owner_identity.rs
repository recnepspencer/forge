use crate::runtime::WorthUiStateOwnershipClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateOwnerIdentity {
    ownership_class: WorthUiStateOwnershipClass,
    identity_basis: String,
}

impl WorthUiStateOwnerIdentity {
    pub fn platform_shell() -> Self {
        Self::new(
            WorthUiStateOwnershipClass::PlatformShell,
            "worth-ui.platform",
        )
    }

    pub(crate) fn platform_state_family(identity_basis: impl Into<String>) -> Self {
        Self::new(WorthUiStateOwnershipClass::PlatformShell, identity_basis)
    }

    pub fn node_identity(identity_basis: impl Into<String>) -> Self {
        Self::new(WorthUiStateOwnershipClass::NodeIdentity, identity_basis)
    }

    pub fn shell_local_interaction(identity_basis: impl Into<String>) -> Self {
        Self::new(
            WorthUiStateOwnershipClass::ShellLocalInteraction,
            identity_basis,
        )
    }

    pub fn custom_hook(identity_basis: impl Into<String>) -> Self {
        Self::new(WorthUiStateOwnershipClass::CustomHook, identity_basis)
    }

    pub fn domain_truth(identity_basis: impl Into<String>) -> Self {
        Self::new(WorthUiStateOwnershipClass::DomainTruth, identity_basis)
    }

    pub fn ownership_class(&self) -> WorthUiStateOwnershipClass {
        self.ownership_class
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn has_explicit_identity_basis(&self) -> bool {
        !self.identity_basis.trim().is_empty()
    }

    fn new(ownership_class: WorthUiStateOwnershipClass, identity_basis: impl Into<String>) -> Self {
        Self {
            ownership_class,
            identity_basis: identity_basis.into(),
        }
    }
}
