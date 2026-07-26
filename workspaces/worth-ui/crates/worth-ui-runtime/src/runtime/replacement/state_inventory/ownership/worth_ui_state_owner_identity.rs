use crate::runtime::WorthUiStateOwnershipClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStateOwnerIdentity {
    ownership_class: WorthUiStateOwnershipClass,
    identity_basis: String,
}

impl WorthUiStateOwnerIdentity {
    pub(crate) fn platform_state_family(identity_basis: impl Into<String>) -> Self {
        Self::new(WorthUiStateOwnershipClass::PlatformShell, identity_basis)
    }

    pub(crate) fn node_identity(identity_basis: impl Into<String>) -> Self {
        Self::new(WorthUiStateOwnershipClass::NodeIdentity, identity_basis)
    }

    pub(crate) fn shell_local_interaction(identity_basis: impl Into<String>) -> Self {
        Self::new(
            WorthUiStateOwnershipClass::ShellLocalInteraction,
            identity_basis,
        )
    }

    fn new(ownership_class: WorthUiStateOwnershipClass, identity_basis: impl Into<String>) -> Self {
        Self {
            ownership_class,
            identity_basis: identity_basis.into(),
        }
    }
}
