use forge_query::facade::consumer_kit::ForgeQuerySupportPinReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryAuthoritySupportSummary {
    requirement_count: usize,
    matched_required_count: usize,
    finding_count: usize,
    blocking_finding_count: usize,
    satisfied: bool,
}

impl PrimitiveConstructionQueryAuthoritySupportSummary {
    pub(crate) fn from_report(report: &ForgeQuerySupportPinReport) -> Self {
        Self {
            requirement_count: report.requirement_count(),
            matched_required_count: report.matched_required_count(),
            finding_count: report.finding_count(),
            blocking_finding_count: report.blocking_finding_count(),
            satisfied: report.satisfied(),
        }
    }

    pub(crate) fn requirement_count(&self) -> usize {
        self.requirement_count
    }

    pub(crate) fn matched_required_count(&self) -> usize {
        self.matched_required_count
    }

    pub(crate) fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub(crate) fn blocking_finding_count(&self) -> usize {
        self.blocking_finding_count
    }

    pub(crate) fn satisfied(&self) -> bool {
        self.satisfied
    }
}
