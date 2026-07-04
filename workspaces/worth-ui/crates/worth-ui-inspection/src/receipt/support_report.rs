use crate::{
    UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionScopeSupportRow,
    UiInspectionSupportPosture, UiInspectionSupportReason, UiInspectionSupportStatus,
    UiInspectionSupportWorld,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionSupportReport {
    scope: UiInspectionScope,
    posture: UiInspectionSupportPosture,
    reason: Option<UiInspectionSupportReason>,
    expected_in: Option<UiInspectionMilestoneExpectation>,
    current_world: UiInspectionSupportWorld,
    expected_world: Option<UiInspectionSupportWorld>,
}

impl UiInspectionSupportReport {
    pub fn from_scope_rows(scope: UiInspectionScope, rows: &[UiInspectionScopeSupportRow]) -> Self {
        debug_assert!(rows.iter().all(|row| row.scope() == scope));
        let mut matching_rows = rows.iter().copied();
        let first_row = matching_rows
            .next()
            .expect("scope support inventory always supplies subsystem rows");
        let aggregate = aggregate_support_truth(first_row, matching_rows);

        Self {
            scope,
            posture: aggregate.posture(),
            reason: aggregate.reason(),
            expected_in: aggregate.expected_in(),
            current_world: aggregate.current_world(),
            expected_world: aggregate.expected_world(),
        }
    }

    pub fn scope(self) -> UiInspectionScope {
        self.scope
    }

    pub fn posture(self) -> UiInspectionSupportPosture {
        self.posture
    }

    pub fn status(self) -> UiInspectionSupportStatus {
        match self.posture {
            UiInspectionSupportPosture::Supported => UiInspectionSupportStatus::Supported,
            UiInspectionSupportPosture::DiagnosticOnly
            | UiInspectionSupportPosture::Unsupported
            | UiInspectionSupportPosture::WrongWorld
            | UiInspectionSupportPosture::Deferred => UiInspectionSupportStatus::Unsupported,
        }
    }

    pub fn reason(self) -> Option<UiInspectionSupportReason> {
        self.reason
    }

    pub fn expected_in(self) -> Option<UiInspectionMilestoneExpectation> {
        self.expected_in
    }

    pub fn current_world(self) -> UiInspectionSupportWorld {
        self.current_world
    }

    pub fn expected_world(self) -> Option<UiInspectionSupportWorld> {
        self.expected_world
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SupportTruth {
    Supported {
        current_world: UiInspectionSupportWorld,
    },
    DiagnosticOnly {
        current_world: UiInspectionSupportWorld,
    },
    Unsupported {
        reason: Option<UiInspectionSupportReason>,
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    },
    WrongWorld {
        expected_world: UiInspectionSupportWorld,
        observed_world: UiInspectionSupportWorld,
    },
    Deferred {
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    },
    Conflict {
        current_world: UiInspectionSupportWorld,
    },
}

impl SupportTruth {
    const fn posture(self) -> UiInspectionSupportPosture {
        match self {
            Self::Supported { .. } => UiInspectionSupportPosture::Supported,
            Self::DiagnosticOnly { .. } => UiInspectionSupportPosture::DiagnosticOnly,
            Self::Unsupported { .. } | Self::Conflict { .. } => {
                UiInspectionSupportPosture::Unsupported
            }
            Self::WrongWorld { .. } => UiInspectionSupportPosture::WrongWorld,
            Self::Deferred { .. } => UiInspectionSupportPosture::Deferred,
        }
    }

    const fn reason(self) -> Option<UiInspectionSupportReason> {
        match self {
            Self::Supported { .. } => None,
            Self::DiagnosticOnly { .. } => Some(UiInspectionSupportReason::DiagnosticOnly),
            Self::Unsupported { reason, .. } => reason,
            Self::WrongWorld { .. } => Some(UiInspectionSupportReason::WrongWorld),
            Self::Deferred { .. } => {
                Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted)
            }
            Self::Conflict { .. } => Some(UiInspectionSupportReason::SubsystemSupportTruthConflict),
        }
    }

    const fn expected_in(self) -> Option<UiInspectionMilestoneExpectation> {
        match self {
            Self::Unsupported { expected_in, .. } | Self::Deferred { expected_in, .. } => {
                expected_in
            }
            Self::Supported { .. }
            | Self::DiagnosticOnly { .. }
            | Self::WrongWorld { .. }
            | Self::Conflict { .. } => None,
        }
    }

    const fn current_world(self) -> UiInspectionSupportWorld {
        match self {
            Self::Supported { current_world }
            | Self::DiagnosticOnly { current_world }
            | Self::Unsupported { current_world, .. }
            | Self::Deferred { current_world, .. }
            | Self::Conflict { current_world } => current_world,
            Self::WrongWorld { observed_world, .. } => observed_world,
        }
    }

    const fn expected_world(self) -> Option<UiInspectionSupportWorld> {
        match self {
            Self::WrongWorld { expected_world, .. } => Some(expected_world),
            Self::Supported { .. }
            | Self::DiagnosticOnly { .. }
            | Self::Unsupported { .. }
            | Self::Deferred { .. }
            | Self::Conflict { .. } => None,
        }
    }
}

fn aggregate_support_truth(
    first_row: UiInspectionScopeSupportRow,
    remaining_rows: impl Iterator<Item = UiInspectionScopeSupportRow>,
) -> SupportTruth {
    let mut support_truth = support_truth_from_row(first_row);

    for row in remaining_rows {
        let row_truth = support_truth_from_row(row);
        support_truth = match (support_truth, row_truth) {
            (SupportTruth::Conflict { current_world }, _) => {
                SupportTruth::Conflict { current_world }
            }
            (_, SupportTruth::Conflict { current_world }) => {
                SupportTruth::Conflict { current_world }
            }
            (SupportTruth::Supported { .. }, other) => other,
            (other, SupportTruth::Supported { .. }) => other,
            (left, right) if left == right => left,
            (left, _) => SupportTruth::Conflict {
                current_world: left.current_world(),
            },
        };
    }

    support_truth
}

fn support_truth_from_row(row: UiInspectionScopeSupportRow) -> SupportTruth {
    match row.posture() {
        UiInspectionSupportPosture::Supported => SupportTruth::Supported {
            current_world: row.current_world(),
        },
        UiInspectionSupportPosture::DiagnosticOnly => SupportTruth::DiagnosticOnly {
            current_world: row.current_world(),
        },
        UiInspectionSupportPosture::Unsupported => SupportTruth::Unsupported {
            reason: row.reason(),
            expected_in: row.expected_in(),
            current_world: row.current_world(),
        },
        UiInspectionSupportPosture::WrongWorld => SupportTruth::WrongWorld {
            expected_world: row
                .expected_world()
                .expect("wrong-world support rows must carry an expected world"),
            observed_world: row.current_world(),
        },
        UiInspectionSupportPosture::Deferred => SupportTruth::Deferred {
            expected_in: row.expected_in(),
            current_world: row.current_world(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::UiInspectionSupportReport;
    use crate::{
        UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionScopeSupportRow,
        UiInspectionSupportPosture, UiInspectionSupportReason, UiInspectionSupportStatus,
        UiInspectionSupportWorld,
    };

    #[test]
    fn mixed_scope_rows_remain_deferred_even_when_first_row_is_supported() {
        let rows = [
            UiInspectionScopeSupportRow::supported(
                "inspection",
                UiInspectionScope::Graph,
                UiInspectionSupportWorld::Authoritative,
            ),
            UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
                "query_binding",
                UiInspectionScope::Graph,
                UiInspectionMilestoneExpectation::Milestone31,
            ),
        ];

        let report = UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Graph, &rows);

        assert_eq!(report.posture(), UiInspectionSupportPosture::Deferred);
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
    fn fully_supported_scope_rows_project_supported_posture() {
        let rows = [
            UiInspectionScopeSupportRow::supported(
                "inspection",
                UiInspectionScope::Graph,
                UiInspectionSupportWorld::Authoritative,
            ),
            UiInspectionScopeSupportRow::supported(
                "query_binding",
                UiInspectionScope::Graph,
                UiInspectionSupportWorld::Authoritative,
            ),
        ];

        let report = UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Graph, &rows);

        assert_eq!(report.posture(), UiInspectionSupportPosture::Supported);
        assert_eq!(report.reason(), None);
        assert_eq!(report.expected_in(), None);
    }

    #[test]
    fn diagnostic_only_scope_rows_remain_structural_support_truth() {
        let rows = [
            UiInspectionScopeSupportRow::diagnostic_only(
                "inspection",
                UiInspectionScope::Measurement,
                UiInspectionSupportWorld::Authoritative,
            ),
            UiInspectionScopeSupportRow::supported(
                "query_binding",
                UiInspectionScope::Measurement,
                UiInspectionSupportWorld::Authoritative,
            ),
        ];

        let report =
            UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Measurement, &rows);

        assert_eq!(report.posture(), UiInspectionSupportPosture::DiagnosticOnly);
        assert_eq!(
            report.reason(),
            Some(UiInspectionSupportReason::DiagnosticOnly)
        );
    }

    #[test]
    fn conflicting_non_supported_rows_project_conflict_reason() {
        let rows = [
            UiInspectionScopeSupportRow::unsupported_not_yet_admitted(
                "inspection",
                UiInspectionScope::Mounting,
                UiInspectionMilestoneExpectation::Milestone31,
            ),
            UiInspectionScopeSupportRow::wrong_world(
                "query_binding",
                UiInspectionScope::Mounting,
                UiInspectionSupportWorld::Authoritative,
                UiInspectionSupportWorld::Preview,
            ),
        ];

        let report = UiInspectionSupportReport::from_scope_rows(UiInspectionScope::Mounting, &rows);

        assert_eq!(report.posture(), UiInspectionSupportPosture::Unsupported);
        assert_eq!(
            report.reason(),
            Some(UiInspectionSupportReason::SubsystemSupportTruthConflict)
        );
        assert_eq!(report.expected_in(), None);
    }
}
