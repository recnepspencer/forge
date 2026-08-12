use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use syn::{Item, UseTree, Visibility};

use super::super::repository_root;
use super::FACADE;

pub(super) fn current_facade() -> Result<BTreeSet<(String, String, String)>, String> {
    let path = repository_root().join(FACADE);
    let mut exports = BTreeSet::new();
    collect_public_api(&path, &[], &mut exports)?;
    collect_exported_associated_items(&path, &mut exports)?;
    Ok(exports)
}

fn collect_public_api(
    path: &Path,
    namespace: &[String],
    exports: &mut BTreeSet<(String, String, String)>,
) -> Result<(), String> {
    let source = std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let file = syn::parse_file(&source).map_err(|error| format!("invalid facade: {error}"))?;
    for item in file.items {
        match item {
            Item::Use(item_use) if public(&item_use.vis) => {
                UseExportCollector::new(namespace, configuration(&item_use.attrs), exports)
                    .collect(&item_use.tree)?;
            }
            Item::Mod(item_mod) if public(&item_mod.vis) => {
                let mut child_namespace = namespace.to_vec();
                child_namespace.push(item_mod.ident.to_string());
                let module = child_namespace.join("::");
                exports.insert((
                    configuration(&item_mod.attrs).to_owned(),
                    module.clone(),
                    module,
                ));
                let child = module_path(path, &item_mod.ident.to_string())?;
                collect_public_api(&child, &child_namespace, exports)?;
            }
            Item::Const(item) if public(&item.vis) => {
                insert_named(namespace, &item.ident.to_string(), exports)
            }
            Item::Enum(item) if public(&item.vis) => {
                insert_named(namespace, &item.ident.to_string(), exports)
            }
            Item::Fn(item) if public(&item.vis) => {
                insert_named(namespace, &item.sig.ident.to_string(), exports)
            }
            Item::Static(item) if public(&item.vis) => {
                insert_named(namespace, &item.ident.to_string(), exports)
            }
            Item::Struct(item) if public(&item.vis) => {
                insert_named(namespace, &item.ident.to_string(), exports)
            }
            Item::Trait(item) if public(&item.vis) => {
                insert_named(namespace, &item.ident.to_string(), exports);
                collect_trait_items(&item, namespace, None, exports);
            }
            Item::Type(item) if public(&item.vis) => {
                insert_named(namespace, &item.ident.to_string(), exports)
            }
            Item::Impl(item) if item.trait_.is_none() => {
                collect_impl_items(&item, namespace, None, exports)
            }
            _ => {}
        }
    }
    Ok(())
}

fn public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

fn configuration(attrs: &[syn::Attribute]) -> &'static str {
    let certification = attrs.iter().any(|attribute| match &attribute.meta {
        syn::Meta::List(list) if list.path.is_ident("cfg") => list
            .tokens
            .to_string()
            .contains("certification-test-authority"),
        _ => false,
    });
    if certification {
        "current-certification"
    } else {
        "current"
    }
}

