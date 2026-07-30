#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryViewRegistrationError {
    Binding(worth_ui_query_binding::WorthUiQueryBindingRegistrationDenial),
    InvalidIdentity(crate::capability::CapabilityIdError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionRegistrationError(
    pub(super) worth_ui_query_binding::WorthUiQueryBindingRegistrationDenial,
);

impl WorthUiProjectionRegistrationError {
    pub fn denial(&self) -> &worth_ui_query_binding::WorthUiQueryBindingRegistrationDenial {
        &self.0
    }
}
