use crate::runtime::{
    WorthUiArtifactToPlanProvenance, WorthUiEguiPlanBoundary, WorthUiPlanChildRange,
    WorthUiPlanNodeFamily, WorthUiPlanRegionStructure, WorthUiQueryInspectionLinks,
    WorthUiRenderResourceRef, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanNodeInspection {
    plan_index: u32,
    runtime_handle: WorthUiRuntimeHandle,
    family: WorthUiPlanNodeFamily,
    child_range: Option<WorthUiPlanChildRange>,
    region_structure: Option<WorthUiPlanRegionStructure>,
    egui_boundary: Option<WorthUiEguiPlanBoundary>,
    render_resource_ref: Option<WorthUiRenderResourceRef>,
    provenance: WorthUiArtifactToPlanProvenance,
}

impl WorthUiPlanNodeInspection {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        plan_index: u32,
        runtime_handle: WorthUiRuntimeHandle,
        family: WorthUiPlanNodeFamily,
        child_range: Option<WorthUiPlanChildRange>,
        region_structure: Option<WorthUiPlanRegionStructure>,
        egui_boundary: Option<WorthUiEguiPlanBoundary>,
        render_resource_ref: Option<WorthUiRenderResourceRef>,
        provenance: WorthUiArtifactToPlanProvenance,
    ) -> Self {
        Self {
            plan_index,
            runtime_handle,
            family,
            child_range,
            region_structure,
            egui_boundary,
            render_resource_ref,
            provenance,
        }
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn runtime_handle(&self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn family(&self) -> WorthUiPlanNodeFamily {
        self.family
    }

    pub fn child_range(&self) -> Option<WorthUiPlanChildRange> {
        self.child_range
    }

    pub fn region_structure(&self) -> Option<WorthUiPlanRegionStructure> {
        self.region_structure
    }

    pub fn egui_boundary(&self) -> Option<&WorthUiEguiPlanBoundary> {
        self.egui_boundary.as_ref()
    }

    pub fn render_resource_ref(&self) -> Option<WorthUiRenderResourceRef> {
        self.render_resource_ref
    }

    pub fn artifact_provenance(&self) -> &WorthUiArtifactToPlanProvenance {
        &self.provenance
    }

    pub fn capability_provenance(&self) -> Option<&str> {
        self.provenance.capability_reference()
    }

    pub fn query_inspection_links(&self) -> Option<&WorthUiQueryInspectionLinks> {
        self.provenance.query_links()
    }
}
