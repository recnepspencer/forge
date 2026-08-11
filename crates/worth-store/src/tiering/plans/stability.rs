use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementStabilityPlan {
    artifact_keys: Vec<String>,
    retained_basis_label: Option<String>,
}

impl PlacementStabilityPlan {
    pub(crate) fn new(
        mut artifact_keys: Vec<String>,
        retained_basis_label: Option<String>,
    ) -> Self {
        artifact_keys.sort();
        artifact_keys.dedup();
        Self {
            artifact_keys,
            retained_basis_label,
        }
    }

    pub fn artifact_keys(&self) -> &[String] {
        &self.artifact_keys
    }

    pub fn retained_basis_label(&self) -> Option<&str> {
        self.retained_basis_label.as_deref()
    }
}
