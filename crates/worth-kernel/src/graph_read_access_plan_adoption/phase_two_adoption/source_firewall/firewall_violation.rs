#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionSourceFirewallViolation {
    file_path: String,
    forbidden_pattern: &'static str,
}

impl WorthGraphReadAccessPlanAdoptionSourceFirewallViolation {
    #[cfg(test)]
    pub(crate) fn new(file_path: impl Into<String>, forbidden_pattern: &'static str) -> Self {
        Self {
            file_path: file_path.into(),
            forbidden_pattern,
        }
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub const fn forbidden_pattern(&self) -> &'static str {
        self.forbidden_pattern
    }
}
