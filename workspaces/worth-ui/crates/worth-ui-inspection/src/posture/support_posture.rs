use crate::UiInspectionSupportReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSupportStatus {
    Supported,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMilestoneExpectation {
    Milestone31,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionPosture {
    Available,
    Unsupported(UiInspectionUnsupportedPosture),
}

impl UiInspectionPosture {
    pub const fn available() -> Self {
        Self::Available
    }

    pub const fn unsupported(
        reason: UiInspectionSupportReason,
        expected_in: Option<UiInspectionMilestoneExpectation>,
    ) -> Self {
        Self::Unsupported(UiInspectionUnsupportedPosture::new(reason, expected_in))
    }

    pub fn unsupported_posture(self) -> Option<UiInspectionUnsupportedPosture> {
        match self {
            Self::Available => None,
            Self::Unsupported(posture) => Some(posture),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionUnsupportedPosture {
    reason: UiInspectionSupportReason,
    expected_in: Option<UiInspectionMilestoneExpectation>,
}

impl UiInspectionUnsupportedPosture {
    pub(crate) const fn new(
        reason: UiInspectionSupportReason,
        expected_in: Option<UiInspectionMilestoneExpectation>,
    ) -> Self {
        Self {
            reason,
            expected_in,
        }
    }

    pub fn reason(self) -> UiInspectionSupportReason {
        self.reason
    }

    pub fn expected_in(self) -> Option<UiInspectionMilestoneExpectation> {
        self.expected_in
    }
}
