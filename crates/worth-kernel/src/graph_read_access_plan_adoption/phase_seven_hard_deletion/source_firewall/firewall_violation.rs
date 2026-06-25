#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionSourceFirewallViolation {
    source_path: String,
    forbidden_pattern: String,
}

impl WorthGraphReadAccessHardDeletionSourceFirewallViolation {
    pub(crate) fn new(source_path: &str, forbidden_pattern: &str) -> Self {
        Self {
            source_path: source_path.to_string(),
            forbidden_pattern: forbidden_pattern.to_string(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn forbidden_pattern(&self) -> &str {
        &self.forbidden_pattern
    }
}
