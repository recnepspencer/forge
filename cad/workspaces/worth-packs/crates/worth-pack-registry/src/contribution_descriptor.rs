use crate::contribution_kind::ContributionKind;
use crate::pack_name::PackName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContributionDescriptor {
    contribution_kind: ContributionKind,
    pack_name: PackName,
}

impl ContributionDescriptor {
    pub fn new(contribution_kind: ContributionKind, pack_name: PackName) -> Self {
        Self {
            contribution_kind,
            pack_name,
        }
    }

    pub fn contribution_kind(&self) -> ContributionKind {
        self.contribution_kind
    }

    pub fn pack_name(&self) -> &PackName {
        &self.pack_name
    }
}
