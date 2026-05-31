pub(super) struct HarnessDiagnosticEntryView<'summary> {
    root: &'summary serde_json::value::Value,
}

impl<'summary> HarnessDiagnosticEntryView<'summary> {
    fn new(root: &'summary serde_json::value::Value) -> Self {
        Self { root }
    }

    pub(super) fn field_matches(&self, field: &str, expected: &str) -> bool {
        self.root["fields"][field].as_str() == Some(expected)
    }

    fn code_matches(&self, code: &str) -> bool {
        self.root["code"].as_str() == Some(code)
    }
}

struct HarnessDiagnosticsSummaryProjection<'summary> {
    root: &'summary serde_json::value::Value,
}

impl<'summary> HarnessDiagnosticsSummaryProjection<'summary> {
    fn new(root: &'summary serde_json::value::Value) -> Self {
        Self { root }
    }

    fn diagnostic_entries(&self, code: &str) -> Vec<HarnessDiagnosticEntryView<'summary>> {
        self.root["publication_diagnostics"]["diagnostics"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|artifact| artifact["entries"].as_array().into_iter().flatten())
            .map(HarnessDiagnosticEntryView::new)
            .filter(|entry| entry.code_matches(code))
            .collect()
    }

    fn field(&self, field: &str) -> Option<&'summary str> {
        self.root[field].as_str()
    }

    fn counter(&self, counter: &str) -> Option<u64> {
        self.root["performance_counters"][counter].as_u64()
    }
}

pub(super) fn harness_diagnostic_entries<'summary>(
    summary: &'summary serde_json::value::Value,
    code: &str,
) -> Vec<HarnessDiagnosticEntryView<'summary>> {
    HarnessDiagnosticsSummaryProjection::new(summary).diagnostic_entries(code)
}

pub(super) fn harness_diagnostic_field_matches(
    entry: &HarnessDiagnosticEntryView<'_>,
    field: &str,
    expected: &str,
) -> bool {
    entry.field_matches(field, expected)
}

pub(super) fn harness_summary_field<'summary>(
    summary: &'summary serde_json::value::Value,
    field: &str,
) -> Option<&'summary str> {
    HarnessDiagnosticsSummaryProjection::new(summary).field(field)
}

pub(super) fn harness_summary_counter(
    summary: &serde_json::value::Value,
    counter: &str,
) -> Option<u64> {
    HarnessDiagnosticsSummaryProjection::new(summary).counter(counter)
}
