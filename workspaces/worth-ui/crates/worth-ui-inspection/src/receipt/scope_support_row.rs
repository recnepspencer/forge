use crate::{
    UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionSupportPosture,
    UiInspectionSupportReason, UiInspectionSupportStatus, UiInspectionSupportWorld,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionScopeSupportRow {
    pub(crate) subsystem: &'static str,
    pub(crate) scope: UiInspectionScope,
    pub(crate) posture: UiInspectionSupportPosture,
    pub(crate) reason: Option<UiInspectionSupportReason>,
    pub(crate) expected_in: Option<UiInspectionMilestoneExpectation>,
    pub(crate) current_world: UiInspectionSupportWorld,
    pub(crate) expected_world: Option<UiInspectionSupportWorld>,
}

impl UiInspectionScopeSupportRow {
    pub const fn supported(
        subsystem: &'static str,
        scope: UiInspectionScope,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            subsystem,
            scope,
            posture: UiInspectionSupportPosture::Supported,
            reason: None,
            expected_in: None,
            current_world,
            expected_world: None,
        }
    }

    pub const fn diagnostic_only(
        subsystem: &'static str,
        scope: UiInspectionScope,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            subsystem,
            scope,
            posture: UiInspectionSupportPosture::DiagnosticOnly,
            reason: Some(UiInspectionSupportReason::DiagnosticOnly),
            expected_in: None,
            current_world,
            expected_world: None,
        }
    }

    pub const fn unsupported(
        subsystem: &'static str,
        scope: UiInspectionScope,
        reason: UiInspectionSupportReason,
        expected_in: Option<UiInspectionMilestoneExpectation>,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            subsystem,
            scope,
            posture: UiInspectionSupportPosture::Unsupported,
            reason: Some(reason),
            expected_in,
            current_world,
            expected_world: None,
        }
    }

    pub const fn wrong_world(
        subsystem: &'static str,
        scope: UiInspectionScope,
        expected_world: UiInspectionSupportWorld,
        observed_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            subsystem,
            scope,
            posture: UiInspectionSupportPosture::WrongWorld,
            reason: Some(UiInspectionSupportReason::WrongWorld),
            expected_in: None,
            current_world: observed_world,
            expected_world: Some(expected_world),
        }
    }

    pub const fn deferred(
        subsystem: &'static str,
        scope: UiInspectionScope,
        expected_in: UiInspectionMilestoneExpectation,
        current_world: UiInspectionSupportWorld,
    ) -> Self {
        Self {
            subsystem,
            scope,
            posture: UiInspectionSupportPosture::Deferred,
            reason: Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
            expected_in: Some(expected_in),
            current_world,
            expected_world: None,
        }
    }

    pub const fn unsupported_not_yet_admitted(
        subsystem: &'static str,
        scope: UiInspectionScope,
        expected_in: UiInspectionMilestoneExpectation,
    ) -> Self {
        Self::deferred(
            subsystem,
            scope,
            expected_in,
            UiInspectionSupportWorld::Authoritative,
        )
    }

    pub fn subsystem(self) -> &'static str {
        self.subsystem
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
