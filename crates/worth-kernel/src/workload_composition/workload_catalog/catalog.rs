use super::error::WorkloadCatalogError;
use super::query::{query_backed_catalog_declaration, query_backed_catalog_support};
use super::recipe_kind::{
    RetainedReplayRecipe, TransformRecipe, WorkloadCatalogRecipeKind,
    WorkloadCatalogSupportPosture, WorkloadTopologyBreadth,
};
use super::recipe_pipeline::build_catalog_workload;
use crate::workload_composition::WorthWorkload;
use topology::facade::{
    TopologySeed, TopologySeedCleanFailReceipt, TopologySeedNeighborhoodReceipt,
};

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
    topology_breadth: WorkloadTopologyBreadth,
}

impl WorkloadCatalogRecipe {
    fn new(kind: WorkloadCatalogRecipeKind) -> Self {
        Self {
            kind,
            declaration: kind.default_declaration().to_string(),
            transform_recipe: None,
            retained_replay_recipe: None,
            topology_breadth: WorkloadTopologyBreadth::Default,
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

    pub fn with_topology_breadth(mut self, topology_breadth: WorkloadTopologyBreadth) -> Self {
        self.topology_breadth = topology_breadth;
        self
    }

    pub fn with_retained_replay_artifacts(mut self) -> Self {
        self.retained_replay_recipe = Some(RetainedReplayRecipe::RetainedCancellationChain);
        self
    }

    pub fn inspect_support(&self) -> Result<WorkloadCatalogSupportReceipt, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision())
    }

    pub fn build(self) -> Result<BuiltWorkloadCatalogRecipe, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        let support = WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision())?;
        if support.posture() != WorkloadCatalogSupportPosture::Admitted {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe: self.kind,
                reason: support.human_reason().to_string(),
            });
        }
        let workload_build = build_catalog_workload(
            self.kind,
            &self.declaration,
            self.transform_recipe
                .unwrap_or_else(|| self.kind.default_transform_recipe()),
            self.retained_replay_recipe
                .unwrap_or_else(|| self.kind.default_retained_replay_recipe()),
            self.topology_breadth,
        )?;
        let topology_neighborhood = workload_build.topology_neighborhood().cloned();
        Ok(BuiltWorkloadCatalogRecipe {
            recipe: self.kind,
            declaration,
            support,
            workload: workload_build.workload(),
            topology_neighborhood,
        })
    }

    pub fn build_clean_fail(self) -> Result<BuiltCleanFailCatalogRecipe, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        let support =
            WorkloadCatalogSupportReceipt::new(&declaration, self.clean_fail_support_decision()?)?;
        let topology_clean_fail = match self.kind {
            WorkloadCatalogRecipeKind::DirtySelfIntersectingLoop => {
                TopologySeed::self_intersecting_loop()
                    .with_declaration(format!("topology seed for {}", self.declaration))
                    .build()
                    .expect_err("dirty self-intersecting loop must clean-fail")
            }
            _ => {
                return Err(WorkloadCatalogError::UnsupportedRecipe {
                    recipe: self.kind,
                    reason: "only dirty workload recipes can be built through the clean-fail catalog lane".to_string(),
                });
            }
        };
        Ok(BuiltCleanFailCatalogRecipe {
            recipe: self.kind,
            declaration,
            support,
            topology_clean_fail,
        })
    }

    fn declaration_receipt(
        &self,
    ) -> Result<WorkloadCatalogDeclarationReceipt, WorkloadCatalogError> {
        reject_blank_declaration(&self.declaration)?;
        WorkloadCatalogDeclarationReceipt::new(self.kind, &self.declaration)
    }

    fn support_decision(&self) -> WorkloadCatalogSupportDecision {
        if let Some(denial) = self.explicit_topology_breadth_denial() {
            return WorkloadCatalogSupportDecision::unsupported(denial);
        }

        if self.kind.is_admitted_now() {
            WorkloadCatalogSupportDecision::admitted(format!(
                "{} is admitted",
                self.kind.human_name()
            ))
        } else {
            WorkloadCatalogSupportDecision::unsupported(format!(
                "{} is not yet supported because the catalog only admits clean, receipt-backed workload recipes today; the self-intersecting branch must deny instead of fabricating a workload",
                self.kind.human_name()
            ))
        }
    }

    fn explicit_topology_breadth_denial(&self) -> Option<String> {
        match self.topology_breadth {
            WorkloadTopologyBreadth::HighValenceVertex { valence }
                if self.kind == WorkloadCatalogRecipeKind::HighValenceVertex
                    && !(3..=16).contains(&valence) =>
            {
                Some(format!(
                    "high valence vertex workload recipe supports valence 3 through 16 today; valence {valence} needs an explicit widening phase"
                ))
            }
            _ => None,
        }
    }

    fn clean_fail_support_decision(
        &self,
    ) -> Result<WorkloadCatalogSupportDecision, WorkloadCatalogError> {
        match self.kind {
            WorkloadCatalogRecipeKind::DirtySelfIntersectingLoop => {
                Ok(WorkloadCatalogSupportDecision::admitted(
                    "dirty self-intersecting loop workload is admitted only as topology-backed clean-fail evidence".to_string(),
                ))
            }
            _ => Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe: self.kind,
                reason: "clean-fail catalog lane is only for dirty workload recipes".to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorkloadCatalogSupportDecision {
    posture: WorkloadCatalogSupportPosture,
    human_reason: String,
}

impl WorkloadCatalogSupportDecision {
    fn admitted(human_reason: String) -> Self {
        Self {
            posture: WorkloadCatalogSupportPosture::Admitted,
            human_reason,
        }
    }

    fn unsupported(human_reason: String) -> Self {
        Self {
            posture: WorkloadCatalogSupportPosture::Unsupported,
            human_reason,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltWorkloadCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    workload: WorthWorkload,
    topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltCleanFailCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    topology_clean_fail: TopologySeedCleanFailReceipt,
}

impl BuiltCleanFailCatalogRecipe {
    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        &self.support
    }

    pub fn topology_clean_fail(&self) -> &TopologySeedCleanFailReceipt {
        &self.topology_clean_fail
    }

    pub fn into_topology_clean_fail(self) -> TopologySeedCleanFailReceipt {
        self.topology_clean_fail
    }
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

    pub fn topology_neighborhood(&self) -> Option<&TopologySeedNeighborhoodReceipt> {
        self.topology_neighborhood.as_ref()
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
        decision: WorkloadCatalogSupportDecision,
    ) -> Result<Self, WorkloadCatalogError> {
        let query_receipt = query_backed_catalog_support(
            declaration.recipe(),
            declaration.declaration(),
            decision.posture,
            declaration.query_declaration_digest(),
        )?;
        Ok(Self {
            recipe: declaration.recipe(),
            posture: decision.posture,
            query_support_digest: query_receipt.declaration_digest().to_string(),
            human_reason: decision.human_reason,
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
