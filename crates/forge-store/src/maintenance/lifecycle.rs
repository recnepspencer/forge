use serde::{Deserialize, Serialize};

use super::{MaintenanceDeclaration, MaintenanceDeclarationClass, MaintenanceDeclarationId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceExecutionStatus {
    Declared,
    Admitted,
    Started,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StartedMaintenance {
    declaration: MaintenanceDeclaration,
}

impl StartedMaintenance {
    pub(crate) fn new(declaration: MaintenanceDeclaration) -> Self {
        Self { declaration }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompletedMaintenance {
    declaration: MaintenanceDeclaration,
    last_completed_phase: String,
}

impl CompletedMaintenance {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        last_completed_phase: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            last_completed_phase: last_completed_phase.into(),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn last_completed_phase(&self) -> &str {
        &self.last_completed_phase
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FailedMaintenance {
    declaration: MaintenanceDeclaration,
    error_kind: String,
    message: String,
}

impl FailedMaintenance {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        error_kind: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            error_kind: error_kind.into(),
            message: message.into(),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn error_kind(&self) -> &str {
        &self.error_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceStatusReport {
    declaration_id: MaintenanceDeclarationId,
    declaration_class: MaintenanceDeclarationClass,
    execution_status: MaintenanceExecutionStatus,
    last_completed_phase: Option<String>,
    durable_error_kind: Option<String>,
    debt_link_artifact_id: Option<String>,
}

impl MaintenanceStatusReport {
    pub(crate) fn new(
        declaration_id: MaintenanceDeclarationId,
        declaration_class: MaintenanceDeclarationClass,
        execution_status: MaintenanceExecutionStatus,
        last_completed_phase: Option<String>,
        durable_error_kind: Option<String>,
        debt_link_artifact_id: Option<String>,
    ) -> Self {
        Self {
            declaration_id,
            declaration_class,
            execution_status,
            last_completed_phase,
            durable_error_kind,
            debt_link_artifact_id,
        }
    }

    pub fn declaration_id(&self) -> &MaintenanceDeclarationId {
        &self.declaration_id
    }

    pub fn declaration_class(&self) -> MaintenanceDeclarationClass {
        self.declaration_class
    }

    pub fn execution_status(&self) -> MaintenanceExecutionStatus {
        self.execution_status
    }

    pub fn last_completed_phase(&self) -> Option<&str> {
        self.last_completed_phase.as_deref()
    }

    pub fn durable_error_kind(&self) -> Option<&str> {
        self.durable_error_kind.as_deref()
    }

    pub fn debt_link_artifact_id(&self) -> Option<&str> {
        self.debt_link_artifact_id.as_deref()
    }
}
