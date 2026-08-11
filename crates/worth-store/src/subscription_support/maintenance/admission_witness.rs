use super::descriptor::SupportMaintenanceDescriptor;
use crate::MaintenanceWorkClass;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceAdmissionWitness {
    maintenance_key: String,
    declaration_id: String,
    descriptor_digest: String,
    maintenance_work_class: MaintenanceWorkClass,
}

impl SupportMaintenanceAdmissionWitness {
    pub(crate) fn new(descriptor: &SupportMaintenanceDescriptor) -> Self {
        Self {
            maintenance_key: descriptor.maintenance_key().to_string(),
            declaration_id: descriptor
                .descriptor()
                .declaration_id()
                .as_str()
                .to_string(),
            descriptor_digest: descriptor.descriptor_digest().to_string(),
            maintenance_work_class: descriptor.descriptor().work_class(),
        }
    }

    pub fn maintenance_key(&self) -> &str {
        &self.maintenance_key
    }

    pub fn declaration_id(&self) -> &str {
        &self.declaration_id
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub fn maintenance_work_class(&self) -> MaintenanceWorkClass {
        self.maintenance_work_class
    }
}
