use crate::PhysicalSimulationProfile;

#[derive(Debug, Clone, Copy)]
pub(crate) struct S6PressureExecutionSample {
    pub(crate) allocation_bytes: u64,
    pub(crate) queue_depth: u32,
    pub(crate) interference_events: u32,
}

impl S6PressureExecutionSample {
    pub(crate) const fn developer_smoke() -> Self {
        Self {
            allocation_bytes: 64,
            queue_depth: 1,
            interference_events: 1,
        }
    }

    pub(crate) const fn ci_certification() -> Self {
        Self {
            allocation_bytes: 128,
            queue_depth: 3,
            interference_events: 2,
        }
    }
}

pub(crate) fn sample_for_profile(profile: PhysicalSimulationProfile) -> S6PressureExecutionSample {
    match profile {
        PhysicalSimulationProfile::DeveloperSmoke => S6PressureExecutionSample::developer_smoke(),
        _ => S6PressureExecutionSample::ci_certification(),
    }
}
