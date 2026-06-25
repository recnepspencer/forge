use crate::capability::SurfaceId;
use crate::runtime::query_graph::WorthUiPrimitiveConstructionGraphPlan;
use crate::runtime::{
    WorthUiGraphFactRegistry, WorthUiProjectionDependencyDeclaration,
    WorthUiProjectionDependencySet, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeGraphAuthority,
    WorthUiValidatedProjectionDependencyContract,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveConstructionFamily {
    BasePrimitive,
    FlowLayout,
    Content,
    AppearanceState,
    Interaction,
    EventGeometry,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveConstructionFamilySelection {
    families: Vec<WorthUiPrimitiveConstructionFamily>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveConstructionRequest {
    surface_id: SurfaceId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveConstructionPlan {
    surface_id: SurfaceId,
    family_selection: WorthUiPrimitiveConstructionFamilySelection,
    dependency_contract: WorthUiValidatedProjectionDependencyContract,
    query_graph_plan: WorthUiPrimitiveConstructionGraphPlan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveConstructionPlanningDenial {
    EmptyDependencyContract,
}

impl WorthUiRuntimeGraphAuthority {
    pub fn plan_primitive_construction(
        &self,
        request: WorthUiPrimitiveConstructionRequest,
    ) -> Result<WorthUiPrimitiveConstructionPlan, WorthUiPrimitiveConstructionPlanningDenial> {
        let dependency_contract = primitive_construction_dependency_contract(request.surface_id())?;
        let dependency_facts = dependency_contract.dependencies().facts().cloned();
        let query_graph_plan = self.plan_primitive_construction_graph_operation(
            request.surface_id().as_str(),
            dependency_facts,
        );
        Ok(WorthUiPrimitiveConstructionPlan::new(
            request,
            WorthUiPrimitiveConstructionFamilySelection::for_current_primitive_surface(),
            dependency_contract,
            query_graph_plan,
        ))
    }
}

impl WorthUiPrimitiveConstructionRequest {
    pub fn for_surface(surface_id: SurfaceId) -> Self {
        Self { surface_id }
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }
}

impl WorthUiPrimitiveConstructionPlan {
    fn new(
        request: WorthUiPrimitiveConstructionRequest,
        family_selection: WorthUiPrimitiveConstructionFamilySelection,
        dependency_contract: WorthUiValidatedProjectionDependencyContract,
        query_graph_plan: WorthUiPrimitiveConstructionGraphPlan,
    ) -> Self {
        Self {
            surface_id: request.surface_id,
            family_selection,
            dependency_contract,
            query_graph_plan,
        }
    }

    pub fn surface_id(&self) -> &SurfaceId {
        &self.surface_id
    }

    pub fn family_selection(&self) -> &WorthUiPrimitiveConstructionFamilySelection {
        &self.family_selection
    }

    pub fn dependency_contract(&self) -> &WorthUiValidatedProjectionDependencyContract {
        &self.dependency_contract
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        self.query_graph_plan.execution_receipt()
    }

    pub fn query_graph_plan(&self) -> &WorthUiPrimitiveConstructionGraphPlan {
        &self.query_graph_plan
    }
}

impl WorthUiPrimitiveConstructionFamilySelection {
    fn for_current_primitive_surface() -> Self {
        Self {
            families: vec![
                WorthUiPrimitiveConstructionFamily::BasePrimitive,
                WorthUiPrimitiveConstructionFamily::FlowLayout,
                WorthUiPrimitiveConstructionFamily::Content,
                WorthUiPrimitiveConstructionFamily::AppearanceState,
                WorthUiPrimitiveConstructionFamily::Interaction,
                WorthUiPrimitiveConstructionFamily::EventGeometry,
            ],
        }
    }

    pub fn families(&self) -> &[WorthUiPrimitiveConstructionFamily] {
        &self.families
    }

    pub fn requires(&self, family: WorthUiPrimitiveConstructionFamily) -> bool {
        self.families.contains(&family)
    }
}

fn primitive_construction_dependency_contract(
    surface_id: &SurfaceId,
) -> Result<WorthUiValidatedProjectionDependencyContract, WorthUiPrimitiveConstructionPlanningDenial>
{
    let registry = WorthUiGraphFactRegistry::for_primitive_surface(surface_id.as_str());
    let dependencies = WorthUiProjectionDependencySet::empty().depends_on(
        crate::runtime::WorthUiRuntimeFactId::authored_surface_props(surface_id.as_str()),
    );
    let dependencies = registry
        .published_facts()
        .facts()
        .cloned()
        .fold(dependencies, |set, fact| set.depends_on(fact));
    let declaration = WorthUiProjectionDependencyDeclaration::from_set(dependencies);
    let identity = WorthUiProjectionIdentity::runtime(format!(
        "query-graph:primitive-construction:{}",
        surface_id.as_str()
    ));
    WorthUiValidatedProjectionDependencyContract::admit(
        identity,
        WorthUiProjectionFamily::PrimitiveProof,
        declaration,
    )
    .map_err(|_| WorthUiPrimitiveConstructionPlanningDenial::EmptyDependencyContract)
}
