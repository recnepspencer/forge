use super::{UiNativeTextValue, UiProjectionAvailability, UiProjectionFactReceipt};

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionFactReceipt {
    core: UiProjectionFactReceipt,
    availability: UiProjectionAvailability<UiNativeTextValue>,
}

impl UiScalarProjectionFactReceipt {
    pub fn core(&self) -> &UiProjectionFactReceipt {
        &self.core
    }

    pub fn availability(&self) -> &UiProjectionAvailability<UiNativeTextValue> {
        &self.availability
    }
}
