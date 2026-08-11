use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use syn::{
    visit::{self, Visit},
    Attribute, Expr, Item, ItemMod, ItemUse, Lit, Macro, Meta, Token,
};

use super::constructor_syntax::{path_is_ident, semantic_identifier};

mod macro_provenance;
mod target_roots;
#[cfg(test)]
mod tests;

use macro_provenance::{macro_path_is_include, unproved_source_expansion, use_renames_include};
use target_roots::production_target_roots;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PendingSource {
    source: PathBuf,
    module_dir: PathBuf,
    path_attr_dir: PathBuf,
}

struct SourceContext {
    module_dir: PathBuf,
    path_attr_dir: PathBuf,
    include_dir: PathBuf,
}

pub(super) fn production_rust_sources(workspace: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = production_target_roots(workspace)?;
    let mut visited = BTreeSet::new();
    let mut source_paths = BTreeSet::new();
    while let Some(next) = pending.pop() {
        if !visited.insert(next.clone()) {
            continue;
        }
        let source = next.source;
        source_paths.insert(source.clone());
        let text = std::fs::read_to_string(&source)
            .map_err(|error| format!("cannot read {}: {error}", source.display()))?;
        let syntax = parse_reachable_source(&text)
            .map_err(|error| format!("invalid Rust source {}: {error}", source.display()))?;
        let context = SourceContext {
            module_dir: next.module_dir,
            path_attr_dir: next.path_attr_dir,
            include_dir: source
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .to_path_buf(),
        };
        collect_external_modules(&syntax.items, &context, &mut pending)?;
    }
    Ok(source_paths.into_iter().collect())
}

fn collect_external_modules(
    items: &[Item],
    context: &SourceContext,
    pending: &mut Vec<PendingSource>,
) -> Result<(), String> {
    let mut visitor = SourceGraphVisitor {
        context,
        pending,
        denial: None,
    };
    for item in items {
        visitor.visit_item(item);
        if visitor.denial.is_some() {
            break;
        }
    }
    visitor.denial.map_or(Ok(()), Err)
}

fn module_source(
    module: &ItemMod,
    module_dir: &Path,
    source_dir: &Path,
) -> Result<PathBuf, String> {
    reject_cfg_attr_path(&module.attrs)?;
    let declared: Vec<_> = module
        .attrs
        .iter()
        .filter(|attr| path_is_ident(attr.path(), "path"))
        .map(|attr| path_attribute(attr, source_dir))
        .collect::<Result<_, _>>()?;
    if declared.len() == 1 {
        return Ok(declared[0].clone());
    }
    if !declared.is_empty() {
        return Err(format!(
            "module {} has multiple path attributes",
            module.ident
        ));
    }
    let name = semantic_identifier(&module.ident);
    let candidates = [
        module_dir.join(format!("{name}.rs")),
        module_dir.join(name).join("mod.rs"),
    ];
    let existing: Vec<_> = candidates
        .into_iter()
        .filter(|path| path.is_file())
        .collect();
    if existing.len() != 1 {
        return Err(format!(
            "production module {} must resolve to one source under {}",
            module.ident,
            module_dir.display()
        ));
    }
    canonical(&existing[0])
}

