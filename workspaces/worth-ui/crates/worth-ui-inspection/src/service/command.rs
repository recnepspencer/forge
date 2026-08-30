use super::{UiRuntimeServiceInspectionCost, UiRuntimeServiceInspectionSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommandRouteScopeInspection {
    Application,
    Surface,
    ActiveRegion,
    FocusedControl,
    ActivePortal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommandRouteLossInspectionReason {
    LowerScopePrecedence,
    LowerDeclaredPriority,
    LowerSpecificity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommandRouteLossInspection {
    command: String,
    reason: UiCommandRouteLossInspectionReason,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiCommandWonInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    command: String,
    scope: UiCommandRouteScopeInspection,
    losers: Box<[UiCommandRouteLossInspection]>,
    cost: UiRuntimeServiceInspectionCost,
}

impl UiCommandRouteLossInspection {
    pub fn new(command: String, reason: UiCommandRouteLossInspectionReason) -> Self {
        Self { command, reason }
    }
    pub fn command(&self) -> &str {
        &self.command
    }
    pub const fn reason(&self) -> UiCommandRouteLossInspectionReason {
        self.reason
    }
}

impl UiCommandWonInspectionSummary {
    pub fn new(
        source: UiRuntimeServiceInspectionSource,
        command: String,
        scope: UiCommandRouteScopeInspection,
        losers: Box<[UiCommandRouteLossInspection]>,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            command,
            scope,
            losers,
            cost,
        }
    }

    pub const fn source(&self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub fn command(&self) -> &str {
        &self.command
    }
    pub const fn scope(&self) -> UiCommandRouteScopeInspection {
        self.scope
    }
    pub fn losers(&self) -> &[UiCommandRouteLossInspection] {
        &self.losers
    }
    pub const fn cost(&self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}
