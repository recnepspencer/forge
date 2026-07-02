use crate::{UiInspectionSupportReason, UiInspectionSupportReport, UiInspectionSupportWorld};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSupportStatus {
    Supported,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMilestoneExpectation {
    Milestone31,
    Milestone32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionPosture {
    Available,
    DiagnosticOnly(UiInspectionDiagnosticOnlyPosture),
    WrongWorld(UiInspectionWrongWorldPosture),
    Deferred(UiInspectionDeferredPosture),
    Unsupported(UiInspectionUnsupportedPosture),
}

impl UiInspectionPosture {
    pub const fn available() -> Self {
        Self::Available
    }

    pub const fn diagnostic_only(current_world: UiInspectionSupportWorld) -> Self {
        Self::DiagnosticOnly(UiInspectionDiagnosticOnlyPosture::new(current_world))
    }

    pub const fn wrong_world(
        expected_world: UiInspectionSupportWorld,
        observed_world: UiInspectionSupportWorld,
    ) -> Self {
        Self::WrongWorld(UiInspectionWrongWorldPosture::new(
            expected_world,
            observed_world,
        ))
    }

    pub const fn deferred(
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self::Deferred(UiInspectionDeferredPosture::new(
            expected_in,
            current_world,
        ))
    }

    pub const fn unsupported(
        reason: UiInspectionSupportReason,
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self::Unsupported(UiInspectionUnsupportedPosture::new(
            reason,
            expected_in,
            current_world,
        ))
    }

    pub fn from_support_report(report: UiInspectionSupportReport) -> Self {
        match report.posture() {
            crate::UiInspectionSupportPosture::Supported => Self::available(),
            crate::UiInspectionSupportPosture::DiagnosticOnly => {
                Self::diagnostic_only(report.current_world())
            }
            crate::UiInspectionSupportPosture::WrongWorld => Self::wrong_world(
                report
                    .expected_world()
                    .expect("wrong-world support reports must retain expected world"),
                report.current_world(),
            ),
            crate::UiInspectionSupportPosture::Deferred => {
                Self::deferred(report.expected_in(), report.current_world())
            }
            crate::UiInspectionSupportPosture::Unsupported => Self::unsupported(
                report
                    .reason()
                    .expect("unsupported support reports must retain a typed reason"),
                report.expected_in(),
                report.current_world(),
            ),
        }
    }

    pub fn unsupported_posture(self) -> Option<UiInspectionUnsupportedPosture> {
        match self {
            Self::Available
            | Self::DiagnosticOnly(_)
            | Self::WrongWorld(_)
            | Self::Deferred(_) => None,
            Self::Unsupported(posture) => Some(posture),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionDiagnosticOnlyPosture {
    current_world: UiInspectionSupportWorld,
}

impl UiInspectionDiagnosticOnlyPosture {
    pub(crate) const fn new(current_world: UiInspectionSupportWorld) -> Self {
        Self { current_world }
    }

    pub fn current_world(self) -> UiInspectionSupportWorld {
        self.current_world
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionWrongWorldPosture {
    expected_world: UiInspectionSupportWorld,
    observed_world: UiInspectionSupportWorld,
}

impl UiInspectionWrongWorldPosture {
    pub(crate) const fn new(
        expected_world: UiInspectionSupportWorld,
        observed_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            expected_world,
            observed_world,
        }
    }

    pub fn expected_world(self) -> UiInspectionSupportWorld {
        self.expected_world
    }

    pub fn observed_world(self) -> UiInspectionSupportWorld {
        self.observed_world
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionDeferredPosture {
    expected_in: Option<UiInspectionMilestoneExpectation>,
    current_world: UiInspectionSupportWorld,
}

impl UiInspectionDeferredPosture {
    pub(crate) const fn new(
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            expected_in,
            current_world,
        }
    }

    pub fn expected_in(self) -> Option<UiInspectionMilestoneExpectation> {
        self.expected_in
    }

    pub fn current_world(self) -> UiInspectionSupportWorld {
        self.current_world
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionUnsupportedPosture {
    reason: UiInspectionSupportReason,
    expected_in: Option<UiInspectionMilestoneExpectation>,
    current_world: UiInspectionSupportWorld,
}

impl UiInspectionUnsupportedPosture {
    pub(crate) const fn new(
        reason: UiInspectionSupportReason,
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            reason,
            expected_in,
            current_world,
        }
    }

    pub fn reason(self) -> UiInspectionSupportReason {
        self.reason
    }

    pub fn expected_in(self) -> Option<UiInspectionMilestoneExpectation> {
        self.expected_in
    }

    pub fn current_world(self) -> UiInspectionSupportWorld {
        self.current_world
    }
}
