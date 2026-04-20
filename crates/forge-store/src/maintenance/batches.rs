use serde::{Deserialize, Serialize};

use super::MaintenanceDeclaration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceBatchClass {
    Retention,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceBatch {
    batch_id: String,
    batch_class: MaintenanceBatchClass,
    declarations: Vec<MaintenanceDeclaration>,
}

impl MaintenanceBatch {
    pub(crate) fn new(
        batch_id: impl Into<String>,
        batch_class: MaintenanceBatchClass,
        declarations: Vec<MaintenanceDeclaration>,
    ) -> Self {
        Self {
            batch_id: batch_id.into(),
            batch_class,
            declarations,
        }
    }

    pub fn batch_id(&self) -> &str {
        &self.batch_id
    }

    pub fn batch_class(&self) -> MaintenanceBatchClass {
        self.batch_class
    }

    pub fn declarations(&self) -> &[MaintenanceDeclaration] {
        &self.declarations
    }

    pub fn summary(&self) -> MaintenanceBatchSummary {
        MaintenanceBatchSummary {
            batch_id: self.batch_id.clone(),
            batch_class: self.batch_class,
            declaration_count: self.declarations.len() as u64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceBatchSummary {
    batch_id: String,
    batch_class: MaintenanceBatchClass,
    declaration_count: u64,
}

impl MaintenanceBatchSummary {
    pub(crate) fn new(
        batch_id: impl Into<String>,
        batch_class: MaintenanceBatchClass,
        declaration_count: u64,
    ) -> Self {
        Self {
            batch_id: batch_id.into(),
            batch_class,
            declaration_count,
        }
    }

    pub fn batch_id(&self) -> &str {
        &self.batch_id
    }

    pub fn batch_class(&self) -> MaintenanceBatchClass {
        self.batch_class
    }

    pub fn declaration_count(&self) -> u64 {
        self.declaration_count
    }
}