fn reject_cfg_attr_path(attrs: &[Attribute]) -> Result<(), String> {
    for attr in attrs
        .iter()
        .filter(|attr| path_is_ident(attr.path(), "cfg_attr"))
    {
        let values = attr
            .parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)
            .map_err(|error| format!("invalid cfg_attr: {error}"))?;
        if values
            .iter()
            .skip(1)
            .map(cfg_meta_contains_path)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .any(|contains_path| contains_path)
        {
            return Err(
                "cfg_attr module path cannot prove one production source without cfg correlation"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn cfg_meta_contains_path(meta: &Meta) -> Result<bool, String> {
    if path_is_ident(meta.path(), "path") {
        return Ok(true);
    }
    let Meta::List(list) = meta else {
        return Ok(false);
    };
    if !path_is_ident(&list.path, "cfg_attr") {
        return Ok(false);
    }
    let nested = list
        .parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)
        .map_err(|error| format!("invalid nested cfg_attr: {error}"))?;
    for value in nested.iter().skip(1) {
        if cfg_meta_contains_path(value)? {
            return Ok(true);
        }
    }
    Ok(false)
}

struct SourceGraphVisitor<'a> {
    context: &'a SourceContext,
    pending: &'a mut Vec<PendingSource>,
    denial: Option<String>,
}

impl<'ast> Visit<'ast> for SourceGraphVisitor<'_> {
    fn visit_item(&mut self, node: &'ast Item) {
        if let Some(attrs) = item_attributes(node) {
            match non_test_reachable(attrs) {
                Ok(false) => return,
                Ok(true) => {}
                Err(denial) => {
                    self.denial = Some(denial);
                    return;
                }
            }
        }
        visit::visit_item(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        let child_dir = self
            .context
            .module_dir
            .join(semantic_identifier(&node.ident));
        if let Some((_, items)) = &node.content {
            let child_context = SourceContext {
                module_dir: child_dir.clone(),
                path_attr_dir: child_dir,
                include_dir: self.context.include_dir.clone(),
            };
            if let Err(denial) = collect_external_modules(items, &child_context, self.pending) {
                self.denial = Some(denial);
            }
            return;
        }
        match module_source(node, &self.context.module_dir, &self.context.path_attr_dir) {
            Ok(source) => {
                let child_module_dir = module_directory(&source);
                let child_path_attr_dir = source
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .to_path_buf();
                self.pending.push(PendingSource {
                    source,
                    module_dir: child_module_dir,
                    path_attr_dir: child_path_attr_dir,
                });
            }
            Err(denial) => self.denial = Some(denial),
        }
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        if use_renames_include(&node.tree) {
            self.denial = Some(
                "include macro import alias cannot prove production source reachability".to_owned(),
            );
            return;
        }
        visit::visit_item_use(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        if macro_path_is_include(&node.path) {
            match syn::parse2::<syn::LitStr>(node.tokens.clone()) {
                Ok(included) => match canonical(&self.context.include_dir.join(included.value())) {
                    Ok(source) => {
                        let source_dir = source
                            .parent()
                            .unwrap_or_else(|| Path::new(""))
                            .to_path_buf();
                        self.pending.push(PendingSource {
                            source,
                            module_dir: source_dir.clone(),
                            path_attr_dir: source_dir,
                        });
                    }
                    Err(denial) => self.denial = Some(denial),
                },
                Err(_) => {
                    self.denial = Some(
                        "generated or computed Rust include cannot prove production reachability"
                            .to_owned(),
                    );
                }
            }
        } else if let Some(expansion) = unproved_source_expansion(&node.tokens) {
            self.denial = Some(format!(
                "macro-generated {expansion} cannot prove production source reachability"
            ));
        }
        visit::visit_macro(self, node);
    }
}

fn item_attributes(item: &Item) -> Option<&[Attribute]> {
    match item {
        Item::Const(item) => Some(&item.attrs),
        Item::Enum(item) => Some(&item.attrs),
        Item::ExternCrate(item) => Some(&item.attrs),
        Item::Fn(item) => Some(&item.attrs),
        Item::ForeignMod(item) => Some(&item.attrs),
        Item::Impl(item) => Some(&item.attrs),
        Item::Macro(item) => Some(&item.attrs),
        Item::Mod(item) => Some(&item.attrs),
        Item::Static(item) => Some(&item.attrs),
        Item::Struct(item) => Some(&item.attrs),
        Item::Trait(item) => Some(&item.attrs),
        Item::TraitAlias(item) => Some(&item.attrs),
        Item::Type(item) => Some(&item.attrs),
        Item::Union(item) => Some(&item.attrs),
        Item::Use(item) => Some(&item.attrs),
        Item::Verbatim(_) | _ => None,
    }
}

fn parse_reachable_source(source: &str) -> syn::Result<syn::File> {
    syn::parse_file(source).or_else(|file_error| {
        syn::parse_file(&format!("fn __included_source() {{ {source} }}")).map_err(|_| file_error)
    })
}

fn module_directory(source: &Path) -> PathBuf {
    let parent = source.parent().unwrap_or_else(|| Path::new(""));
    match source.file_stem().and_then(|stem| stem.to_str()) {
        Some("mod") => parent.to_path_buf(),
        Some(stem) => parent.join(stem),
        None => parent.to_path_buf(),
    }
}

fn path_attribute(attr: &Attribute, module_dir: &Path) -> Result<PathBuf, String> {
    let Meta::NameValue(path) = &attr.meta else {
        return Err("module path attribute must be a string value".to_owned());
    };
    let Expr::Lit(value) = &path.value else {
        return Err("module path attribute must be a literal".to_owned());
    };
    let Lit::Str(value) = &value.lit else {
        return Err("module path attribute must be a string".to_owned());
    };
    let relative = PathBuf::from(value.value());
    let resolved = module_dir.join(&relative);
    if !resolved.is_file() {
        return Err(format!(
            "module path {} must resolve from {}",
            relative.display(),
            module_dir.display()
        ));
    }
    canonical(&resolved)
}

fn non_test_reachable(attrs: &[Attribute]) -> Result<bool, String> {
    for attr in attrs {
        if !path_is_ident(attr.path(), "cfg") {
            continue;
        }
        let predicate: Meta = attr
            .parse_args()
            .map_err(|error| format!("invalid cfg predicate: {error}"))?;
        if !possible_truth(&predicate)?.1 {
            return Ok(false);
        }
    }
    Ok(true)
}

fn possible_truth(meta: &Meta) -> Result<(bool, bool), String> {
    match meta {
        Meta::Path(path) if path_is_ident(path, "test") => Ok((true, false)),
        Meta::Path(_) | Meta::NameValue(_) => Ok((true, true)),
        Meta::List(list) if path_is_ident(&list.path, "all") => combine_predicates(list, true),
        Meta::List(list) if path_is_ident(&list.path, "any") => combine_predicates(list, false),
        Meta::List(list) if path_is_ident(&list.path, "not") => {
            let value: Meta = list
                .parse_args()
                .map_err(|error| format!("invalid cfg not predicate: {error}"))?;
            let possible = possible_truth(&value)?;
            Ok((possible.1, possible.0))
        }
        Meta::List(_) => Ok((true, true)),
    }
}

fn combine_predicates(list: &syn::MetaList, all: bool) -> Result<(bool, bool), String> {
    let values = list
        .parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)
        .map_err(|error| format!("invalid cfg predicate list: {error}"))?;
    let mut can_false = !all;
    let mut can_true = all;
    for value in values {
        let possible = possible_truth(&value)?;
        if all {
            can_false |= possible.0;
            can_true &= possible.1;
        } else {
            can_false &= possible.0;
            can_true |= possible.1;
        }
    }
    Ok((can_false, can_true))
}

fn canonical(path: &Path) -> Result<PathBuf, String> {
    let path = path
        .canonicalize()
        .map_err(|error| format!("cannot resolve {}: {error}", path.display()))?;
    let rendered = path.to_string_lossy();
    Ok(rendered
        .strip_prefix(r"\\?\")
        .map_or(path.clone(), PathBuf::from))
}
