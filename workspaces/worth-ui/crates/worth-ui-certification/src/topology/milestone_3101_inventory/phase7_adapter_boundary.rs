use std::path::Path;

use syn::visit::Visit;

use crate::topology::WorkspaceSourceInventory;

const HEADLESS_ADAPTER_ROOT: &str = "crates/worth-ui-runtime/src/host/adapter";
const ALLOWED_USE_ROOTS: &[&str] = &["std", "super", "worth_ui_host_contract"];
const FORBIDDEN_SEGMENTS: &[&str] = &[
    "source_ingress",
    "worth_ui_dsl",
    "WorthUiAuthoredSourceInput",
    "WorthUiDslCompiler",
    "WorthUiPreparedSemanticHandoffMaterial",
    "WorthUiSealedSemanticPackage",
    "WorthUiSemanticHandoffEvidence",
    "WorthUiSemanticPackageIdentity",
    "WorthUiWatchedCandidateSubmission",
];

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for source in inventory.rust_files_under(Path::new(HEADLESS_ADAPTER_ROOT)) {
        let path = source.relative_path().to_string_lossy().replace('\\', "/");
        let file_name = source
            .relative_path()
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        if !file_name.starts_with("headless_") || file_name.ends_with("_tests.rs") {
            continue;
        }
        reject_source_authority(&path, source.text())?;
    }
    Ok(())
}

fn reject_source_authority(path: &str, source: &str) -> Result<(), String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("`{path}` should parse: {error}"))?;
    let mut visitor = AdapterDependencyVisitor::default();
    visitor.visit_file(&syntax);
    if let Some(dependency) = visitor.forbidden_dependency {
        return Err(format!(
            "headless adapter `{path}` reaches source/DSL authority through `{dependency}`"
        ));
    }
    Ok(())
}

#[derive(Default)]
struct AdapterDependencyVisitor {
    forbidden_dependency: Option<String>,
}

impl<'syntax> Visit<'syntax> for AdapterDependencyVisitor {
    fn visit_item_use(&mut self, item: &'syntax syn::ItemUse) {
        if self.forbidden_dependency.is_none() {
            self.forbidden_dependency = use_roots(&item.tree)
                .into_iter()
                .find(|root| !ALLOWED_USE_ROOTS.contains(&root.as_str()));
        }
        syn::visit::visit_item_use(self, item);
    }

    fn visit_path(&mut self, path: &'syntax syn::Path) {
        if self.forbidden_dependency.is_none() {
            self.forbidden_dependency = path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .find(|segment| FORBIDDEN_SEGMENTS.contains(&segment.as_str()));
        }
        syn::visit::visit_path(self, path);
    }
}

fn use_roots(tree: &syn::UseTree) -> Vec<String> {
    match tree {
        syn::UseTree::Path(path) => vec![path.ident.to_string()],
        syn::UseTree::Name(name) => vec![name.ident.to_string()],
        syn::UseTree::Rename(rename) => vec![rename.ident.to_string()],
        syn::UseTree::Group(group) => group.items.iter().flat_map(use_roots).collect(),
        syn::UseTree::Glob(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::reject_source_authority;

    #[test]
    fn headless_adapter_cannot_disguise_runtime_source_authority_as_mechanics() {
        let source = r#"
            use crate::runtime::source_ingress as mechanics;

            fn reopen(candidate: mechanics::WorthUiWatchedCandidateSubmission) {
                drop(candidate);
            }
        "#;
        let error = reject_source_authority(
            "crates/worth-ui-runtime/src/host/adapter/headless_reopen.rs",
            source,
        )
        .expect_err("headless adapter source authority must fail");
        assert!(error.contains("source/DSL authority"));
    }
}
