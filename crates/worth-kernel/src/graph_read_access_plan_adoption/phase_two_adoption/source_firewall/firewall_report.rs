use super::firewall_violation::WorthGraphReadAccessPlanAdoptionSourceFirewallViolation;
#[cfg(test)]
use super::forbidden_pattern::FORBIDDEN_EXECUTION_PATTERNS;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionSourceFirewallReport {
    scanned_file_count: usize,
    violation_count: usize,
    violations: Vec<WorthGraphReadAccessPlanAdoptionSourceFirewallViolation>,
}

impl WorthGraphReadAccessPlanAdoptionSourceFirewallReport {
    #[cfg(test)]
    pub(crate) fn from_sources(
        sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let mut scanned_file_count = 0;
        let mut violations = Vec::new();

        for (path, contents) in sources {
            scanned_file_count += 1;
            violations.extend(forbidden_source_patterns(path.into(), contents.into()));
        }

        Self {
            scanned_file_count,
            violation_count: violations.len(),
            violations,
        }
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn violations(&self) -> &[WorthGraphReadAccessPlanAdoptionSourceFirewallViolation] {
        &self.violations
    }
}

#[cfg(test)]
fn forbidden_source_patterns(
    file_path: String,
    source_contents: String,
) -> Vec<WorthGraphReadAccessPlanAdoptionSourceFirewallViolation> {
    FORBIDDEN_EXECUTION_PATTERNS
        .iter()
        .filter(|forbidden_pattern| source_contents.contains(*forbidden_pattern))
        .map(|forbidden_pattern| {
            WorthGraphReadAccessPlanAdoptionSourceFirewallViolation::new(
                file_path.clone(),
                *forbidden_pattern,
            )
        })
        .collect()
}
