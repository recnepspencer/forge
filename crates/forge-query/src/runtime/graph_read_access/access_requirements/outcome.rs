use super::ForgeQueryGraphReadAccessRequirementSet;
use crate::runtime::{
    ForgeQueryGraphReadOperationCapabilityRequirement,
    ForgeQueryGraphReadOperationUnsupportedDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessRequirementExplanationOutcome {
    RequirementSet(ForgeQueryGraphReadAccessRequirementSet),
    RequiresAccessCapabilityRegistration(ForgeQueryGraphReadOperationCapabilityRequirement),
    DeniedUnsupportedShape(ForgeQueryGraphReadOperationUnsupportedDenial),
}

impl ForgeQueryGraphReadAccessRequirementExplanationOutcome {
    pub fn requirement_set(&self) -> Option<&ForgeQueryGraphReadAccessRequirementSet> {
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
