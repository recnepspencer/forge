use super::WorthQueryGraphReadAccessRequirementSet;
use crate::runtime::{
    WorthQueryGraphReadOperationCapabilityRequirement,
    WorthQueryGraphReadOperationUnsupportedDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadAccessRequirementExplanationOutcome {
    RequirementSet(WorthQueryGraphReadAccessRequirementSet),
    RequiresAccessCapabilityRegistration(WorthQueryGraphReadOperationCapabilityRequirement),
    DeniedUnsupportedShape(WorthQueryGraphReadOperationUnsupportedDenial),
}

impl WorthQueryGraphReadAccessRequirementExplanationOutcome {
    pub fn requirement_set(&self) -> Option<&WorthQueryGraphReadAccessRequirementSet> {
        match self {
            Self::RequirementSet(requirement_set) => Some(requirement_set),
            Self::RequiresAccessCapabilityRegistration(_) | Self::DeniedUnsupportedShape(_) => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RequirementSet(_) => "requirement_set",
            Self::RequiresAccessCapabilityRegistration(_) => {
                "requires_access_capability_registration"
            }
            Self::DeniedUnsupportedShape(_) => "denied_unsupported_shape",
        }
    }
}
