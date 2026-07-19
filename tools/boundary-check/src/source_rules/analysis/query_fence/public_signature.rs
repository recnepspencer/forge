//! Query types must not escape through governed public signatures.
//!
//! Classification is name based over configured Query roots and known facade
//! export names. The visitor intentionally does not resolve arbitrary local
//! aliases or renames; the Phase 6 facade-manifest ratchet owns that deferred
//! surface-drift gap.

use super::super::crate_modules::{GovernedCrate, ModuleGraph};
use super::super::public_reachability::{Reachability, ReachableItemKey};
use super::vocabulary::QueryVocabulary;
use crate::diagnostics::{Diagnostic, DiagnosticCode};
use proc_macro2::{TokenStream, TokenTree};
use std::collections::BTreeSet;
use syn::visit::{self, Visit};
use syn::{Fields, ForeignItem, ImplItem, Item, TraitItem, Type, Visibility};

pub(super) fn enforce(
    _governed: &GovernedCrate,
    graph: &ModuleGraph,
    reachable: &Reachability,
    vocabulary: &QueryVocabulary,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for (module_path, node) in &graph.modules {
        for item in &node.items {
            if let Item::Macro(item_macro) = item {
                let exported_definition = item_macro
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("macro_export"));
                let explicit_invocation = item_macro.ident.is_none();
                if exported_definition || explicit_invocation {
                    let names = token_identifiers(item_macro.mac.tokens.clone());
                    if names.iter().any(|name| name == "pub")
                        && names.iter().any(|name| {
                            matches!(
                                name.as_str(),
                                "fn" | "struct" | "enum" | "trait" | "type" | "const" | "static"
                            )
                        })
                    {
                        let hits = names
                            .into_iter()
                            .filter(|name| vocabulary.is_query_spelling(name))
                            .collect();
                        push_diagnostics(
                            &mut diagnostics,
                            &node.relative_source,
                            "exported macro",
                            hits,
                        );
                    }
                }
                continue;
            }
            if let Item::Impl(item_impl) = item {
                if impl_self_is_reachable(item_impl, reachable) {
                    for (name, hits) in inspect_impl(item_impl, vocabulary) {
                        push_diagnostics(&mut diagnostics, &node.relative_source, &name, hits);
                    }
                }
                continue;
            }
            if let Item::ForeignMod(foreign) = item {
                if reachable.public_modules.contains(module_path) {
                    for (name, hits) in inspect_foreign(foreign, vocabulary) {
                        push_diagnostics(&mut diagnostics, &node.relative_source, &name, hits);
                    }
                }
                continue;
            }
            let Some(name) = item_name(item) else {
                continue;
            };
            let key = ReachableItemKey {
                module_path: module_path.clone(),
                item_name: name.clone(),
            };
            if !reachable.items.contains(&key) {
                continue;
            }
            let hits = inspect_item(item, vocabulary);
            push_diagnostics(&mut diagnostics, &node.relative_source, &name, hits);
        }
    }
    diagnostics
}

fn push_diagnostics(
    diagnostics: &mut Vec<Diagnostic>,
    source: &str,
    name: &str,
    hits: BTreeSet<String>,
) {
    for spelling in hits {
        diagnostics.push(Diagnostic::new(
            DiagnosticCode::Bc3102QueryPublicSignature,
            format!("{source}::{name}"),
            format!(
                "Query type `{spelling}` is denied on a governed public signature; entry crates may consume Query audience types but may not publish them; legal home: rule_contracts.query_audience and workspaces/worth-query/crates/worth-query-*/src/facade.rs"
            ),
        ));
    }
}

fn impl_self_is_reachable(item_impl: &syn::ItemImpl, reachable: &Reachability) -> bool {
    let Type::Path(self_path) = item_impl.self_ty.as_ref() else {
        return false;
    };
    let Some(name) = self_path.path.segments.last() else {
        return false;
    };
    reachable
        .items
        .iter()
        .any(|key| key.item_name == name.ident.to_string())
}

