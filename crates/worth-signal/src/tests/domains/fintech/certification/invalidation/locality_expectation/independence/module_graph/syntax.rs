use std::collections::BTreeSet;
use std::path::Path;

use syn::visit::{self, Visit};

use super::SAFE_MACROS;

pub(super) fn references(
    manifest: &Path,
    path: &Path,
    source: &str,
) -> Result<Vec<Vec<String>>, String> {
    let file = syn::parse_file(source).map_err(|error| error.to_string())?;
    let current = module_segments(manifest, path)?;
    let mut visitor = ReferenceVisitor::default();
    visitor.visit_file(&file);
    if visitor.has_glob {
        return Err(format!("{} contains an opaque glob import", path.display()));
    }
    Ok(visitor
        .paths
        .into_iter()
        .filter_map(|segments| absolutize(&current, &segments))
        .collect())
}

pub(super) fn validate_macros(path: &Path, file: &syn::File) -> Result<(), String> {
    let mut visitor = MacroVisitor::default();
    visitor.visit_file(file);
    for name in visitor.names {
        if !SAFE_MACROS.contains(&name.as_str()) {
            return Err(format!("{} contains opaque macro {name}!", path.display()));
        }
    }
    Ok(())
}

fn module_segments(manifest: &Path, path: &Path) -> Result<Vec<String>, String> {
    let relative = path
        .strip_prefix(manifest.join("src"))
        .map_err(|_| format!("{} is outside crate src", path.display()))?;
    let mut segments = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    let file = segments.pop().expect("source path must have a file");
    if file != "mod.rs" {
        segments.push(file.trim_end_matches(".rs").to_owned());
    }
    Ok(segments)
}

fn absolutize(current: &[String], reference: &[String]) -> Option<Vec<String>> {
    match reference.first().map(String::as_str) {
        Some("crate") => Some(reference[1..].to_vec()),
        Some("self") => {
            let mut absolute = current.to_vec();
            absolute.extend_from_slice(&reference[1..]);
            Some(absolute)
        }
        Some("super") => {
            let mut absolute = current.to_vec();
            let mut index = 0;
            while reference
                .get(index)
                .is_some_and(|segment| segment == "super")
            {
                absolute.pop()?;
                index += 1;
            }
            absolute.extend_from_slice(&reference[index..]);
            Some(absolute)
        }
        _ => None,
    }
}

#[derive(Default)]
struct ReferenceVisitor {
    paths: BTreeSet<Vec<String>>,
    has_glob: bool,
}

impl<'ast> Visit<'ast> for ReferenceVisitor {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if !cfg_test_only(&item.attrs) {
            visit::visit_item_mod(self, item);
        }
    }

    fn visit_visibility(&mut self, _visibility: &'ast syn::Visibility) {}

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_paths(&item.tree, &mut Vec::new(), self);
        visit::visit_item_use(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        self.paths.insert(
            path.segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        );
        visit::visit_path(self, path);
    }
}

fn collect_use_paths(tree: &syn::UseTree, prefix: &mut Vec<String>, found: &mut ReferenceVisitor) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_paths(&path.tree, prefix, found);
            prefix.pop();
        }
        syn::UseTree::Name(name) => {
            prefix.push(name.ident.to_string());
            found.paths.insert(prefix.clone());
            prefix.pop();
        }
        syn::UseTree::Rename(rename) => {
            prefix.push(rename.ident.to_string());
            found.paths.insert(prefix.clone());
            prefix.pop();
        }
        syn::UseTree::Glob(_) => {
            found.has_glob |= prefix
                .first()
                .is_some_and(|segment| matches!(segment.as_str(), "crate" | "self" | "super"));
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_paths(item, prefix, found);
            }
        }
    }
}

#[derive(Default)]
struct MacroVisitor {
    names: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for MacroVisitor {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if !cfg_test_only(&item.attrs) {
            visit::visit_item_mod(self, item);
        }
    }

    fn visit_macro(&mut self, item: &'ast syn::Macro) {
        if let Some(segment) = item.path.segments.last() {
            self.names.insert(segment.ident.to_string());
        }
        visit::visit_macro(self, item);
    }
}

fn cfg_test_only(attributes: &[syn::Attribute]) -> bool {
    use quote::ToTokens;

    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg")
            && attribute.meta.to_token_stream().to_string() == "cfg (test)"
    })
}
