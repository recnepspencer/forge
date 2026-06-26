use forge_query::facade::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryViewCapabilityReference {
    family: ForgeQueryCapabilityFamily,
    status: ForgeQueryCapabilityStatus,
    reason: &'static str,
}

impl QueryViewCapabilityReference {
    pub fn from_query_capability_descriptor(descriptor: &ForgeQueryCapabilityDescriptor) -> Self {
        Self {
            family: descriptor.family(),
            status: descriptor.status(),
            reason: descriptor.reason(),
        }
    }

    pub fn family(&self) -> ForgeQueryCapabilityFamily {
        self.family
    }

    pub fn status(&self) -> ForgeQueryCapabilityStatus {
        self.status
    }

    pub fn is_admitted(&self) -> bool {
        self.status == ForgeQueryCapabilityStatus::Admitted
    }

    pub fn digest_basis(&self) -> String {
        format!(
            "{}|{}|{}",
            self.family.as_str(),
            self.status.as_str(),
            self.reason
        )
    }
}
