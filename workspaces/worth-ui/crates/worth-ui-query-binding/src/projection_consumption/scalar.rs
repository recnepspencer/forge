use super::{UiNativeTextValue, UiProjectionAvailability, UiProjectionFactReceipt};

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub struct UiScalarProjectionFactReceipt {
    core: UiProjectionFactReceipt,
    availability: UiProjectionAvailability<UiNativeTextValue>,
}

impl UiScalarProjectionFactReceipt {
    pub(crate) fn admitted(
        core: UiProjectionFactReceipt,
        availability: UiProjectionAvailability<UiNativeTextValue>,
    ) -> Self {
        Self { core, availability }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiProjectionFactReceipt,
        UiProjectionAvailability<UiNativeTextValue>,
    ) {
        (self.core, self.availability)
    }

    pub fn core(&self) -> &UiProjectionFactReceipt {
        &self.core
    }

    pub fn availability(&self) -> &UiProjectionAvailability<UiNativeTextValue> {
        &self.availability
    }

    pub fn into_observation(self) -> crate::UiScalarProjectionObservation {
        crate::UiScalarProjectionObservation::query_issued(self)
    }
}
