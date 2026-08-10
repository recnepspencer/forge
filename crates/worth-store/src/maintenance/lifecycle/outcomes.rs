use serde::Serialize;

use super::super::{MaintenanceDeclaration, MaintenanceFailureKind, MaintenanceWorkDescriptor};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompletedMaintenance {
    declaration: MaintenanceDeclaration,
    descriptor: MaintenanceWorkDescriptor,
    last_completed_phase: String,
}

impl CompletedMaintenance {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
        last_completed_phase: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            descriptor,
            last_completed_phase: last_completed_phase.into(),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        &self.descriptor
    }

    pub fn last_completed_phase(&self) -> &str {
        &self.last_completed_phase
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FailedMaintenance {
    declaration: MaintenanceDeclaration,
    descriptor: Option<MaintenanceWorkDescriptor>,
    failure_kind: MaintenanceFailureKind,
    error_kind: String,
    message: String,
}

impl FailedMaintenance {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: Option<MaintenanceWorkDescriptor>,
        failure_kind: MaintenanceFailureKind,
        error_kind: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            declaration,
            descriptor,
            failure_kind,
            error_kind: error_kind.into(),
            message: message.into(),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        &self.declaration
    }

    pub fn descriptor(&self) -> Option<&MaintenanceWorkDescriptor> {
        self.descriptor.as_ref()
    }

    pub fn failure_kind(&self) -> MaintenanceFailureKind {
        self.failure_kind
    }

    pub fn error_kind(&self) -> &str {
        &self.error_kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
