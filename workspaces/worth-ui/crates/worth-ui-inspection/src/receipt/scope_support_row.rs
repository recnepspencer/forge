use crate::{
    UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionSupportReason,
    UiInspectionSupportStatus,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionScopeSupportRow {
    pub(crate) subsystem: &'static str,
    pub(crate) scope: UiInspectionScope,
    pub(crate) status: UiInspectionSupportStatus,
    pub(crate) reason: Option<UiInspectionSupportReason>,
    pub(crate) expected_in: Option<UiInspectionMilestoneExpectation>,
}

impl UiInspectionScopeSupportRow {
    pub const fn supported(subsystem: &'static str, scope: UiInspectionScope) -> Self {
        Self {
            subsystem,
            scope,
            status: UiInspectionSupportStatus::Supported,
            reason: None,
            expected_in: None,
        }
    }

    pub const fn unsupported(
        subsystem: &'static str,
        scope: UiInspectionScope,
        reason: UiInspectionSupportReason,
        expected_in: Option<UiInspectionMilestoneExpectation>,
    ) -> Self {
        Self {
            subsystem,
            scope,
            status: UiInspectionSupportStatus::Unsupported,
            reason: Some(reason),
            expected_in,
        }
    }

    pub const fn unsupported_not_yet_admitted(
        subsystem: &'static str,
        scope: UiInspectionScope,
        expected_in: UiInspectionMilestoneExpectation,
    ) -> Self {
        Self::unsupported(
            subsystem,
            scope,
            UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted,
            Some(expected_in),
        )
    }

    pub fn subsystem(self) -> &'static str {
        self.subsystem
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