fn collect_exported_associated_items(
    facade: &Path,
    exports: &mut BTreeSet<(String, String, String)>,
) -> Result<(), String> {
    let root = facade
        .parent()
        .ok_or_else(|| "facade has no source root".to_owned())?;
    let sources = rust_sources(root)?;
    let exported = exported_types(exports, &sources)?;
    for source_path in sources {
        let source = std::fs::read_to_string(&source_path)
            .map_err(|error| format!("cannot read {}: {error}", source_path.display()))?;
        let file = syn::parse_file(&source)
            .map_err(|error| format!("invalid {}: {error}", source_path.display()))?;
        for item in file.items {
            match item {
                Item::Impl(item_impl) => {
                    let Some(type_name) = impl_type_name(&item_impl) else {
                        continue;
                    };
                    for (scope, surface, owner) in exported.iter().filter(|(_, surface, _)| {
                        surface.rsplit("::").next() == Some(type_name.as_str())
                    }) {
                        collect_impl_items(&item_impl, &[], Some((scope, surface, owner)), exports);
                    }
                }
                Item::Trait(item_trait) => {
                    for (scope, surface, owner) in exported.iter().filter(|(_, surface, _)| {
                        surface.rsplit("::").next() == Some(&item_trait.ident.to_string())
                    }) {
                        collect_trait_items(
                            &item_trait,
                            &[],
                            Some((scope, surface, owner)),
                            exports,
                        );
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn exported_types(
    exports: &BTreeSet<(String, String, String)>,
    sources: &[PathBuf],
) -> Result<Vec<(String, String, String)>, String> {
    let mut declared = BTreeSet::new();
    for source_path in sources {
        let source = std::fs::read_to_string(source_path)
            .map_err(|error| format!("cannot read {}: {error}", source_path.display()))?;
        let file = syn::parse_file(&source)
            .map_err(|error| format!("invalid {}: {error}", source_path.display()))?;
        for item in file.items {
            let identity = match item {
                Item::Enum(item) if public(&item.vis) => Some(item.ident),
                Item::Struct(item) if public(&item.vis) => Some(item.ident),
                Item::Trait(item) if public(&item.vis) => Some(item.ident),
                Item::Type(item) if public(&item.vis) => Some(item.ident),
                Item::Union(item) if public(&item.vis) => Some(item.ident),
                _ => None,
            };
            if let Some(identity) = identity {
                declared.insert(identity.to_string());
            }
        }
    }
    Ok(exports
        .iter()
        .filter(|(_, surface, owner)| {
            !owner.is_empty()
                && surface
                    .rsplit("::")
                    .next()
                    .is_some_and(|name| declared.contains(name))
        })
        .cloned()
        .collect())
}

fn impl_type_name(item_impl: &syn::ItemImpl) -> Option<String> {
    if item_impl.trait_.is_some() {
        return None;
    }
    let syn::Type::Path(self_type) = item_impl.self_ty.as_ref() else {
        return None;
    };
    self_type
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_owned()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?
        {
            let path = entry
                .map_err(|error| format!("cannot enumerate {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs")
                && path.file_name().is_none_or(|name| name != "tests.rs")
            {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

fn collect_trait_items(
    item_trait: &syn::ItemTrait,
    namespace: &[String],
    exported: Option<(&str, &str, &str)>,
    exports: &mut BTreeSet<(String, String, String)>,
) {
    let (base_scope, surface, owner) = match exported {
        Some((scope, surface, owner)) => (scope.to_owned(), surface.to_owned(), owner.to_owned()),
        None if !namespace.is_empty() => (
            configuration(&item_trait.attrs).into(),
            format!("{}::{}", namespace.join("::"), item_trait.ident),
            namespace.join("::"),
        ),
        None => return,
    };
    for item in &item_trait.items {
        let (ident, attrs) = match item {
            syn::TraitItem::Const(item) => (&item.ident, item.attrs.as_slice()),
            syn::TraitItem::Fn(item) => (&item.sig.ident, item.attrs.as_slice()),
            syn::TraitItem::Type(item) => (&item.ident, item.attrs.as_slice()),
            _ => continue,
        };
        let scope = if base_scope == "current-certification"
            || configuration(attrs) == "current-certification"
        {
            "current-certification"
        } else {
            "current"
        };
        exports.insert((scope.into(), format!("{surface}::{ident}"), owner.clone()));
    }
}

fn collect_impl_items(
    item_impl: &syn::ItemImpl,
    namespace: &[String],
    exported: Option<(&str, &str, &str)>,
    exports: &mut BTreeSet<(String, String, String)>,
) {
    let Some((base_scope, surface, owner)) = impl_identity(item_impl, namespace, exported) else {
        return;
    };
    let impl_scope = if base_scope == "current-certification"
        || configuration(&item_impl.attrs) == "current-certification"
    {
        "current-certification"
    } else {
        "current"
    };
    for item in &item_impl.items {
        insert_public_impl_item(item, (impl_scope, &surface, &owner), exports);
    }
}

fn impl_identity(
    item_impl: &syn::ItemImpl,
    namespace: &[String],
    exported: Option<(&str, &str, &str)>,
) -> Option<(String, String, String)> {
    if item_impl.trait_.is_some() {
        return None;
    }
    let syn::Type::Path(self_type) = item_impl.self_ty.as_ref() else {
        return None;
    };
    let type_name = self_type.path.segments.last()?.ident.to_string();
    match exported {
        Some((scope, surface, owner))
            if surface.rsplit("::").next() == Some(type_name.as_str()) =>
        {
            Some((scope.to_owned(), surface.to_owned(), owner.to_owned()))
        }
        Some(_) => None,
        None if namespace.is_empty() => None,
        None => Some((
            "current".into(),
            format!("{}::{type_name}", namespace.join("::")),
            namespace.join("::"),
        )),
    }
}

fn insert_public_impl_item(
    item: &syn::ImplItem,
    identity: (&str, &str, &str),
    exports: &mut BTreeSet<(String, String, String)>,
) {
    let (impl_scope, surface, owner) = identity;
    let (vis, ident, attrs) = match item {
        syn::ImplItem::Const(item) => (&item.vis, &item.ident, item.attrs.as_slice()),
        syn::ImplItem::Fn(item) => (&item.vis, &item.sig.ident, item.attrs.as_slice()),
        syn::ImplItem::Type(item) => (&item.vis, &item.ident, item.attrs.as_slice()),
        _ => return,
    };
    if !public(vis) {
        return;
    }
    let scope = if impl_scope == "current-certification"
        || configuration(attrs) == "current-certification"
    {
        "current-certification"
    } else {
        "current"
    };
    exports.insert((scope.into(), format!("{surface}::{ident}"), owner.into()));
}

fn insert_named(
    namespace: &[String],
    name: &str,
    exports: &mut BTreeSet<(String, String, String)>,
) {
    if namespace.is_empty() {
        return;
    }
    exports.insert((
        "current".into(),
        format!("{}::{name}", namespace.join("::")),
        namespace.join("::"),
    ));
}

fn module_path(parent: &Path, module: &str) -> Result<PathBuf, String> {
    let directory = parent
        .parent()
        .ok_or_else(|| format!("{} has no parent", parent.display()))?;
    let sibling = directory.join(format!("{module}.rs"));
    if sibling.is_file() {
        return Ok(sibling);
    }
    let nested = directory.join(module).join("mod.rs");
    nested.is_file().then_some(nested).ok_or_else(|| {
        format!(
            "public module `{module}` has no source beside {}",
            parent.display()
        )
    })
}

struct UseExportCollector<'a> {
    namespace: &'a [String],
    prefix: Vec<String>,
    scope: &'a str,
    exports: &'a mut BTreeSet<(String, String, String)>,
}

impl<'a> UseExportCollector<'a> {
    fn new(
        namespace: &'a [String],
        scope: &'a str,
        exports: &'a mut BTreeSet<(String, String, String)>,
    ) -> Self {
        Self {
            namespace,
            prefix: Vec::new(),
            scope,
            exports,
        }
    }

    fn collect(&mut self, tree: &UseTree) -> Result<(), String> {
        match tree {
            UseTree::Path(path) => {
                self.prefix.push(path.ident.to_string());
                self.collect(&path.tree)?;
                self.prefix.pop();
            }
            UseTree::Name(name) => self.insert(&name.ident.to_string()),
            UseTree::Rename(rename) => self.insert(&rename.rename.to_string()),
            UseTree::Group(group) => {
                for item in &group.items {
                    self.collect(item)?;
                }
            }
            UseTree::Glob(_) => {
                return Err("C.8 facade cannot be inventoried through a glob".into())
            }
        }
        Ok(())
    }

    fn insert(&mut self, name: &str) {
        let surface = [self.namespace, &[name.to_owned()]].concat().join("::");
        let owner = [self.namespace, self.prefix.as_slice()].concat().join("::");
        self.exports.insert((self.scope.to_owned(), surface, owner));
    }
}
