use std::collections::BTreeMap;
use worth_ui_dsl::WorthUiSourceModuleId;

use crate::source::WorthUiArtifactHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactImpact {
    impacted_handles: Vec<WorthUiArtifactHandle>,
    full_artifact_scan_required: bool,
    full_artifact_handle_count: usize,
    lookup_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactImpactMetadata {
    module_impacts: BTreeMap<WorthUiSourceModuleId, Vec<WorthUiArtifactHandle>>,
    subtree_impacts: BTreeMap<WorthUiArtifactHandle, Vec<WorthUiArtifactHandle>>,
    full_artifact_handle_count: usize,
}

impl WorthUiArtifactImpact {
    pub(crate) fn new(
        mut impacted_handles: Vec<WorthUiArtifactHandle>,
        full_artifact_scan_required: bool,
        full_artifact_handle_count: usize,
        lookup_count: usize,
    ) -> Self {
        impacted_handles.sort();
        impacted_handles.dedup();
        Self {
            impacted_handles,
            full_artifact_scan_required,
            full_artifact_handle_count,
            lookup_count,
        }
    }

    pub(crate) fn impacted_handles(&self) -> &[WorthUiArtifactHandle] {
        &self.impacted_handles
    }

    #[cfg(test)]
    pub(crate) fn requires_less_than_full_artifact_scan(&self) -> bool {
        !self.full_artifact_scan_required
            && self.impacted_handles.len() < self.full_artifact_handle_count
    }

    #[cfg(test)]
    pub(crate) fn full_artifact_handle_count(&self) -> usize {
        self.full_artifact_handle_count
    }

    pub(crate) fn lookup_count(&self) -> usize {
        self.lookup_count
    }
}

impl WorthUiArtifactImpactMetadata {
    pub(crate) fn new(
        module_impacts: BTreeMap<WorthUiSourceModuleId, Vec<WorthUiArtifactHandle>>,
        subtree_impacts: BTreeMap<WorthUiArtifactHandle, Vec<WorthUiArtifactHandle>>,
        full_artifact_handle_count: usize,
    ) -> Self {
        Self {
            module_impacts,
            subtree_impacts,
            full_artifact_handle_count,
        }
    }

    pub(crate) fn impact_for_module(
        &self,
        module_id: &WorthUiSourceModuleId,
    ) -> WorthUiArtifactImpact {
        WorthUiArtifactImpact::new(
            self.module_impacts
                .get(module_id)
                .cloned()
                .unwrap_or_default(),
            false,
            self.full_artifact_handle_count,
            1,
        )
    }

    pub(crate) fn impact_for_subtree(
        &self,
        handle: &WorthUiArtifactHandle,
    ) -> WorthUiArtifactImpact {
        WorthUiArtifactImpact::new(
            self.subtree_impacts
                .get(handle)
                .cloned()
                .unwrap_or_default(),
            false,
            self.full_artifact_handle_count,
            1,
        )
    }

    pub(crate) fn module_impacts(
        &self,
    ) -> &BTreeMap<WorthUiSourceModuleId, Vec<WorthUiArtifactHandle>> {
        &self.module_impacts
    }

    pub(crate) fn subtree_impacts(
        &self,
    ) -> &BTreeMap<WorthUiArtifactHandle, Vec<WorthUiArtifactHandle>> {
        &self.subtree_impacts
    }

    pub(crate) fn full_artifact_handle_count(&self) -> usize {
        self.full_artifact_handle_count
    }
}
