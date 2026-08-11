use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RebuildMaintenanceDeclaration {
    retained_basis_label: String,
    family_label: String,
    rebuild_target_id: String,
    debt_link_artifact_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedFamilyRebuildMaintenanceDeclaration {
    retained_basis_label: String,
    family_label: String,
    rebuild_target_id: String,
}

impl DerivedFamilyRebuildMaintenanceDeclaration {
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        rebuild_target_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            rebuild_target_id: rebuild_target_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn family_label(&self) -> &str {
        &self.family_label
    }

    pub fn rebuild_target_id(&self) -> &str {
        &self.rebuild_target_id
    }
}

impl RebuildMaintenanceDeclaration {
    #[allow(dead_code)]
    pub(crate) fn new(
        retained_basis_label: impl Into<String>,
        family_label: impl Into<String>,
        rebuild_target_id: impl Into<String>,
        debt_link_artifact_id: Option<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_label: family_label.into(),
            rebuild_target_id: rebuild_target_id.into(),
            debt_link_artifact_id,
        }
    }
    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }
    pub fn family_label(&self) -> &str {
        &self.family_label
    }
    pub fn rebuild_target_id(&self) -> &str {
        &self.rebuild_target_id
    }
    pub fn debt_link_artifact_id(&self) -> Option<&str> {
        self.debt_link_artifact_id.as_deref()
    }
}
