use super::error::WorkloadCatalogError;
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
use crate::workload_composition::trace_scope;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use topology::facade::{NmtTopologyConstructionReceipt, TopologySeed};
use worth_spatial::facade::workload_binding::PlanarLoopBoundaryCatalogProfile;

pub const HIGH_VALENCE_VERTEX_MAX_ADMITTED_VALENCE: usize = 128;

pub struct WorkloadCatalog;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCatalogRecipe {
    kind: WorkloadCatalogRecipeKind,
    declaration: String,
    transform_recipe: Option<TransformRecipe>,
    retained_replay_recipe: Option<RetainedReplayRecipe>,
    topology_breadth: WorkloadTopologyBreadth,
    planar_loop_boundary_profile: PlanarLoopBoundaryCatalogProfile,
    topology_construction_plan: Option<WorkloadCatalogTopologyConstructionPlan>,
}

impl WorkloadCatalogRecipe {
    pub(super) fn new(kind: WorkloadCatalogRecipeKind) -> Self {
        Self {
            kind,
            declaration: kind.default_declaration().to_string(),
            transform_recipe: None,
            retained_replay_recipe: None,
            topology_breadth: WorkloadTopologyBreadth::Default,
            planar_loop_boundary_profile: PlanarLoopBoundaryCatalogProfile::Default,
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

    pub fn with_planar_loop_boundary_profile(
        mut self,
        profile: PlanarLoopBoundaryCatalogProfile,
    ) -> Self {
        self.planar_loop_boundary_profile = profile;
        self
    }

    pub fn with_retained_replay_artifacts(mut self) -> Self {
        self.retained_replay_recipe = Some(RetainedReplayRecipe::RetainedCancellationChain);
        self
    }

    pub(super) fn with_topology_construction_plan(
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
        let cache_key = self.cache_key();
        if let Some(cached) = cache_key.as_ref().and_then(|key| cached_catalog_build(key)) {
            return Ok(cached);
        }

        trace_scope("workload_catalog_recipe_build", || {
            let declaration = trace_scope("catalog_recipe_declaration_receipt", || {
                self.declaration_receipt()
            })?;
            let support = trace_scope("catalog_recipe_support_receipt", || {
                WorkloadCatalogSupportReceipt::new(&declaration, self.support_decision())
            })?;
            if support.posture() != WorkloadCatalogSupportPosture::Admitted {
                return Err(WorkloadCatalogError::UnsupportedRecipe {
                    recipe: self.kind,
                    reason: support.human_reason().to_string(),
                });
            }
            let topology_construction =
                trace_scope("catalog_recipe_topology_construction", || {
                    self.compile_topology_construction()
                })?;
            let workload_build = trace_scope("catalog_recipe_build_catalog_workload", || {
                build_catalog_workload(
                    self.kind,
                    &self.declaration,
                    self.transform_recipe
                        .unwrap_or_else(|| self.kind.default_transform_recipe()),
                    self.retained_replay_recipe
                        .unwrap_or_else(|| self.kind.default_retained_replay_recipe()),
                    self.topology_breadth,
                    self.planar_loop_boundary_profile,
                    topology_construction,
                )
            })?;
            let topology_neighborhood = workload_build.topology_neighborhood().cloned();
            let topology_construction = workload_build.topology_construction().cloned();
            let bound_geometry = workload_build.bound_geometry().clone();
            let surface_support = workload_build.surface_support().clone();
            let projected = workload_build.projected().clone();
            let transform_receipts = workload_build.transform_receipts().clone();
            let replay_receipts = workload_build.replay_receipts().cloned();
            let built = BuiltWorkloadCatalogRecipe::new(
                self.kind,
                declaration,
                support,
                workload_build.workload(),
                topology_neighborhood,
                topology_construction,
                bound_geometry,
                surface_support,
                projected,
                transform_receipts,
                replay_receipts,
            );
            if let Some(key) = cache_key {
                cache_catalog_build(key, built.clone());
            }
            Ok(built)
        })
    }

    fn compile_topology_construction(
        &self,
    ) -> Result<Option<NmtTopologyConstructionReceipt>, WorkloadCatalogError> {
        self.topology_construction_plan
            .as_ref()
            .map(|plan| plan.compile(&self.declaration))
            .transpose()
    }

    fn cache_key(&self) -> Option<String> {
        let topology_construction = match &self.topology_construction_plan {
            Some(plan) => plan.cache_key()?,
            None => "none".to_string(),
        };
        Some(format!(
            "{}::{}::{:?}::{:?}::{:?}::{:?}::{}",
            self.kind.query_key(),
            self.declaration,
            self.transform_recipe,
            self.retained_replay_recipe,
            self.topology_breadth,
            self.planar_loop_boundary_profile,
            topology_construction
        ))
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

    pub(crate) fn inspect_clean_fail_support(
        &self,
    ) -> Result<WorkloadCatalogSupportReceipt, WorkloadCatalogError> {
        let declaration = self.declaration_receipt()?;
        WorkloadCatalogSupportReceipt::new(&declaration, self.clean_fail_support_decision()?)
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
            WorkloadTopologyBreadth::SingleFaceLoopEdges { edge_count }
                if self.kind == WorkloadCatalogRecipeKind::SingleFaceLoop && edge_count < 3 =>
            {
                Some(format!(
                    "single face loop workload recipes require at least 3 boundary edges; requested {edge_count}"
                ))
            }
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

fn cached_catalog_build(key: &str) -> Option<BuiltWorkloadCatalogRecipe> {
    catalog_build_cache()
        .lock()
        .expect("catalog build cache should not be poisoned")
        .get(key)
        .cloned()
}

fn cache_catalog_build(key: String, built: BuiltWorkloadCatalogRecipe) {
    catalog_build_cache()
        .lock()
        .expect("catalog build cache should not be poisoned")
        .insert(key, built);
}

fn catalog_build_cache() -> &'static Mutex<BTreeMap<String, BuiltWorkloadCatalogRecipe>> {
    static CACHE: OnceLock<Mutex<BTreeMap<String, BuiltWorkloadCatalogRecipe>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(BTreeMap::new()))
}
