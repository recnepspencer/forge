use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, Item, Lit, Meta, Token};

use super::{cfg_reachability, parse_source, production_attrs};

#[derive(Clone, Debug)]
pub(super) struct SourceModule {
    pub(super) logical: Vec<String>,
    pub(super) path: PathBuf,
    pub(super) inline: Vec<(String, usize)>,
    pub(super) publicly_reachable: bool,
    child_directory: PathBuf,
    path_attribute_directory: PathBuf,
}

pub(super) struct ModuleGraph {
    modules: BTreeMap<Vec<String>, Vec<SourceModule>>,
}

impl ModuleGraph {
    pub(super) fn build(source_root: &Path) -> Result<Self, String> {
        let entry = crate_entry(source_root)?;
        let directory = entry
            .parent()
            .ok_or_else(|| format!("{} has no parent", entry.display()))?
            .to_owned();
        let root = SourceModule {
            logical: Vec::new(),
            path: entry,
            inline: Vec::new(),
            publicly_reachable: true,
            child_directory: directory.clone(),
            path_attribute_directory: directory,
        };
        let mut graph = Self {
            modules: BTreeMap::new(),
        };
        let mut visiting = BTreeSet::new();
        graph.visit(root, &mut visiting)?;
        Ok(graph)
    }

    pub(super) fn modules_at(&self, logical: &[String]) -> &[SourceModule] {
        self.modules.get(logical).map(Vec::as_slice).unwrap_or(&[])
    }

    pub(super) fn module_for_file(&self, path: &Path) -> Option<&SourceModule> {
        self.modules
            .values()
            .flatten()
            .find(|module| module.path == path && module.inline.is_empty())
    }

    pub(super) fn modules(&self) -> impl Iterator<Item = &SourceModule> {
        self.modules.values().flatten()
    }

    pub(super) fn items(&self, module: &SourceModule) -> Result<Vec<Item>, String> {
        let syntax = parse_source(&module.path)?;
        inline_items(syntax.items, &module.inline, &module.path)
    }

    fn visit(
        &mut self,
        module: SourceModule,
        visiting: &mut BTreeSet<(PathBuf, Vec<(String, usize)>)>,
    ) -> Result<(), String> {
        let identity = (module.path.clone(), module.inline.clone());
        if !visiting.insert(identity.clone()) {
            return Err(format!(
                "recursive module inclusion at {}",
                module.path.display()
            ));
        }
        let items = self.items(&module)?;
        let variants = self.modules.entry(module.logical.clone()).or_default();
        if !variants.iter().any(|known| {
            known.path == module.path
                && known.inline == module.inline
                && known.publicly_reachable == module.publicly_reachable
        }) {
            variants.push(module.clone());
        }
        let mut inline_occurrences = BTreeMap::<String, usize>::new();
        for item in items {
            let Item::Mod(child) = item else {
                continue;
            };
            let name = child.ident.to_string();
            let occurrence = inline_occurrences.entry(name.clone()).or_default();
            let syntax_occurrence = *occurrence;
            *occurrence += 1;
            if !production_attrs(&child.attrs) {
                continue;
            }
            let mut logical = module.logical.clone();
            logical.push(name.clone());
            let publicly_reachable =
                module.publicly_reachable && super::production_public(&child.vis, &child.attrs);
            if child.content.is_some() {
                let mut inline = module.inline.clone();
                inline.push((name.clone(), syntax_occurrence));
                let directory = module.child_directory.join(&name);
                self.visit(
                    SourceModule {
                        logical,
                        path: module.path.clone(),
                        inline,
                        publicly_reachable,
                        child_directory: directory.clone(),
                        path_attribute_directory: directory,
                    },
                    visiting,
                )?;
                continue;
            }
            for path in declared_module_paths(&module, &child)? {
                self.visit(
                    SourceModule {
                        logical: logical.clone(),
                        child_directory: conventional_child_directory(&path)?,
                        path_attribute_directory: path
                            .parent()
                            .ok_or_else(|| format!("{} has no parent", path.display()))?
                            .to_owned(),
                        path,
                        inline: Vec::new(),
                        publicly_reachable,
                    },
                    visiting,
                )?;
            }
        }
        visiting.remove(&identity);
        Ok(())
    }
}

