use crate::capability::{UiIntentExecutionDestination, UiIntentId};

use super::UiIntentProviderVersion;

pub const UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS: usize = 16;
pub const UI_INTENT_MAXIMUM_DESTINATION_ATTEMPTS: usize = 16;
pub const UI_INTENT_MAXIMUM_PROVIDER_ATTEMPTS: usize = 16;
pub const UI_INTENT_MAXIMUM_INTENT_ATTEMPTS: usize = 16;
pub const UI_INTENT_MAXIMUM_RETAINED_PAYLOAD_BYTES: usize = 4_194_304;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiIntentExecutionReservationDenial {
    ProviderCapacityExceeded {
        intent: UiIntentId,
        provider_version: UiIntentProviderVersion,
        maximum: usize,
    },
    IntentCapacityExceeded {
        intent: UiIntentId,
        maximum: usize,
    },
    DestinationCapacityExceeded {
        destination: UiIntentExecutionDestination,
        maximum: usize,
    },
    ApplicationCapacityExceeded {
        maximum: usize,
    },
    RetainedPayloadBytesExceeded {
        active: usize,
        requested: usize,
        maximum: usize,
    },
}

#[derive(Clone, Copy)]
pub(crate) struct UiIntentExecutionReservationBasis {
    intent: UiIntentId,
    destination: UiIntentExecutionDestination,
    provider_version: UiIntentProviderVersion,
    retained_payload_bytes: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct UiIntentExecutionCapacity {
    application_attempts: usize,
    destination_attempts: usize,
    provider_attempts: usize,
    intent_attempts: usize,
    retained_payload_bytes: usize,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct UiIntentExecutionReservationCounts {
    pub(crate) application_attempts: usize,
    pub(crate) destination_attempts: usize,
    pub(crate) provider_attempts: usize,
    pub(crate) intent_attempts: usize,
    pub(crate) retained_payload_bytes: usize,
}

impl UiIntentExecutionReservationBasis {
    pub(crate) const fn new(
        intent: UiIntentId,
        destination: UiIntentExecutionDestination,
        provider_version: UiIntentProviderVersion,
        retained_payload_bytes: usize,
    ) -> Self {
        Self {
            intent,
            destination,
            provider_version,
            retained_payload_bytes,
        }
    }

    pub(crate) const fn intent(self) -> UiIntentId {
        self.intent
    }

    pub(crate) const fn destination(self) -> UiIntentExecutionDestination {
        self.destination
    }

    pub(crate) const fn retained_payload_bytes(self) -> usize {
        self.retained_payload_bytes
    }

    pub(crate) fn same_provider(self, other: Self) -> bool {
        self.intent == other.intent && self.provider_version == other.provider_version
    }
}

impl UiIntentExecutionCapacity {
    pub(crate) const fn production() -> Self {
        Self {
            application_attempts: UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS,
            destination_attempts: UI_INTENT_MAXIMUM_DESTINATION_ATTEMPTS,
            provider_attempts: UI_INTENT_MAXIMUM_PROVIDER_ATTEMPTS,
            intent_attempts: UI_INTENT_MAXIMUM_INTENT_ATTEMPTS,
            retained_payload_bytes: UI_INTENT_MAXIMUM_RETAINED_PAYLOAD_BYTES,
        }
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) const fn bounded_for_certification(
        application_attempts: usize,
        destination_attempts: usize,
        provider_attempts: usize,
        intent_attempts: usize,
        retained_payload_bytes: usize,
    ) -> Option<Self> {
        if application_attempts == 0
            || application_attempts > UI_INTENT_MAXIMUM_APPLICATION_ATTEMPTS
            || destination_attempts == 0
            || destination_attempts > UI_INTENT_MAXIMUM_DESTINATION_ATTEMPTS
            || provider_attempts == 0
            || provider_attempts > UI_INTENT_MAXIMUM_PROVIDER_ATTEMPTS
            || intent_attempts == 0
            || intent_attempts > UI_INTENT_MAXIMUM_INTENT_ATTEMPTS
            || retained_payload_bytes == 0
            || retained_payload_bytes > UI_INTENT_MAXIMUM_RETAINED_PAYLOAD_BYTES
        {
            return None;
        }
        Some(Self {
            application_attempts,
            destination_attempts,
            provider_attempts,
            intent_attempts,
            retained_payload_bytes,
        })
    }

    pub(crate) fn admit(
        self,
        basis: UiIntentExecutionReservationBasis,
        counts: UiIntentExecutionReservationCounts,
    ) -> Result<(), UiIntentExecutionReservationDenial> {
        if counts.provider_attempts >= self.provider_attempts {
            return Err(
                UiIntentExecutionReservationDenial::ProviderCapacityExceeded {
                    intent: basis.intent,
                    provider_version: basis.provider_version,
                    maximum: self.provider_attempts,
                },
            );
        }
        if counts.intent_attempts >= self.intent_attempts {
            return Err(UiIntentExecutionReservationDenial::IntentCapacityExceeded {
                intent: basis.intent,
                maximum: self.intent_attempts,
            });
        }
        if counts.destination_attempts >= self.destination_attempts {
            return Err(
                UiIntentExecutionReservationDenial::DestinationCapacityExceeded {
                    destination: basis.destination,
                    maximum: self.destination_attempts,
                },
            );
        }
        if counts.application_attempts >= self.application_attempts {
            return Err(
                UiIntentExecutionReservationDenial::ApplicationCapacityExceeded {
                    maximum: self.application_attempts,
                },
            );
        }
        let Some(retained_after) = counts
            .retained_payload_bytes
            .checked_add(basis.retained_payload_bytes)
        else {
            return Err(
                UiIntentExecutionReservationDenial::RetainedPayloadBytesExceeded {
                    active: counts.retained_payload_bytes,
                    requested: basis.retained_payload_bytes,
                    maximum: self.retained_payload_bytes,
                },
            );
        };
        if retained_after > self.retained_payload_bytes {
            return Err(
                UiIntentExecutionReservationDenial::RetainedPayloadBytesExceeded {
                    active: counts.retained_payload_bytes,
                    requested: basis.retained_payload_bytes,
                    maximum: self.retained_payload_bytes,
                },
            );
        }
        Ok(())
    }
}
