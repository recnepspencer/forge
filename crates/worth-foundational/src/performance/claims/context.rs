use crate::profiles::{FoundationalObservationDisposition, FoundationalProfileIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPerformanceObservationContext {
    profile_identity: FoundationalProfileIdentity,
    disposition: FoundationalObservationDisposition,
}

impl FoundationalPerformanceObservationContext {
    pub fn new(
        profile_identity: FoundationalProfileIdentity,
        disposition: FoundationalObservationDisposition,
    ) -> Self {
        Self {
            profile_identity,
            disposition,
        }
    }

    pub const fn disposition(&self) -> FoundationalObservationDisposition {
        self.disposition
    }

    pub fn profile_identity(&self) -> &FoundationalProfileIdentity {
        &self.profile_identity
    }
}
