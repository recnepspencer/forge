use crate::contribution_descriptor::ContributionDescriptor;
use crate::contribution_kinds::ContributionKind;
use crate::pack_name::PackName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackRegistration {
    contribution_kind: ContributionKind,
    pack_name: PackName,
}

impl PackRegistration {
    pub fn new(descriptor: ContributionDescriptor) -> Self {
        Self {
            contribution_kind: descriptor.contribution_kind(),
            pack_name: descriptor.pack_name().clone(),
        }
    }

    pub fn contribution_kind(&self) -> ContributionKind {
        self.contribution_kind
    }

    pub fn pack_name(&self) -> &PackName {
        &self.pack_name
    }
}
