use crate::WorthUiQueryViewIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingRegistrationDenialKind {
    ForeignInstalledDomain,
    DuplicateViewIdentity,
    DuplicateProjectionIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingRegistrationDenial {
    pub(super) kind: WorthUiQueryBindingRegistrationDenialKind,
    pub(super) identity: WorthUiQueryViewIdentity,
}

impl WorthUiQueryBindingRegistrationDenial {
    pub fn kind(&self) -> WorthUiQueryBindingRegistrationDenialKind {
        self.kind
    }

    pub fn identity(&self) -> &WorthUiQueryViewIdentity {
        &self.identity
    }
}
