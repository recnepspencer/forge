use std::collections::BTreeSet;
use std::path::Path;

use syn::visit::Visit;

use super::Callable;

type OwnerIdentity = (&'static str, String);

pub(super) fn collect_file_callables(
    syntax: &syn::File,
    source: &str,
    owners: &BTreeSet<OwnerIdentity>,
    actual: &mut BTreeSet<Callable>,
) -> Result<(), String> {
    CallableCollector {
        source,
        owners,
        actual,
    }
    .collect_items(&syntax.items, false)
}

struct CallableCollector<'inventory> {
    source: &'inventory str,
    owners: &'inventory BTreeSet<OwnerIdentity>,
    actual: &'inventory mut BTreeSet<Callable>,
}

impl CallableCollector<'_> {
    fn collect_items(
        &mut self,
        items: &[syn::Item],
        ancestor_is_cfg_gated: bool,
    ) -> Result<(), String> {
        for item in items {
            match item {
                syn::Item::Impl(item) if item.trait_.is_none() => {
                    self.collect_inherent_impl(item, ancestor_is_cfg_gated)?;
                }
                syn::Item::Trait(item) => {
                    self.collect_extension_trait(item, ancestor_is_cfg_gated)?;
                }
                syn::Item::Mod(item) => {
                    if let Some((_, nested)) = &item.content {
                        self.collect_items(nested, ancestor_is_cfg_gated || has_cfg(&item.attrs))?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn collect_inherent_impl(
        &mut self,
        item: &syn::ItemImpl,
        ancestor_is_cfg_gated: bool,
    ) -> Result<(), String> {
        let Some(owner) = simple_type_name(&item.self_ty) else {
            return Ok(());
        };
        if !self.owners.contains(&("inherent", owner.clone())) {
            return Ok(());
        }
        reject_cfg_route(
            &owner,
            "<impl>",
            ancestor_is_cfg_gated || has_cfg(&item.attrs),
        )?;
        for member in &item.items {
            let syn::ImplItem::Fn(method) = member else {
                continue;
            };
            if !matches!(method.vis, syn::Visibility::Public(_)) {
                continue;
            }
            reject_cfg_route(
                &owner,
                &method.sig.ident.to_string(),
                has_cfg(&method.attrs),
            )?;
            self.actual.insert(Callable {
                kind: "inherent",
                owner: owner.clone(),
                method: method.sig.ident.to_string(),
                source: self.source.to_owned(),
            });
        }
        Ok(())
    }

    fn collect_extension_trait(
        &mut self,
        item: &syn::ItemTrait,
        ancestor_is_cfg_gated: bool,
    ) -> Result<(), String> {
        let owner = item.ident.to_string();
        if !self.owners.contains(&("extension_trait", owner.clone())) || !is_public(&item.vis) {
            return Ok(());
        }
        reject_cfg_route(
            &owner,
            "<trait>",
            ancestor_is_cfg_gated || has_cfg(&item.attrs),
        )?;
        for member in &item.items {
            let syn::TraitItem::Fn(method) = member else {
                continue;
            };
            reject_cfg_route(
                &owner,
                &method.sig.ident.to_string(),
                has_cfg(&method.attrs),
            )?;
            self.actual.insert(Callable {
                kind: "extension_trait",
                owner: owner.clone(),
                method: method.sig.ident.to_string(),
                source: self.source.to_owned(),
            });
        }
        Ok(())
    }
}

fn simple_type_name(ty: &syn::Type) -> Option<String> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

fn reject_cfg_route(owner: &str, method: &str, is_cfg_gated: bool) -> Result<(), String> {
    if is_cfg_gated {
        return Err(format!(
            "public callable `{owner}::{method}` may not be feature- or test-gated"
        ));
    }
    Ok(())
}

fn has_cfg(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("cfg"))
}

pub(super) fn reject_forbidden_symbols_in_source(
    path: &Path,
    source: &str,
    forbidden: &BTreeSet<&str>,
) -> Result<(), String> {
    let syntax = syn::parse_file(source)
        .map_err(|error| format!("{} should parse: {error}", path.display()))?;
    for item in syntax.items {
        for name in public_names(&item) {
            if forbidden.contains(name.as_str()) {
                return Err(format!(
                    "forbidden predecessor symbol `{name}` remains public in {}",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

fn public_names(item: &syn::Item) -> Vec<String> {
    match item {
        syn::Item::Type(item) if is_public(&item.vis) => vec![item.ident.to_string()],
        syn::Item::Struct(item) if is_public(&item.vis) => vec![item.ident.to_string()],
        syn::Item::Enum(item) if is_public(&item.vis) => vec![item.ident.to_string()],
        syn::Item::Trait(item) if is_public(&item.vis) => vec![item.ident.to_string()],
        syn::Item::Fn(item) if is_public(&item.vis) => vec![item.sig.ident.to_string()],
        syn::Item::Use(item) if is_public(&item.vis) => public_use_names(&item.tree),
        _ => Vec::new(),
    }
}

fn public_use_names(tree: &syn::UseTree) -> Vec<String> {
    match tree {
        syn::UseTree::Name(name) => vec![name.ident.to_string()],
        syn::UseTree::Rename(rename) => vec![rename.rename.to_string()],
        syn::UseTree::Path(path) => public_use_names(&path.tree),
        syn::UseTree::Group(group) => group.items.iter().flat_map(public_use_names).collect(),
        syn::UseTree::Glob(_) => Vec::new(),
    }
}

fn is_public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

pub(super) fn source_calls_method(source: &str, method: &str) -> Result<bool, String> {
    let syntax =
        syn::parse_file(source).map_err(|error| format!("caller source should parse: {error}"))?;
    let mut visitor = CallVisitor {
        method,
        found: false,
    };
    visitor.visit_file(&syntax);
    Ok(visitor.found)
}

struct CallVisitor<'method> {
    method: &'method str,
    found: bool,
}

impl<'ast> Visit<'ast> for CallVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.found |= node.method == self.method;
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func {
            self.found |= path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == self.method);
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let mut tokens = node.tokens.clone().into_iter().peekable();
        while let Some(token) = tokens.next() {
            if token.to_string() == self.method
                && tokens
                    .peek()
                    .is_some_and(|next| next.to_string().starts_with('('))
            {
                self.found = true;
                break;
            }
        }
        syn::visit::visit_macro(self, node);
    }
}
