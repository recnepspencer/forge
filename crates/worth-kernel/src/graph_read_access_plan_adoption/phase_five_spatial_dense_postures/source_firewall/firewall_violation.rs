#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDenseSourceFirewallViolation {
    source_path: String,
    forbidden_pattern: &'static str,
}

impl WorthGraphReadAccessSpatialDenseSourceFirewallViolation {
    pub(crate) fn new(source_path: &str, forbidden_pattern: &'static str) -> Self {
        Self {
            source_path: source_path.to_string(),
            forbidden_pattern,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn forbidden_pattern(&self) -> &'static str {
        self.forbidden_pattern
    }
}
