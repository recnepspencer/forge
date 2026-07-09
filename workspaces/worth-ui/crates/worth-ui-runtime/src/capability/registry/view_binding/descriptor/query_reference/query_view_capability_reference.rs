use worth_query::facade::{
    WorthQueryCapabilityDescriptor, WorthQueryCapabilityFamily, WorthQueryCapabilityStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryViewCapabilityReference {
    family: WorthQueryCapabilityFamily,
    status: WorthQueryCapabilityStatus,
    reason: &'static str,
}

impl QueryViewCapabilityReference {
    pub fn from_query_capability_descriptor(descriptor: &WorthQueryCapabilityDescriptor) -> Self {
        Self {
            family: descriptor.family(),
            status: descriptor.status(),
            reason: descriptor.reason(),
        }
    }

    pub fn family(&self) -> WorthQueryCapabilityFamily {
        self.family
    }

    pub fn status(&self) -> WorthQueryCapabilityStatus {
        self.status
    }

    pub fn is_admitted(&self) -> bool {
        self.status == WorthQueryCapabilityStatus::Admitted
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
