use crate::{MaintenanceDeclaration, MaintenanceDeclarationClass, MaintenanceWorkDescriptor};

use super::descriptor::PersistedMaintenanceWorkDescriptor;
use super::families::declarations::PersistedMaintenanceDeclaration;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintenanceDeclarationRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub batch_id: String,
    pub declaration_class: MaintenanceDeclarationClass,
    pub declaration: MaintenanceDeclaration,
    pub retained_basis_label: Option<String>,
    pub family_label: Option<String>,
    pub debt_link_artifact_id: Option<String>,
    pub work_descriptor: MaintenanceWorkDescriptor,
    pub created_order: u64,
}

impl Serialize for MaintenanceDeclarationRecord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        PersistedMaintenanceDeclarationRecord::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MaintenanceDeclarationRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let persisted = PersistedMaintenanceDeclarationRecord::deserialize(deserializer)?;
        Self::try_from(persisted).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedMaintenanceDeclarationRecord {
    artifact_id: String,
    family_version: u32,
    batch_id: String,
    declaration_class: MaintenanceDeclarationClass,
    declaration: PersistedMaintenanceDeclaration,
    retained_basis_label: Option<String>,
    family_label: Option<String>,
    debt_link_artifact_id: Option<String>,
    #[serde(default)]
    work_descriptor: Option<PersistedMaintenanceWorkDescriptor>,
    created_order: u64,
}

impl From<&MaintenanceDeclarationRecord> for PersistedMaintenanceDeclarationRecord {
    fn from(record: &MaintenanceDeclarationRecord) -> Self {
        Self {
            artifact_id: record.artifact_id.clone(),
            family_version: record.family_version,
            batch_id: record.batch_id.clone(),
            declaration_class: record.declaration_class,
            declaration: PersistedMaintenanceDeclaration::from(&record.declaration),
            retained_basis_label: record.retained_basis_label.clone(),
            family_label: record.family_label.clone(),
            debt_link_artifact_id: record.debt_link_artifact_id.clone(),
            work_descriptor: Some(PersistedMaintenanceWorkDescriptor::from(
                &record.work_descriptor,
            )),
            created_order: record.created_order,
        }
    }
}

impl TryFrom<PersistedMaintenanceDeclarationRecord> for MaintenanceDeclarationRecord {
    type Error = String;

    fn try_from(record: PersistedMaintenanceDeclarationRecord) -> Result<Self, Self::Error> {
        let declaration = MaintenanceDeclaration::try_from(record.declaration)?;
        let work_descriptor = record
            .work_descriptor
            .map(MaintenanceWorkDescriptor::try_from)
            .transpose()?
            .unwrap_or_else(|| declaration.work_descriptor());
        Ok(Self {
            artifact_id: record.artifact_id,
            family_version: record.family_version,
            batch_id: record.batch_id,
            declaration_class: record.declaration_class,
            declaration,
            retained_basis_label: record.retained_basis_label,
            family_label: record.family_label,
            debt_link_artifact_id: record.debt_link_artifact_id,
            work_descriptor,
            created_order: record.created_order,
        })
    }
}
