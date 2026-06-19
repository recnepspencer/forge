#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerQueryDependencyRuntimeReadiness {
    QueryNineSevenSharedReadClosureReady,
    QueryNineSevenDeterministicSubmissionClosureReady,
    QueryNineEightConsumerKitClosureReady,
    StaticTestOnly,
    LegacyAssumption,
}

impl ForgeServerQueryDependencyRuntimeReadiness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryNineSevenSharedReadClosureReady => "query-9.7-shared-read-ready",
            Self::QueryNineSevenDeterministicSubmissionClosureReady => {
                "query-9.7-deterministic-submission-ready"
            }
            Self::QueryNineEightConsumerKitClosureReady => "query-9.8-consumer-kit-ready",
            Self::StaticTestOnly => "static-test-only",
            Self::LegacyAssumption => "legacy-assumption",
        }
    }
}
