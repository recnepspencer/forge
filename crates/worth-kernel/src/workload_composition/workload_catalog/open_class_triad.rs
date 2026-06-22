use super::catalog::WorkloadCatalog;
use super::error::WorkloadCatalogError;
use super::recipe_kind::TransformRecipe;
use super::BuiltWorkloadCatalogRecipe;

#[derive(Clone, Debug, PartialEq)]
pub struct OpenClassTriadCatalogRecipe {
    declaration: String,
    incident_faces: usize,
    transform: Option<TransformRecipe>,
    retained_replay_artifacts: bool,
}

impl OpenClassTriadCatalogRecipe {
    pub(crate) fn new(incident_faces: usize) -> Self {
        Self {
            declaration: "open-class triad workload catalog".to_string(),
            incident_faces,
            transform: None,
            retained_replay_artifacts: true,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_transform(mut self, transform: TransformRecipe) -> Self {
        self.transform = Some(transform);
        self
    }

    pub fn with_retained_replay_artifacts(mut self) -> Self {
        self.retained_replay_artifacts = true;
        self
    }

    pub fn build(self) -> Result<BuiltOpenClassTriadCatalog, WorkloadCatalogError> {
        let wire = self.build_member("open wire", WorkloadCatalog::open_wire())?;
        let sheet = self.build_member("open sheet", WorkloadCatalog::open_sheet())?;
        let fan = self.build_member(
            "open radial fan",
            WorkloadCatalog::open_shell_nmt_edge_fan(self.incident_faces),
        )?;
        Ok(BuiltOpenClassTriadCatalog { wire, sheet, fan })
    }

    fn build_member(
        &self,
        label: &str,
        mut recipe: super::catalog::WorkloadCatalogRecipe,
    ) -> Result<BuiltWorkloadCatalogRecipe, WorkloadCatalogError> {
        recipe = recipe.declared(format!("{} {label}", self.declaration));
        if let Some(transform) = self.transform {
            recipe = recipe.with_transform(transform);
        }
        if self.retained_replay_artifacts {
            recipe = recipe.with_retained_replay_artifacts();
        }
        recipe.build()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltOpenClassTriadCatalog {
    wire: BuiltWorkloadCatalogRecipe,
    sheet: BuiltWorkloadCatalogRecipe,
    fan: BuiltWorkloadCatalogRecipe,
}

impl BuiltOpenClassTriadCatalog {
    pub fn wire(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.wire
    }

    pub fn sheet(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.sheet
    }

    pub fn fan(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.fan
    }
}
