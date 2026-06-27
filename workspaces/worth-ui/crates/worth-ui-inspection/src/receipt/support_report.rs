use crate::{
    UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionScopeSupportRow,
    UiInspectionSupportReason, UiInspectionSupportStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionSupportReport {
    scope: UiInspectionScope,
    status: UiInspectionSupportStatus,
    reason: Option<UiInspectionSupportReason>,
    expected_in: Option<UiInspectionMilestoneExpectation>,
}

impl UiInspectionSupportReport {
    pub(crate) fn from_scope_rows(
        scope: UiInspectionScope,
        rows: &[UiInspectionScopeSupportRow],
    ) -> Self {
        debug_assert!(rows.iter().all(|row| row.scope() == scope));
        let mut matching_rows = rows.iter().copied();
        let first_row = matching_rows
            .next()
            .expect("scope support inventory always supplies subsystem rows");
        let (status, reason, expected_in) =
            match aggregate_unsupported_truth(first_row, matching_rows) {
                UnsupportedTruth::Supported => (UiInspectionSupportStatus::Supported, None, None),
                UnsupportedTruth::Uniform {
                    reason,
                    expected_in,
                } => (UiInspectionSupportStatus::Unsupported, reason, expected_in),
                UnsupportedTruth::Conflict => (
                    UiInspectionSupportStatus::Unsupported,
                    Some(UiInspectionSupportReason::SubsystemSupportTruthConflict),
                    None,
                ),
            };

        Self {
            scope,
            status,
            reason,
            expected_in,
        }
    }

    pub fn scope(self) -> UiInspectionScope {
        self.scope
    }

    pub fn status(self) -> UiInspectionSupportStatus {
        self.status
    }

    pub fn reason(self) -> Option<UiInspectionSupportReason> {
        self.reason
    }

    pub fn expected_in(self) -> Option<UiInspectionMilestoneExpectation> {
        self.expected_in
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UnsupportedTruth {
    Supported,
    Uniform {
        reason: Option<UiInspectionSupportReason>,
        expected_in: Option<UiInspectionMilestoneExpectation>,
    },
    Conflict,
}

fn aggregate_unsupported_truth(
    first_row: UiInspectionScopeSupportRow,
    remaining_rows: impl Iterator<Item = UiInspectionScopeSupportRow>,
) -> UnsupportedTruth {
    let mut unsupported_truth = unsupported_truth_from_row(first_row);

    for row in remaining_rows {
        let row_truth = unsupported_truth_from_row(row);
        unsupported_truth = match (unsupported_truth, row_truth) {
            (UnsupportedTruth::Conflict, _) | (_, UnsupportedTruth::Conflict) => {
                UnsupportedTruth::Conflict
            }
            (UnsupportedTruth::Supported, other) => other,
            (other, UnsupportedTruth::Supported) => other,
            (
                UnsupportedTruth::Uniform {
                    reason: left_reason,
                    expected_in: left_expected_in,
                },
                UnsupportedTruth::Uniform {
                    reason: right_reason,
                    expected_in: right_expected_in,
                },
            ) if left_reason == right_reason && left_expected_in == right_expected_in => {
                UnsupportedTruth::Uniform {
                    reason: left_reason,
                    expected_in: left_expected_in,
                }
            }
            _ => UnsupportedTruth::Conflict,
        };
    }

    unsupported_truth
}

fn unsupported_truth_from_row(row: UiInspectionScopeSupportRow) -> UnsupportedTruth {
    match row.status() {
        UiInspectionSupportStatus::Supported => UnsupportedTruth::Supported,
        UiInspectionSupportStatus::Unsupported => UnsupportedTruth::Uniform {
            reason: row.reason(),
            expected_in: row.expected_in(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::UiInspectionSupportReport;
    use crate::{
        UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionScopeSupportRow,
        UiInspectionSupportReason, UiInspectionSupportStatus,
    };

    #[test]
    fn mixed_scope_rows_remain_unsupported_even_when_first_row_is_supported() {
        let rows = [
            UiInspectionScopeSupportRow::supported("inspection", UiInspectionScope::Graph),
            UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
                "query_binding",
                UiInspectionScope::Graph,
                UiInspectionMilestoneExpectation::Milestone31,
            ),
        ];

        let report = UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Graph, &rows);

        assert_eq!(report.status(), UiInspectionSupportStatus::Unsupported);
        assert_eq!(
            report.reason(),
            Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted)
        );
        assert_eq!(
            report.expected_in(),
            Some(UiInspectionMilestoneExpectation::Milestone31)
        );
    }

    #[test]
    fn fully_supported_scope_rows_project_supported_status() {
        let rows = [
            UiInspectionScopeSupportRow::supported("inspection", UiInspectionScope::Graph),
            UiInspectionScopeSupportRow::supported("query_binding", UiInspectionScope::Graph),
        ];

        let report = UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Graph, &rows);

        assert_eq!(report.status(), UiInspectionSupportStatus::Supported);
        assert_eq!(report.reason(), None);
        assert_eq!(report.expected_in(), None);
    }

    #[test]
    fn unsupported_scope_rows_keep_reason_and_expectation_for_the_requested_scope() {
        let rows = [
            UiInspectionScopeSupportRow::supported("inspection", UiInspectionScope::Measurement),
            UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
                "query_binding",
                UiInspectionScope::Measurement,
                UiInspectionMilestoneExpectation::Milestone31,
            ),
        ];

        let report =
            UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Measurement, &rows);

        assert_eq!(report.scope(), UiInspectionScope::Measurement);
        assert_eq!(report.status(), UiInspectionSupportStatus::Unsupported);
        assert_eq!(
            report.reason(),
            Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted)
        );
        assert_eq!(
            report.expected_in(),
            Some(UiInspectionMilestoneExpectation::Milestone31)
        );
    }

    #[test]
    fn conflicting_unsupported_rows_project_conflict_reason_instead_of_first_match() {
        let rows = [
            UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
                "inspection",
                UiInspectionScope::Mounting,
                UiInspectionMilestoneExpectation::Milestone31,
            ),
            UiInspectionScopeSupportRow {
                subsystem: "query_binding",
                scope: UiInspectionScope::Mounting,
                status: UiInspectionSupportStatus::Unsupported,
                reason: Some(UiInspectionSupportReason::SubsystemSupportTruthConflict),
                expected_in: None,
            },
        ];

        let report = UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Mounting, &rows);

        assert_eq!(report.status(), UiInspectionSupportStatus::Unsupported);
        assert_eq!(
            report.reason(),
            Some(UiInspectionSupportReason::SubsystemSupportTruthConflict)
        );
        assert_eq!(report.expected_in(), None);
    }
}
