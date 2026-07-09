use std::collections::BTreeMap;

#[cfg(test)]
use crate::source::{WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceComparator};
#[cfg(test)]
use crate::source::{WorthUiArtifactHandle, WorthUiArtifactNode};
use crate::source::{WorthUiArtifactModule, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifact {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiArtifactModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiArtifact {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiArtifactModule>,
        canonical_module_order: Vec<WorthUiSourceModuleId>,
    ) -> Self {
        Self {
            modules,
            canonical_module_order,
        }
    }

    pub(crate) fn module(
        &self,
        module_id: &WorthUiSourceModuleId,
    ) -> Option<&WorthUiArtifactModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    #[cfg(test)]
    pub(crate) fn node(&self, handle: &WorthUiArtifactHandle) -> Option<&WorthUiArtifactNode> {
        self.module(handle.module_id())
            .and_then(|module| module.node(handle.node_index()))
    }

    pub(crate) fn authored_provenance_digests(&self) -> Vec<u64> {
        let mut digests = Vec::new();
        for module_id in &self.canonical_module_order {
            let Some(module) = self.module(module_id) else {
                continue;
            };
            for node in module.nodes() {
                digests.push(node.authored_provenance_digest());
            }
        }
        digests.sort_unstable();
        digests.dedup();
        digests
    }

    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiArtifactEquivalenceComparator::compare(
            self,
            other,
            WorthUiArtifactEquivalenceBasis::semantic(),
        )
        .is_equivalent()
    }
}