fn crate_entry(source_root: &Path) -> Result<PathBuf, String> {
    for name in ["lib.rs", "mod.rs"] {
        let candidate = source_root.join(name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(format!("{} has no lib.rs or mod.rs", source_root.display()))
}

fn inline_items(
    mut items: Vec<Item>,
    inline: &[(String, usize)],
    path: &Path,
) -> Result<Vec<Item>, String> {
    for (name, occurrence) in inline {
        let child = items
            .into_iter()
            .filter_map(|item| match item {
                Item::Mod(module) if module.ident == name => Some(module),
                _ => None,
            })
            .nth(*occurrence)
            .ok_or_else(|| format!("inline module {name} disappeared from {}", path.display()))?;
        items = child
            .content
            .ok_or_else(|| format!("inline module {name} has no body in {}", path.display()))?
            .1;
    }
    Ok(items)
}

fn declared_module_paths(
    parent: &SourceModule,
    module: &syn::ItemMod,
) -> Result<Vec<PathBuf>, String> {
    let (attributes, direct) = path_attributes(module)?;
    let mut candidates = attributes
        .iter()
        .map(|path| parent.path_attribute_directory.join(path))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    if !direct {
        candidates.extend(conventional_module_path(parent, module));
    }
    candidates.sort();
    candidates.dedup();
    if !candidates.is_empty() {
        return Ok(candidates);
    }
    Err(format!(
        "production module {} has no reachable source variant",
        module.ident
    ))
}

fn conventional_module_path(parent: &SourceModule, module: &syn::ItemMod) -> Option<PathBuf> {
    let name = module.ident.to_string();
    let flat = parent.child_directory.join(&name).with_extension("rs");
    if flat.is_file() {
        return Some(flat);
    }
    let nested = parent.child_directory.join(&name).join("mod.rs");
    nested.is_file().then_some(nested)
}

fn path_attributes(module: &syn::ItemMod) -> Result<(Vec<PathBuf>, bool), String> {
    let mut paths = Vec::new();
    let mut direct = false;
    for attribute in &module.attrs {
        if attribute.path().is_ident("path") {
            paths.push(path_from_meta(&attribute.meta, &module.ident.to_string())?);
            direct = true;
        } else if attribute.path().is_ident("cfg_attr") {
            paths.extend(cfg_attr_paths(attribute, &module.ident.to_string())?);
        }
    }
    Ok((paths, direct))
}

fn cfg_attr_paths(attribute: &syn::Attribute, module: &str) -> Result<Vec<PathBuf>, String> {
    let Meta::List(list) = &attribute.meta else {
        return Ok(Vec::new());
    };
    let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
    let values = parser
        .parse2(list.tokens.clone())
        .map_err(|error| format!("module {module} has malformed cfg_attr: {error}"))?;
    let Some(condition) = values.first() else {
        return Ok(Vec::new());
    };
    if cfg_reachability::truth(condition) == cfg_reachability::Truth::AlwaysFalse {
        return Ok(Vec::new());
    }
    values
        .iter()
        .skip(1)
        .filter(|meta| meta.path().is_ident("path"))
        .map(|meta| path_from_meta(meta, module))
        .collect()
}

fn path_from_meta(meta: &Meta, module: &str) -> Result<PathBuf, String> {
    let Meta::NameValue(value) = meta else {
        return Err(format!("module {module} has malformed path attribute"));
    };
    let Expr::Lit(literal) = &value.value else {
        return Err(format!("module {module} has non-literal path"));
    };
    let Lit::Str(path) = &literal.lit else {
        return Err(format!("module {module} has non-string path"));
    };
    Ok(PathBuf::from(path.value()))
}

fn conventional_child_directory(path: &Path) -> Result<PathBuf, String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent", path.display()))?;
    let file = path
        .file_name()
        .ok_or_else(|| format!("{} has no filename", path.display()))?;
    if file == "lib.rs" || file == "mod.rs" {
        return Ok(parent.to_owned());
    }
    let stem = path
        .file_stem()
        .ok_or_else(|| format!("{} has no stem", path.display()))?;
    Ok(parent.join(stem))
}
