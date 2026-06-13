use crate::source::{
    WorthUiArtifact, WorthUiArtifactNode, WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts,
};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiPlanNodeTopologyInput {
    structure_declared: bool,
    root_region_count: usize,
    region_count: usize,
    mount_count: usize,
    max_region_depth: usize,
}

impl WorthUiPlanNodeTopologyInput {
    pub(crate) fn from_structure(structure: &WorthUiMosaicStructureFacts) -> Self {
        let mut topology = Self {
            structure_declared: true,
            root_region_count: structure.root_regions().len(),
            region_count: 0,
            mount_count: 0,
            max_region_depth: 0,
        };
        for region in structure.root_regions() {
            topology.record_region(region, 1);
        }
        topology
    }

    pub(crate) fn empty() -> Self {
        Self::default()
    }

    pub(crate) fn has_region_structure(self) -> bool {
        self.structure_declared
    }

    fn record_region(&mut self, region: &WorthUiMosaicRegionFacts, depth: usize) {
        self.region_count += 1;
        self.mount_count += region.mounts().len();
        self.max_region_depth = self.max_region_depth.max(depth);
        for child in region.child_regions() {
            self.record_region(child, depth + 1);
        }
    }

    pub fn structure_declared(self) -> bool {
        self.structure_declared
    }

    pub fn root_region_count(self) -> usize {
        self.root_region_count
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

pub(crate) struct WorthUiPlanNodeTopologyInputIndex {
    by_identity_basis: BTreeMap<String, WorthUiPlanNodeTopologyInput>,
}

impl WorthUiPlanNodeTopologyInputIndex {
    pub(crate) fn from_artifact(artifact: &WorthUiArtifact) -> Self {
        let mut by_identity_basis = BTreeMap::new();
        for module_id in artifact.module_ids() {
            let Some(module) = artifact.module(module_id) else {
                continue;
            };
            for node in module.nodes() {
                record_artifact_node_topology_input(node, &mut by_identity_basis);
            }
        }
        Self { by_identity_basis }
    }

    pub(crate) fn input_for_identity(
        &self,
        identity_basis: &str,
    ) -> Option<WorthUiPlanNodeTopologyInput> {
        self.by_identity_basis.get(identity_basis).copied()
    }
}

fn record_artifact_node_topology_input(
    node: &WorthUiArtifactNode,
    by_identity_basis: &mut BTreeMap<String, WorthUiPlanNodeTopologyInput>,
) {
    match node {
        WorthUiArtifactNode::Component(component) => {
            by_identity_basis.insert(
                component.identity_seed().basis().to_owned(),
                WorthUiPlanNodeTopologyInput::from_structure(component.structure()),
            );
        }
        WorthUiArtifactNode::Surface(surface) => {
            by_identity_basis.insert(
                surface.identity_seed().basis().to_owned(),
                WorthUiPlanNodeTopologyInput::from_structure(surface.structure()),
            );
        }
        WorthUiArtifactNode::Binding(binding) => {
            let topology_input = WorthUiPlanNodeTopologyInput::from_structure(binding.structure());
            by_identity_basis.insert(binding.identity_seed().basis().to_owned(), topology_input);
            by_identity_basis.insert(
                binding
                    .view_binding_reference()
                    .view_binding()
                    .id()
                    .as_str()
                    .to_owned(),
                topology_input,
            );
        }
        WorthUiArtifactNode::Import(_) | WorthUiArtifactNode::Token(_) => {}
    }
}