fn inspect_impl(
    item_impl: &syn::ItemImpl,
    vocabulary: &QueryVocabulary,
) -> Vec<(String, BTreeSet<String>)> {
    let trait_impl = item_impl.trait_.is_some();
    let mut surfaces = Vec::new();
    let mut header = QueryTypeVisitor::new(vocabulary);
    header.visit_generics(&item_impl.generics);
    header.visit_type(&item_impl.self_ty);
    if let Some((_, trait_path, _)) = &item_impl.trait_ {
        header.visit_path(trait_path);
    }
    if !header.hits.is_empty() {
        surfaces.push(("impl header".to_owned(), header.hits));
    }
    surfaces.extend(
        item_impl
            .items
            .iter()
            .filter_map(|item| {
                let (name, inspect): (String, Box<dyn FnOnce(&mut QueryTypeVisitor<'_>)>) =
                    match item {
                        ImplItem::Fn(method)
                            if trait_impl || matches!(method.vis, Visibility::Public(_)) =>
                        {
                            let signature = &method.sig;
                            (
                                method.sig.ident.to_string(),
                                Box::new(move |v| v.visit_signature(signature)),
                            )
                        }
                        ImplItem::Type(assoc)
                            if trait_impl || matches!(assoc.vis, Visibility::Public(_)) =>
                        {
                            let ty = &assoc.ty;
                            (assoc.ident.to_string(), Box::new(move |v| v.visit_type(ty)))
                        }
                        ImplItem::Const(assoc)
                            if trait_impl || matches!(assoc.vis, Visibility::Public(_)) =>
                        {
                            let ty = &assoc.ty;
                            (assoc.ident.to_string(), Box::new(move |v| v.visit_type(ty)))
                        }
                        _ => return None,
                    };
                let mut visitor = QueryTypeVisitor::new(vocabulary);
                inspect(&mut visitor);
                Some((name, visitor.hits))
            })
            .collect::<Vec<_>>(),
    );
    surfaces
}

fn inspect_foreign(
    foreign: &syn::ItemForeignMod,
    vocabulary: &QueryVocabulary,
) -> Vec<(String, BTreeSet<String>)> {
    foreign
        .items
        .iter()
        .filter_map(|item| {
            let mut visitor = QueryTypeVisitor::new(vocabulary);
            let name = match item {
                ForeignItem::Fn(function) if matches!(function.vis, Visibility::Public(_)) => {
                    visitor.visit_signature(&function.sig);
                    function.sig.ident.to_string()
                }
                ForeignItem::Static(item) if matches!(item.vis, Visibility::Public(_)) => {
                    visitor.visit_type(&item.ty);
                    item.ident.to_string()
                }
                ForeignItem::Type(item) if matches!(item.vis, Visibility::Public(_)) => {
                    item.ident.to_string()
                }
                _ => return None,
            };
            Some((name, visitor.hits))
        })
        .collect()
}

fn inspect_item(item: &Item, vocabulary: &QueryVocabulary) -> BTreeSet<String> {
    let mut visitor = QueryTypeVisitor {
        vocabulary,
        hits: BTreeSet::new(),
    };
    match item {
        Item::Fn(item) => visitor.visit_signature(&item.sig),
        Item::Struct(item) => {
            visitor.visit_generics(&item.generics);
            visit_public_fields(&item.fields, &mut visitor);
        }
        Item::Enum(item) => {
            visitor.visit_generics(&item.generics);
            for variant in &item.variants {
                visit_all_fields(&variant.fields, &mut visitor);
            }
        }
        Item::Union(item) => {
            visitor.visit_generics(&item.generics);
            for field in &item.fields.named {
                if matches!(field.vis, Visibility::Public(_)) {
                    visitor.visit_type(&field.ty);
                }
            }
        }
        Item::Type(item) => {
            visitor.visit_generics(&item.generics);
            visitor.visit_type(&item.ty);
        }
        Item::Const(item) => visitor.visit_type(&item.ty),
        Item::Static(item) => visitor.visit_type(&item.ty),
        Item::Trait(item) => {
            visitor.visit_generics(&item.generics);
            for bound in &item.supertraits {
                visitor.visit_type_param_bound(bound);
            }
            for trait_item in &item.items {
                match trait_item {
                    TraitItem::Fn(method) => visitor.visit_signature(&method.sig),
                    TraitItem::Type(assoc) => {
                        visitor.visit_generics(&assoc.generics);
                        for bound in &assoc.bounds {
                            visitor.visit_type_param_bound(bound);
                        }
                        if let Some((_, ty)) = &assoc.default {
                            visitor.visit_type(ty);
                        }
                    }
                    TraitItem::Const(assoc) => visitor.visit_type(&assoc.ty),
                    _ => {}
                }
            }
        }
        Item::TraitAlias(item) => {
            visitor.visit_generics(&item.generics);
            for bound in &item.bounds {
                visitor.visit_type_param_bound(bound);
            }
        }
        _ => {}
    }
    visitor.hits
}

fn token_identifiers(tokens: TokenStream) -> Vec<String> {
    let mut names = Vec::new();
    for token in tokens {
        match token {
            TokenTree::Ident(ident) => names.push(ident.to_string()),
            TokenTree::Group(group) => names.extend(token_identifiers(group.stream())),
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
    names
}

fn visit_public_fields(fields: &Fields, visitor: &mut QueryTypeVisitor<'_>) {
    for field in fields {
        if matches!(field.vis, Visibility::Public(_)) {
            visitor.visit_type(&field.ty);
        }
    }
}

fn visit_all_fields(fields: &Fields, visitor: &mut QueryTypeVisitor<'_>) {
    for field in fields {
        visitor.visit_type(&field.ty);
    }
}

struct QueryTypeVisitor<'a> {
    vocabulary: &'a QueryVocabulary,
    hits: BTreeSet<String>,
}

impl<'a> QueryTypeVisitor<'a> {
    fn new(vocabulary: &'a QueryVocabulary) -> Self {
        Self {
            vocabulary,
            hits: BTreeSet::new(),
        }
    }
}

impl<'ast> Visit<'ast> for QueryTypeVisitor<'_> {
    fn visit_type(&mut self, ty: &'ast Type) {
        visit::visit_type(self, ty);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(first) = path.segments.first() {
            let first = first.ident.to_string();
            let last = path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .unwrap_or_else(|| first.clone());
            if self.vocabulary.is_query_root(&first)
                || self.vocabulary.is_query_type_name(&first)
                || self.vocabulary.is_query_type_name(&last)
            {
                self.hits.insert(
                    path.segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::"),
                );
            }
        }
        visit::visit_path(self, path);
    }
}

fn item_name(item: &Item) -> Option<String> {
    match item {
        Item::Const(item) => Some(item.ident.to_string()),
        Item::Enum(item) => Some(item.ident.to_string()),
        Item::Fn(item) => Some(item.sig.ident.to_string()),
        Item::Static(item) => Some(item.ident.to_string()),
        Item::Struct(item) => Some(item.ident.to_string()),
        Item::Trait(item) => Some(item.ident.to_string()),
        Item::TraitAlias(item) => Some(item.ident.to_string()),
        Item::Type(item) => Some(item.ident.to_string()),
        Item::Union(item) => Some(item.ident.to_string()),
        _ => None,
    }
}
