#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerQueryDependencyScopePosture {
    QueryFamilyScoped,
    ConsumerKitScoped,
    StaticTestOnly,
    Unclassified,
}

impl ForgeServerQueryDependencyScopePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryFamilyScoped => "query-family-scoped",
            Self::ConsumerKitScoped => "consumer-kit-scoped",
            Self::StaticTestOnly => "static-test-only",
            Self::Unclassified => "unclassified",
        }
    }
}
