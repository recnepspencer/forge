use super::{
    ForgeQueryGraphReadOperationCapabilityRequirement, ForgeQueryGraphReadOperationResolution,
    ForgeQueryGraphReadOperationUnsupportedDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadOperationOutcome {
    Resolved(ForgeQueryGraphReadOperationResolution),
    RequiresAccessCapabilityRegistration(ForgeQueryGraphReadOperationCapabilityRequirement),
    DeniedUnsupportedShape(ForgeQueryGraphReadOperationUnsupportedDenial),
}

impl ForgeQueryGraphReadOperationOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Resolved(_) => "resolved",
            Self::RequiresAccessCapabilityRegistration(_) => {
                "requires_access_capability_registration"
            }
            Self::DeniedUnsupportedShape(_) => "denied_unsupported_shape",
        }
    }

    pub fn resolved(&self) -> Option<&ForgeQueryGraphReadOperationResolution> {
        match self {
            Self::Resolved(resolution) => Some(resolution),
            Self::RequiresAccessCapabilityRegistration(_) | Self::DeniedUnsupportedShape(_) => None,
        }
    }
}
