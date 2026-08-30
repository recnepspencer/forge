/// Capability-driven installation slot for one concrete runtime-service owner.
/// The slot carries no substitute state when the family is unsupported.
pub(crate) struct UiRuntimeServiceInstallation<Owner> {
    owner: Option<Owner>,
}

impl<Owner> UiRuntimeServiceInstallation<Owner> {
    pub(crate) fn from_optional(owner: Option<Owner>) -> Self {
        Self { owner }
    }

    pub(crate) const fn is_installed(&self) -> bool {
        self.owner.is_some()
    }

    pub(crate) const fn as_ref(&self) -> Option<&Owner> {
        self.owner.as_ref()
    }

    pub(crate) fn as_mut(&mut self) -> Option<&mut Owner> {
        self.owner.as_mut()
    }

    pub(crate) fn take(&mut self) -> Option<Owner> {
        self.owner.take()
    }
}
