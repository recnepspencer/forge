use super::error::WorkloadCatalogError;
use super::grazing_basket_spec::GrazingBasketStackSpec;
use super::open_class_triad::OpenClassTriadCatalogRecipe;
use super::recipe_kind::{
    RetainedReplayRecipe, TransformRecipe, WorkloadCatalogRecipeKind,
    WorkloadCatalogSupportPosture, WorkloadTopologyBreadth,
};
use super::recipe_pipeline::build_catalog_workload;
use super::support_receipt::{
    WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportDecision,
    WorkloadCatalogSupportReceipt,
};
use super::topology_construction_plan::WorkloadCatalogTopologyConstructionPlan;
use super::{BuiltCleanFailCatalogRecipe, BuiltWorkloadCatalogRecipe};
use topology::facade::{
    NmtTopologyConstructionReceipt, OpenLayerStackSpec, OpenRadialFanSpec, OpenSheetPatchSpec,
    OpenWireChainSpec, TopologySeed,
};

pub const HIGH_VALENCE_VERTEX_MAX_ADMITTED_VALENCE: usize = 128;

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

    pub fn mixed_surface_kill_box() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::MixedSurfaceKillBox)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::OpenSheet(
                OpenSheetPatchSpec::new(),
            ))
    }

    pub fn open_wire() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenWire)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::OpenWire(
                OpenWireChainSpec::new(),
            ))
    }

    pub fn open_sheet() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenSheet)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::OpenSheet(
                OpenSheetPatchSpec::new(),
            ))
    }

    pub fn open_shell_nmt_edge_fan(incident_faces: usize) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenShellNmtEdgeFan)
            .with_topology_construction_plan(
                WorkloadCatalogTopologyConstructionPlan::OpenRadialFan(
                    OpenRadialFanSpec::new().incident_faces(incident_faces),
                ),
            )
    }

    pub fn open_layer_stack(spec: OpenLayerStackSpec) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::OpenLayerStack)
            .with_topology_construction_plan(
                WorkloadCatalogTopologyConstructionPlan::OpenLayerStack(spec),
            )
    }

    pub fn grazing_open_shell_basket_stack(spec: GrazingBasketStackSpec) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::GrazingBasketStack)
            .with_topology_construction_plan(
                WorkloadCatalogTopologyConstructionPlan::OpenLayerStack(
                    spec.into_open_layer_stack_spec(),
                ),
            )
    }

    pub fn open_class_triad(incident_faces: usize) -> OpenClassTriadCatalogRecipe {
        OpenClassTriadCatalogRecipe::new(incident_faces)
    }

    pub fn transform_cycle() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::TransformCycle)
    }

    pub fn retained_cancellation_chain() -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::RetainedCancellationChain)
    }

    pub fn from_topology_construction(
        construction: NmtTopologyConstructionReceipt,
    ) -> WorkloadCatalogRecipe {
        WorkloadCatalogRecipe::new(WorkloadCatalogRecipeKind::NmtTopologyConstruction)
            .with_topology_construction_plan(WorkloadCatalogTopologyConstructionPlan::Receipt(
                construction,
            ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogRecipe {
    kind: WorkloadCatalogRecipeKind,
    declaration: String,
    transform_recipe: Option<TransformRecipe>,
    retained_replay_recipe: Option<RetainedReplayRecipe>,
    topology_breadth: WorkloadTopologyBreadth,
    topology_construction_plan: Option<WorkloadCatalogTopologyConstructionPlan>,
}

impl WorkloadCatalogRecipe {
    fn new(kind: WorkloadCatalogRecipeKind) -> Self {
        Self {
            kind,
            declaration: kind.default_declaration().to_string(),
            transform_recipe: None,
            retained_replay_recipe: None,
            topology_breadth: WorkloadTopologyBreadth::Default,
            topology_construction_plan: None,
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

    fn with_topology_construction_plan(
        mut self,
        plan: WorkloadCatalogTopologyConstructionPlan,
    ) -> Self {
        self.topology_construction_plan = Some(plan);
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
        let topology_construction = self.compile_topology_construction()?;
        let workload_build = build_catalog_workload(
            self.kind,
            &self.declaration,
            self.transform_recipe
                .unwrap_or_else(|| self.kind.default_transform_recipe()),
            self.retained_replay_recipe
                .unwrap_or_else(|| self.kind.default_retained_replay_recipe()),
            self.topology_breadth,
            topology_construction,
        )?;
        let topology_neighborhood = workload_build.topology_neighborhood().cloned();
        let topology_construction = workload_build.topology_construction().cloned();
        let bound_geometry = workload_build.bound_geometry().clone();
        let projected = workload_build.projected().clone();
        let transform_receipts = workload_build.transform_receipts().clone();
        let replay_receipts = workload_build.replay_receipts().cloned();
        Ok(BuiltWorkloadCatalogRecipe::new(
            self.kind,
            declaration,
            support,
            workload_build.workload(),
            topology_neighborhood,
            topology_construction,
            bound_geometry,
            projected,
            transform_receipts,
            replay_receipts,
        ))
    }

    fn compile_topology_construction(
        &self,
    ) -> Result<Option<NmtTopologyConstructionReceipt>, WorkloadCatalogError> {
        self.topology_construction_plan
            .as_ref()
            .map(|plan| plan.compile(&self.declaration))
            .transpose()
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
        Ok(BuiltCleanFailCatalogRecipe::new(
            self.kind,
            declaration,
            support,
            topology_clean_fail,
        ))
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

        if let Some(denial) = self.explicit_topology_construction_denial() {
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

    fn explicit_topology_construction_denial(&self) -> Option<String> {
        self.topology_construction_plan
            .as_ref()
            .and_then(WorkloadCatalogTopologyConstructionPlan::support_denial)
    }

    fn explicit_topology_breadth_denial(&self) -> Option<String> {
        match self.topology_breadth {
            WorkloadTopologyBreadth::HighValenceVertex { valence }
                if self.kind == WorkloadCatalogRecipeKind::HighValenceVertex
                    && !(3..=HIGH_VALENCE_VERTEX_MAX_ADMITTED_VALENCE).contains(&valence) =>
            {
                Some(format!(
                    "high valence vertex workload recipe supports valence 3 through {HIGH_VALENCE_VERTEX_MAX_ADMITTED_VALENCE} today; valence {valence} needs an explicit widening phase"
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

fn reject_blank_declaration(declaration: &str) -> Result<(), WorkloadCatalogError> {
    if declaration.trim().is_empty() {
        Err(WorkloadCatalogError::MissingDeclaration)
    } else {
        Ok(())
    }
}
