use crate::runtime::{WorthQueryAuthorityLane, WorthQueryDerivedViewHandle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedViewIntentSeed {
    view_name: String,
    authority_lane: WorthQueryAuthorityLane,
    dependency_digest: String,
    materialization_digest: String,
    inspection_digest: String,
    row_count: usize,
}

impl WorthQueryDerivedViewIntentSeed {
    pub(crate) fn new<T>(
        view: &WorthQueryDerivedViewHandle<T>,
        authority_lane: WorthQueryAuthorityLane,
        dependency_digest: impl Into<String>,
        materialization_digest: impl Into<String>,
        inspection_digest: impl Into<String>,
        row_count: usize,
    ) -> Self {
        Self {
            view_name: view.name().to_string(),
            authority_lane,
            dependency_digest: dependency_digest.into(),
            materialization_digest: materialization_digest.into(),
            inspection_digest: inspection_digest.into(),
            row_count,
        }
    }

    pub fn request_label(&self, operation: &str) -> String {
        format!("derived-view.{operation}.{}", self.view_name)
    }

    pub fn request_input_digest(&self, operation: &str) -> String {
        match operation {
            "materialize" => format!(
                "{}:{}:{}:{}",
                self.view_name, self.dependency_digest, self.materialization_digest, self.row_count
            ),
            "inspect" => format!(
                "{}:{}:{}:{}",
                self.view_name, self.dependency_digest, self.inspection_digest, self.row_count
            ),
            other => panic!("unknown derived view operation `{other}`"),
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }

    pub fn dependency_digest(&self) -> &str {
        &self.dependency_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }
}
