use crate::source::{
    WorthUiCanonicalModuleOrder, WorthUiSourceImportGraph, WorthUiSourceModuleRecord,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiSourcePackageDigest(u64);

impl WorthUiSourcePackageDigest {
    pub(crate) fn from_package_parts(
        canonical_order: &WorthUiCanonicalModuleOrder,
        import_graph: &WorthUiSourceImportGraph,
        modules: &[WorthUiSourceModuleRecord],
    ) -> Self {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        for module_id in canonical_order.module_ids() {
            fold_text(&mut digest, module_id.as_str());
            if let Some(imports) = import_graph.imports_for(module_id) {
                for import in imports {
                    fold_text(&mut digest, import.target_module_id().as_str());
                }
            }
        }
        for module in modules {
            fold_text(
                &mut digest,
                module.relative_path().to_string_lossy().as_ref(),
            );
            fold_text(&mut digest, module.source_text());
        }
        Self(digest)
    }

    pub(crate) fn raw(self) -> u64 {
        self.0
    }
}

fn fold_text(digest: &mut u64, text: &str) {
    for byte in text.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}
