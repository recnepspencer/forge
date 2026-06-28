#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecoveryCandidateDiscoveryTrace {
    profile: String,
    physical_basis: String,
    discovery_order: u64,
}

impl RecoveryCandidateDiscoveryTrace {
    pub fn new(
        profile: impl Into<String>,
        physical_basis: impl Into<String>,
        discovery_order: u64,
    ) -> Self {
        Self {
            profile: profile.into(),
            physical_basis: physical_basis.into(),
            discovery_order,
        }
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }

    pub fn physical_basis(&self) -> &str {
        &self.physical_basis
    }

    pub const fn discovery_order(&self) -> u64 {
        self.discovery_order
    }
}
