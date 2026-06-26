#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthGraphReadAccessDiscoveredSurface {
    source_path: String,
    evidence: String,
    test_support: bool,
}

impl WorthGraphReadAccessDiscoveredSurface {
    pub(crate) fn new(
        source_path: impl Into<String>,
        evidence: impl Into<String>,
        test_support: bool,
    ) -> Self {
        Self {
            source_path: source_path.into(),
            evidence: evidence.into(),
            test_support,
        }
    }

    pub(crate) fn source_path(&self) -> &str {
        &self.source_path
    }

    pub(crate) fn evidence(&self) -> &str {
        &self.evidence
    }

    pub(crate) const fn is_test_support(&self) -> bool {
        self.test_support
    }
}
