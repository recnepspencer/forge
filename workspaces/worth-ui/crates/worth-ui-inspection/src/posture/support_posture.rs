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
    Unsupported {
        expected_in: UiInspectionMilestoneExpectation,
    },
}
