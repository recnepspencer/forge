use super::error::WorkloadCatalogError;
use super::query::{query_backed_catalog_declaration, query_backed_catalog_support};
use super::recipe_kind::{
    RetainedReplayRecipe, TransformRecipe, WorkloadCatalogRecipeKind, WorkloadCatalogSupportPosture,
};
use super::recipe_pipeline::build_catalog_workload;
use crate::workload_composition::WorthWorkload;

pub struct WorkloadCatalog;

impl WorkloadCatalog {
    pub fn cube() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::Cube)
    }

    pub fn tetrahedron() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::Tetrahedron)
    }

    pub fn single_face_loop() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::SingleFaceLoop)
    }

    pub fn coplanar_overlap_storm() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::CoplanarOverlapStorm)
    }

    pub fn thin_feature_wall() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::ThinFeatureWall)
    }

    pub fn dirty_self_intersecting_loop() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::DirtySelfIntersectingLoop)
    }

    pub fn high_valence_vertex() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::HighValenceVertex)
    }

    pub fn open_sheet() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenSheet)
    }

    pub fn transform_cycle() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::TransformCycle)
    }

    pub fn retained_cancellation_chain() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::RetainedCancellationChain)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogRecipe {
    kind: WorkloadCatalogRecipeKind,
    declaration: String,
    transform_recipe: Option<TransformRecipe>,
    retained_replay_recipe: Option<RetainedReplayRecipe>,
}

impl WorkloadCatalogRecipe {
    fn new(kind: WorkloadCatalogRecipeKind) -> Self {
        Self {
            kind,
            declaration: kind.default_declaration().to_string(),
            transform_recipe: None,
            retained_replay_recipe: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_transform(mut self, transform_recipe: TransformRecipe) -> Self {
        self.transform_recipe = Some(transform_recipe);
        self
    }

    pub fn inspect_support(&self) -> Result<WorkloadCatalogSupportReceipt, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        WorkloadCatalogSupportReceipt::new(&declaration, self.support_posture())
    }

    pub fn build(self) -> Result<BuiltWorkloadCatalogRecipe, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        let support = WorkloadCatalogSupportReceipt::new(&declaration, self.support_posture())?;
        if support.posture() != WorkloadCatalogSupportPosture::Admitted {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe: self.kind,
                reason: support.human_reason().to_string(),
            });
        }
        let workload = build_catalog_workload(
            self.kind,
            &self.declaration,
            self.transform_recipe
                .unwrap_or_else(|| self.kind.default_transform_recipe()),
            self.retained_replay_recipe
                .unwrap_or_else(|| self.kind.default_retained_replay_recipe()),
        )?;
        Ok(BuiltWorkloadCatalogRecipe {
            recipe: self.kind,
            declaration,
            support,
            workload,
        })
    }

    fn declaration_receipt(
        &self,
    ) -> Result<WorkloadCatalogDeclarationReceipt, WorkloadCatalogError> {
        reject_blank_declaration(&self.declaration)?;
        WorkloadCatalogDeclarationReceipt::new(self.kind, &self.declaration)
    }

    fn support_posture(&self) -> WorkloadCatalogSupportPosture {
        if self.kind.is_admitted_now() {
            WorkloadCatalogSupportPosture::Admitted
        } else {
            WorkloadCatalogSupportPosture::Unsupported
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltWorkloadCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    workload: WorthWorkload,
}

impl BuiltWorkloadCatalogRecipe {
    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        &self.support
    }

    pub fn workload(&self) -> &WorthWorkload {
        &self.workload
    }

    pub fn into_workload(self) -> WorthWorkload {
        self.workload
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogDeclarationReceipt {
    recipe: WorkloadCatalogRecipeKind,
    declaration: String,
    query_declaration_digest: String,
    query_envelope_digest: String,
    query_handle_digest: String,
}

impl WorkloadCatalogDeclarationReceipt {
    fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: &str,
    ) -> Result<Self, WorkloadCatalogError> {
        let query_receipt = query_backed_catalog_declaration(recipe, declaration)?;
        Ok(Self {
            recipe,
            declaration: declaration.to_string(),
            query_declaration_digest: query_receipt.declaration_digest().to_string(),
            query_envelope_digest: query_receipt.envelope_digest().to_string(),
            query_handle_digest: query_receipt.handle_digest().to_string(),
        })
    }

    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn query_envelope_digest(&self) -> &str {
        &self.query_envelope_digest
    }

    pub fn query_handle_digest(&self) -> &str {
        &self.query_handle_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogSupportReceipt {
    recipe: WorkloadCatalogRecipeKind,
    posture: WorkloadCatalogSupportPosture,
    query_support_digest: String,
    human_reason: String,
}

impl WorkloadCatalogSupportReceipt {
    fn new(
        declaration: &WorkloadCatalogDeclarationReceipt,
        posture: WorkloadCatalogSupportPosture,
    ) -> Result<Self, WorkloadCatalogError> {
        let query_receipt = query_backed_catalog_support(
            declaration.recipe(),
            declaration.declaration(),
            posture,
            declaration.query_declaration_digest(),
        )?;
        let human_reason = match posture {
            WorkloadCatalogSupportPosture::Admitted => {
                format!("{} is admitted", declaration.recipe().human_name())
            }
            WorkloadCatalogSupportPosture::Unsupported => {
                format!(
                    "{} is not yet supported because the catalog only admits clean, receipt-backed workload recipes today; the self-intersecting branch must deny instead of fabricating a workload",
                    declaration.recipe().human_name()
                )
            }
        };
        Ok(Self {
            recipe: declaration.recipe(),
            posture,
            query_support_digest: query_receipt.declaration_digest().to_string(),
            human_reason,
        })
    }

    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn posture(&self) -> WorkloadCatalogSupportPosture {
        self.posture
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

fn reject_blank_declaration(declaration: &str) -> Result<(), WorkloadCatalogError> {
    if declaration.trim().is_empty() {
        Err(WorkloadCatalogError::MissingDeclaration)
    } else {
        Ok(())
    }
}
