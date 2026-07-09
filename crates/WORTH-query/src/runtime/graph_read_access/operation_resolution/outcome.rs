use super::{
    WorthQueryGraphReadOperationCapabilityRequirement, WorthQueryGraphReadOperationResolution,
    WorthQueryGraphReadOperationUnsupportedDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadOperationOutcome {
    Resolved(WorthQueryGraphReadOperationResolution),
    RequiresAccessCapabilityRegistration(WorthQueryGraphReadOperationCapabilityRequirement),
    DeniedUnsupportedShape(WorthQueryGraphReadOperationUnsupportedDenial),
}

impl WorthQueryGraphReadOperationOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Resolved(_) => "resolved",
            Self::RequiresAccessCapabilityRegistration(_) => {
                "requires_access_capability_registration"
            }
            Self::DeniedUnsupportedShape(_) => "denied_unsupported_shape",
        }
    }

    pub fn resolved(&self) -> Option<&WorthQueryGraphReadOperationResolution> {
        match self {
            Self::Resolved(resolution) => Some(resolution),
            Self::RequiresAccessCapabilityRegistration(_) | Self::DeniedUnsupportedShape(_) => None,
        }
    }
}
