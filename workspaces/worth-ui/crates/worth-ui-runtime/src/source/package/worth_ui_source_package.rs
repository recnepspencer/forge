use std::collections::BTreeMap;
use std::path::Path;

use crate::source::{
    WorthUiCanonicalModuleOrder, WorthUiSourceImportGraph, WorthUiSourceModuleId,
    WorthUiSourceModuleRecord, WorthUiSourcePackageDigest,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiSourcePackage {
    workspace_root: std::path::PathBuf,
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiSourceModuleRecord>,
    canonical_module_order: WorthUiCanonicalModuleOrder,
    import_graph: WorthUiSourceImportGraph,
    digest: WorthUiSourcePackageDigest,
}

impl WorthUiSourcePackage {
    pub(crate) fn new(
        workspace_root: std::path::PathBuf,
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiSourceModuleRecord>,
        canonical_module_order: WorthUiCanonicalModuleOrder,
        import_graph: WorthUiSourceImportGraph,
        digest: WorthUiSourcePackageDigest,
    ) -> Self {
        Self {
            workspace_root,
            modules,
            canonical_module_order,
            import_graph,
            digest,
        }
    }

    pub(crate) fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub(crate) fn digest(&self) -> WorthUiSourcePackageDigest {
        self.digest
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        self.canonical_module_order.module_ids()
    }

    pub(crate) fn module_record(
        &self,
        module_id: &WorthUiSourceModuleId,
    ) -> Option<&WorthUiSourceModuleRecord> {
        self.modules.get(module_id)
    }

    pub(crate) fn import_graph(&self) -> &WorthUiSourceImportGraph {
        &self.import_graph
    }
}
