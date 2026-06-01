use forge_harness::facade::{HarnessRecordSummaryValue, HarnessRecordSummaryView};

pub(super) struct HarnessDiagnosticEntryView<'summary> {
    root: HarnessRecordSummaryView<'summary>,
}

impl<'summary> HarnessDiagnosticEntryView<'summary> {
    fn new(root: HarnessRecordSummaryView<'summary>) -> Self {
        Self { root }
    }

    pub(super) fn field_matches(&self, field: &str, expected: &str) -> bool {
        self.root.string_field_at(&["fields", field]) == Some(expected)
    }

    fn code_matches(&self, code: &str) -> bool {
        self.root.string_field("code") == Some(code)
    }
}

struct HarnessDiagnosticsSummaryProjection<'summary> {
    root: HarnessRecordSummaryView<'summary>,
}

impl<'summary> HarnessDiagnosticsSummaryProjection<'summary> {
    fn new(root: &'summary HarnessRecordSummaryValue) -> Self {
        Self {
            root: HarnessRecordSummaryView::new(root),
        }
    }

    fn diagnostic_entries(&self, code: &str) -> Vec<HarnessDiagnosticEntryView<'summary>> {
        self.root
            .object_array_at(&["publication_diagnostics", "diagnostics"])
            .into_iter()
            .flat_map(|artifact| artifact.object_array_at(&["entries"]))
            .map(HarnessDiagnosticEntryView::new)
            .filter(|entry| entry.code_matches(code))
            .collect()
    }

    fn field(&self, field: &str) -> Option<&'summary str> {
        self.root.string_field(field)
    }

    fn counter(&self, counter: &str) -> Option<u64> {
        self.root.u64_field_at(&["performance_counters", counter])
    }
}

pub(super) fn harness_diagnostic_entries<'summary>(
    summary: &'summary HarnessRecordSummaryValue,
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
    summary: &'summary HarnessRecordSummaryValue,
    field: &str,
) -> Option<&'summary str> {
    HarnessDiagnosticsSummaryProjection::new(summary).field(field)
}

pub(super) fn harness_summary_counter(
    summary: &HarnessRecordSummaryValue,
    counter: &str,
) -> Option<u64> {
    HarnessDiagnosticsSummaryProjection::new(summary).counter(counter)
}
