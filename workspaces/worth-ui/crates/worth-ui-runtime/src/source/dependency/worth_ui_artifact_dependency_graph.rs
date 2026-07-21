use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifactDependencyEdge, WorthUiArtifactHandle, WorthUiArtifactSubtreeDigest,
    WorthUiRuntimeDependencyHook, WorthUiSourceModuleId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactDependencyGraph {
    edges: Vec<WorthUiArtifactDependencyEdge>,
    module_dependencies: BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceModuleId>>,
    subtree_digests: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSubtreeDigest>,
    runtime_hooks: BTreeMap<WorthUiArtifactHandle, Vec<WorthUiRuntimeDependencyHook>>,
    query_hook_handles: BTreeMap<String, Vec<WorthUiArtifactHandle>>,
}

impl WorthUiArtifactDependencyGraph {
    pub(crate) fn new(
        mut edges: Vec<WorthUiArtifactDependencyEdge>,
        mut module_dependencies: BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceModuleId>>,
        subtree_digests: BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSubtreeDigest>,
        mut runtime_hooks: BTreeMap<WorthUiArtifactHandle, Vec<WorthUiRuntimeDependencyHook>>,
    ) -> Self {
        edges.sort();
        edges.dedup();
        canonicalize_map_values(&mut module_dependencies);
        canonicalize_map_values(&mut runtime_hooks);
        let mut query_hook_handles = BTreeMap::<String, Vec<WorthUiArtifactHandle>>::new();
        for (handle, hooks) in &runtime_hooks {
            for hook in hooks {
                query_hook_handles
                    .entry(hook.view_binding_id().as_str().to_owned())
                    .or_default()
                    .push(handle.clone());
            }
        }
        canonicalize_map_values(&mut query_hook_handles);
        Self {
            edges,
            module_dependencies,
            subtree_digests,
            runtime_hooks,
            query_hook_handles,
        }
    }

    pub(crate) fn edges(&self) -> &[WorthUiArtifactDependencyEdge] {
        &self.edges
    }

    pub(crate) fn module_dependencies(
        &self,
    ) -> &BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceModuleId>> {
        &self.module_dependencies
    }

    pub(crate) fn subtree_digest(
        &self,
        handle: &WorthUiArtifactHandle,
    ) -> Option<WorthUiArtifactSubtreeDigest> {
        self.subtree_digests.get(handle).copied()
    }

    pub(crate) fn subtree_digests(
        &self,
    ) -> &BTreeMap<WorthUiArtifactHandle, WorthUiArtifactSubtreeDigest> {
        &self.subtree_digests
    }

    pub(crate) fn runtime_hooks_for(
        &self,
        handle: &WorthUiArtifactHandle,
    ) -> &[WorthUiRuntimeDependencyHook] {
        self.runtime_hooks.get(handle).map_or(&[], Vec::as_slice)
    }

    pub(crate) fn runtime_hooks(
        &self,
    ) -> &BTreeMap<WorthUiArtifactHandle, Vec<WorthUiRuntimeDependencyHook>> {
        &self.runtime_hooks
    }

    pub(crate) fn runtime_hooks_for_query_binding<'graph>(
        &'graph self,
        view_binding_id: &'graph str,
    ) -> impl Iterator<Item = &'graph WorthUiRuntimeDependencyHook> + 'graph {
        self.query_hook_handles
            .get(view_binding_id)
            .into_iter()
            .flatten()
            .flat_map(|handle| self.runtime_hooks_for(handle))
            .filter(move |hook| hook.view_binding_id().as_str() == view_binding_id)
    }
}

fn canonicalize_map_values<K: Ord, V: Ord>(map: &mut BTreeMap<K, Vec<V>>) {
    for values in map.values_mut() {
        values.sort();
        values.dedup();
    }
}
