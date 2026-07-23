use crate::runtime::{
    WorthUiArtifactToPlanProvenance, WorthUiPlanChildRange, WorthUiPlanNodeFamily,
    WorthUiPlanRegionStructure, WorthUiQueryInspectionLinks, WorthUiRenderResourceRef,
    WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanNodeInspection {
    plan_index: u32,
    runtime_handle: WorthUiRuntimeHandle,
    family: WorthUiPlanNodeFamily,
    child_range: Option<WorthUiPlanChildRange>,
    region_structure: Option<WorthUiPlanRegionStructure>,
    render_resource_ref: Option<WorthUiRenderResourceRef>,
    provenance: WorthUiArtifactToPlanProvenance,
}

#[cfg(any(test, feature = "certification-support"))]
pub(crate) struct WorthUiPlanNodeInspectionInput {
    pub plan_index: u32,
    pub runtime_handle: WorthUiRuntimeHandle,
    pub family: WorthUiPlanNodeFamily,
    pub child_range: Option<WorthUiPlanChildRange>,
    pub region_structure: Option<WorthUiPlanRegionStructure>,
    pub render_resource_ref: Option<WorthUiRenderResourceRef>,
    pub provenance: WorthUiArtifactToPlanProvenance,
}

impl WorthUiPlanNodeInspection {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn new(input: WorthUiPlanNodeInspectionInput) -> Self {
        let WorthUiPlanNodeInspectionInput {
            plan_index,
            runtime_handle,
            family,
            child_range,
            region_structure,
            render_resource_ref,
            provenance,
        } = input;
        Self {
            plan_index,
            runtime_handle,
            family,
            child_range,
            region_structure,
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
