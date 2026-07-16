#[cfg(test)]
use crate::application::support::WorthQueryCapabilityDescriptor;
use crate::application::support::WorthQueryCapabilityFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityAdmissionFailureClass {
    UnsupportedCapabilityFamily,
    DeferredCapabilityFamily,
    MissingOwningSection,
    InvalidComposedSupportPosture,
}

impl CapabilityAdmissionFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedCapabilityFamily => "unsupported_capability_family",
            Self::DeferredCapabilityFamily => "deferred_capability_family",
            Self::MissingOwningSection => "missing_owning_section",
            Self::InvalidComposedSupportPosture => "invalid_composed_support_posture",
        }
    }
}

pub type WorthQueryFacadeFailureClass = CapabilityAdmissionFailureClass;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryFacadeCounters {
    capability_lookup_count: usize,
    configuration_section_resolution_count: usize,
    unsupported_composition_denial_count: usize,
    deferred_capability_denial_count: usize,
}

impl WorthQueryFacadeCounters {
    pub fn capability_lookup_count(&self) -> usize {
        self.capability_lookup_count
    }

    pub fn configuration_section_resolution_count(&self) -> usize {
        self.configuration_section_resolution_count
    }

    pub fn unsupported_composition_denial_count(&self) -> usize {
        self.unsupported_composition_denial_count
    }

    pub fn deferred_capability_denial_count(&self) -> usize {
        self.deferred_capability_denial_count
    }

    #[cfg(test)]
    pub(crate) fn config_resolution() -> Self {
        Self {
            capability_lookup_count: 0,
            configuration_section_resolution_count: 1,
            unsupported_composition_denial_count: 0,
            deferred_capability_denial_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn admitted_lookup() -> Self {
        Self {
            capability_lookup_count: 1,
            configuration_section_resolution_count: 1,
            unsupported_composition_denial_count: 0,
            deferred_capability_denial_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn unsupported_denial() -> Self {
        Self {
            capability_lookup_count: 1,
            configuration_section_resolution_count: 1,
            unsupported_composition_denial_count: 1,
            deferred_capability_denial_count: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn deferred_denial() -> Self {
        Self {
            capability_lookup_count: 1,
            configuration_section_resolution_count: 1,
            unsupported_composition_denial_count: 0,
            deferred_capability_denial_count: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryFacadeError {
    failure_class: CapabilityAdmissionFailureClass,
    capability_family: Option<WorthQueryCapabilityFamily>,
    counters: WorthQueryFacadeCounters,
    reason: &'static str,
}

impl WorthQueryFacadeError {
    #[cfg(test)]
    pub(crate) fn capability_denied(
        descriptor: &WorthQueryCapabilityDescriptor,
        failure_class: CapabilityAdmissionFailureClass,
    ) -> Self {
        let counters = match failure_class {
            CapabilityAdmissionFailureClass::DeferredCapabilityFamily => {
                WorthQueryFacadeCounters::deferred_denial()
            }
            CapabilityAdmissionFailureClass::UnsupportedCapabilityFamily
            | CapabilityAdmissionFailureClass::MissingOwningSection
            | CapabilityAdmissionFailureClass::InvalidComposedSupportPosture => {
                WorthQueryFacadeCounters::unsupported_denial()
            }
        };
        Self {
            failure_class,
            capability_family: Some(descriptor.family()),
            counters,
            reason: descriptor.reason(),
        }
    }

    pub fn failure_class(&self) -> CapabilityAdmissionFailureClass {
        self.failure_class
    }

    pub fn capability_family(&self) -> Option<WorthQueryCapabilityFamily> {
        self.capability_family
    }

    pub fn counters(&self) -> &WorthQueryFacadeCounters {
        &self.counters
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

pub type CapabilityAdmissionError = WorthQueryFacadeError;
