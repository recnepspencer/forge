#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthGraphReadAccessDiscoveredSurface {
    source_path: String,
    evidence: String,
    test_support: bool,
}

impl WorthGraphReadAccessDiscoveredSurface {
    pub(super) fn new(
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

    pub(super) fn source_path(&self) -> &str {
        &self.source_path
    }

    pub(super) fn evidence(&self) -> &str {
        &self.evidence
    }

    pub(super) const fn is_test_support(&self) -> bool {
        self.test_support
    }
}
