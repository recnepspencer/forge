use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifactHandle, WorthUiArtifactNodeInspection, WorthUiArtifactProvenanceMap,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInspection {
    provenance_map: WorthUiArtifactProvenanceMap,
    node_inspections: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactNodeInspection>,
}

impl WorthUiArtifactInspection {
    pub(crate) fn new(
        provenance_map: WorthUiArtifactProvenanceMap,
        node_inspections: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactNodeInspection>,
    ) -> Self {
        Self {
            provenance_map,
            node_inspections,
        }
    }

    pub(crate) fn provenance_map(&self) -> &WorthUiArtifactProvenanceMap {
        &self.provenance_map
    }

    pub(crate) fn node(
        &self,
        handle: &WorthUiArtifactHandle,
    ) -> Option<&WorthUiArtifactNodeInspection> {
        self.node_inspections.get(handle)
    }

    pub(crate) fn handles(&self) -> &[WorthUiArtifactHandle] {
        self.provenance_map.handles()
    }
}
