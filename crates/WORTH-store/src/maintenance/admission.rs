use serde::Serialize;

use super::{
    AdmittedMaintenanceWork, MaintenanceBatchSummary, MaintenanceDeclaration,
    MaintenanceDeclarationId, MaintenanceWorkDescriptor,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdmittedMaintenanceDeclaration {
    admitted_work: AdmittedMaintenanceWork,
}

impl AdmittedMaintenanceDeclaration {
    pub(crate) fn new(
        declaration: MaintenanceDeclaration,
        descriptor: MaintenanceWorkDescriptor,
    ) -> Self {
        Self {
            admitted_work: AdmittedMaintenanceWork::new(declaration, descriptor),
        }
    }

    pub fn declaration(&self) -> &MaintenanceDeclaration {
        self.admitted_work.declaration()
    }

    pub fn descriptor(&self) -> &MaintenanceWorkDescriptor {
        self.admitted_work.descriptor()
    }

    pub fn admitted_work(&self) -> &AdmittedMaintenanceWork {
        &self.admitted_work
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceAdmissionRejection {
    declaration_id: MaintenanceDeclarationId,
    reason: String,
}

impl MaintenanceAdmissionRejection {
    pub(crate) fn new(declaration_id: MaintenanceDeclarationId, reason: impl Into<String>) -> Self {
        Self {
            declaration_id,
            reason: reason.into(),
        }
    }

    pub fn declaration_id(&self) -> &MaintenanceDeclarationId {
        &self.declaration_id
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceAdmissionReceipt {
    batch_summary: MaintenanceBatchSummary,
    admitted_declarations: Vec<AdmittedMaintenanceDeclaration>,
    rejections: Vec<MaintenanceAdmissionRejection>,
}

impl MaintenanceAdmissionReceipt {
    pub(crate) fn new(
        batch_summary: MaintenanceBatchSummary,
        admitted_declarations: Vec<AdmittedMaintenanceDeclaration>,
        rejections: Vec<MaintenanceAdmissionRejection>,
    ) -> Self {
        Self {
            batch_summary,
            admitted_declarations,
            rejections,
        }
    }

    pub fn batch_summary(&self) -> &MaintenanceBatchSummary {
        &self.batch_summary
    }

    pub fn admitted_declarations(&self) -> &[AdmittedMaintenanceDeclaration] {
        &self.admitted_declarations
    }

    pub fn rejections(&self) -> &[MaintenanceAdmissionRejection] {
        &self.rejections
    }
}
