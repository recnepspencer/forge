use crate::runtime::{
    WorthUiPlanNodeInputFamily, WorthUiPlanNodeTopologyInput, WorthUiRuntimeHandle,
    WorthUiRuntimeHandleLocator,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiPlanNodeFamily {
    input_family: WorthUiPlanNodeInputFamily,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPlanChildRange {
    owner_plan_index: u32,
    start: u32,
    len: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRenderResourceRef {
    locator: WorthUiRuntimeHandleLocator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPlanRegionStructure {
    structure_declared: bool,
    root_region_count: usize,
    region_count: usize,
    mount_count: usize,
    max_region_depth: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanNode {
    runtime_handle: WorthUiRuntimeHandle,
    family: WorthUiPlanNodeFamily,
    child_range: Option<WorthUiPlanChildRange>,
    region_structure: Option<WorthUiPlanRegionStructure>,
    render_resource_ref: Option<WorthUiRenderResourceRef>,
}

impl WorthUiPlanNodeFamily {
    pub(crate) fn from_input_family(input_family: WorthUiPlanNodeInputFamily) -> Self {
        Self { input_family }
    }

    pub fn input_family(self) -> WorthUiPlanNodeInputFamily {
        self.input_family
    }
}

impl WorthUiPlanChildRange {
    pub(crate) fn from_root_region_count(owner_plan_index: u32, root_region_count: u32) -> Self {
        Self {
            owner_plan_index,
            start: owner_plan_index,
            len: root_region_count,
        }
    }

    pub(crate) fn from_compact_row(owner_plan_index: u32, len: u32) -> Self {
        Self {
            owner_plan_index,
            start: 0,
            len,
        }
    }

    pub fn owner_plan_index(self) -> u32 {
        self.owner_plan_index
    }

    pub fn start(self) -> u32 {
        self.start
    }

    pub fn len(self) -> u32 {
        self.len
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }
}

impl WorthUiRenderResourceRef {
    pub(crate) fn new(locator: WorthUiRuntimeHandleLocator) -> Self {
        Self { locator }
    }

    pub fn owner_plan_index(self) -> u32 {
        self.locator.plan_index()
    }

    pub fn locator(self) -> WorthUiRuntimeHandleLocator {
        self.locator
    }
}

impl WorthUiPlanRegionStructure {
    pub(crate) fn from_topology_input(input: WorthUiPlanNodeTopologyInput) -> Option<Self> {
        if !input.has_region_structure() {
            return None;
        }
        Some(Self {
            structure_declared: input.structure_declared(),
            root_region_count: input.root_region_count(),
            region_count: input.region_count(),
            mount_count: input.mount_count(),
            max_region_depth: input.max_region_depth(),
        })
    }

    pub fn root_region_count(self) -> usize {
        self.root_region_count
    }

    pub fn structure_declared(self) -> bool {
        self.structure_declared
    }

    pub fn region_count(self) -> usize {
        self.region_count
    }

    pub fn mount_count(self) -> usize {
        self.mount_count
    }

    pub fn max_region_depth(self) -> usize {
        self.max_region_depth
    }
}

impl WorthUiPlanNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        family: WorthUiPlanNodeFamily,
        child_range: Option<WorthUiPlanChildRange>,
        region_structure: Option<WorthUiPlanRegionStructure>,
        render_resource_ref: Option<WorthUiRenderResourceRef>,
    ) -> Self {
        Self {
            runtime_handle,
            family,
            child_range,
            region_structure,
            render_resource_ref,
        }
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
}
