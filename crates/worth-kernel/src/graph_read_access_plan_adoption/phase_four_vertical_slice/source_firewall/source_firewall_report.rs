use super::post_admission_forbidden_pattern::POST_ADMISSION_FORBIDDEN_GRAPH_READ_PATTERNS;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostAdmissionSourceFirewallReport {
    checked_source_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostAdmissionSourceFirewallViolation {
    source_path: String,
    forbidden_pattern: &'static str,
}

pub fn reject_post_admission_local_graph_read_residue(
    sources: &[(&str, &str)],
) -> Result<
    WorthGraphReadAccessPostAdmissionSourceFirewallReport,
    WorthGraphReadAccessPostAdmissionSourceFirewallViolation,
> {
    for (source_path, source_text) in sources {
        if let Some(forbidden_pattern) = first_forbidden_pattern(source_text) {
            return Err(WorthGraphReadAccessPostAdmissionSourceFirewallViolation {
                source_path: (*source_path).to_string(),
                forbidden_pattern,
            });
        }
    }
    Ok(WorthGraphReadAccessPostAdmissionSourceFirewallReport {
        checked_source_count: sources.len(),
    })
}

fn first_forbidden_pattern(source_text: &str) -> Option<&'static str> {
    POST_ADMISSION_FORBIDDEN_GRAPH_READ_PATTERNS
        .iter()
        .copied()
        .find(|pattern| source_text.contains(pattern))
}

impl WorthGraphReadAccessPostAdmissionSourceFirewallReport {
    pub const fn checked_source_count(&self) -> usize {
        self.checked_source_count
    }
}

impl WorthGraphReadAccessPostAdmissionSourceFirewallViolation {
    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn forbidden_pattern(&self) -> &'static str {
        self.forbidden_pattern
    }
}
