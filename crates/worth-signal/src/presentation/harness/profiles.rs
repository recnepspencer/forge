use worth_harness::facade::ExecutionProfile;

pub struct SignalProfileCatalog;

impl SignalProfileCatalog {
    pub fn serial(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::serial(name)
    }

    pub fn operational(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::operational(name)
    }

    pub fn development(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::development(name)
    }

    pub fn forensic(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::forensic(name)
    }

    pub fn replay(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::replay(name)
    }

    pub fn staged_parallel(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::staged_parallel(name)
    }

    pub fn full_parallel(name: impl Into<String>) -> ExecutionProfile {
        ExecutionProfile::full_parallel(name)
    }
}
