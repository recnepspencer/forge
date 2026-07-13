//! Known-safe attribute/derive allowlist and opaque expansion detection.
//!
//! Opaque attributes and custom derives on externally reachable surfaces can mint
//! unsealed ceremony signatures the AST pass cannot inspect. Nested `cfg_attr`
//! is expanded recursively so smuggled attributes cannot hide behind predicates.

use syn::{Attribute, Meta};

/// Builtin attribute paths that cannot mint authority-generic public signatures.
const SAFE_ATTR_PATHS: &[&str] = &[
    "allow",
    "warn",
    "deny",
    "forbid",
    "cfg",
    "cfg_attr",
    "deprecated",
    "doc",
    "inline",
    "cold",
    "must_use",
    "non_exhaustive",
    "repr",
    "track_caller",
    "must_not_suspend",
    "target_feature",
    "export_name",
    "link_name",
    "link_section",
    "no_mangle",
    "used",
    "automatically_derived",
    "prelude_import",
    "macro_export",
    "macro_use",
    "path",
    "test",
    "bench",
    "ignore",
    "should_panic",
    "proc_macro",
    "proc_macro_derive",
    "proc_macro_attribute",
];

/// Builtin derive helpers that do not invent public authority-generic ceremonies.
const SAFE_DERIVES: &[&str] = &[
    "Clone",
    "Copy",
    "Debug",
    "Default",
    "Eq",
    "Hash",
    "Ord",
    "PartialEq",
    "PartialOrd",
];

/// First opaque attribute/derive path among `attrs`, if any.
pub(super) fn first_opaque_attribute(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if let Some(path) = opaque_attr_path(attr) {
            return Some(path);
        }
    }
    None
}

fn opaque_attr_path(attr: &Attribute) -> Option<String> {
    let path = path_display(attr.path());
    let root = attr
        .path()
        .segments
        .first()
        .map(|seg| seg.ident.to_string())
        .unwrap_or_default();

    if root == "derive" {
        return opaque_derive_component(attr);
    }
    if root == "cfg_attr" {
        return opaque_cfg_attr_nested(attr);
    }
    if SAFE_ATTR_PATHS.contains(&root.as_str()) {
        return None;
    }
    Some(path)
}

fn opaque_derive_component(attr: &Attribute) -> Option<String> {
    let Meta::List(list) = &attr.meta else {
        return Some("derive".to_owned());
    };
    opaque_derive_tokens(&list.tokens)
}

fn opaque_derive_tokens(tokens: &proc_macro2::TokenStream) -> Option<String> {
    let nested = tokens.to_string();
    for part in nested.split(',') {
        let name = part.trim().split("::").last().unwrap_or("").trim();
        if name.is_empty() {
            continue;
        }
        if !SAFE_DERIVES.contains(&name) {
            return Some(format!("derive({name})"));
        }
    }
    None
}

/// Inspect nested attributes carried by `cfg_attr(predicate, attr, ...)`.
fn opaque_cfg_attr_nested(attr: &Attribute) -> Option<String> {
    let Meta::List(list) = &attr.meta else {
        return Some("cfg_attr".to_owned());
    };
    let groups = split_top_level_comma_groups(list.tokens.clone());
    if groups.len() < 2 {
        return None;
    }
    for nested in groups.into_iter().skip(1) {
        if let Some(path) = opaque_nested_meta_tokens(nested) {
            return Some(path);
        }
    }
    None
}

fn opaque_nested_meta_tokens(tokens: proc_macro2::TokenStream) -> Option<String> {
    let meta: Meta = syn::parse2(tokens).ok()?;
    opaque_nested_meta(&meta)
}

fn opaque_nested_meta(meta: &Meta) -> Option<String> {
    let path = match meta {
        Meta::Path(path)
        | Meta::List(syn::MetaList { path, .. })
        | Meta::NameValue(syn::MetaNameValue { path, .. }) => path,
    };
    let root = path
        .segments
        .first()
        .map(|seg| seg.ident.to_string())
        .unwrap_or_default();
    if root == "derive" {
        let Meta::List(list) = meta else {
            return Some("derive".to_owned());
        };
        return opaque_derive_tokens(&list.tokens);
    }
    if root == "cfg_attr" {
        let Meta::List(list) = meta else {
            return Some("cfg_attr".to_owned());
        };
        let groups = split_top_level_comma_groups(list.tokens.clone());
        for nested in groups.into_iter().skip(1) {
            if let Some(path) = opaque_nested_meta_tokens(nested) {
                return Some(path);
            }
        }
        return None;
    }
    if SAFE_ATTR_PATHS.contains(&root.as_str()) {
        return None;
    }
    Some(path_display(path))
}

fn split_top_level_comma_groups(tokens: proc_macro2::TokenStream) -> Vec<proc_macro2::TokenStream> {
    use proc_macro2::TokenTree;
    let mut groups = Vec::new();
    let mut current = Vec::new();
    for tree in tokens {
        match &tree {
            TokenTree::Punct(punct) if punct.as_char() == ',' => {
                groups.push(current.into_iter().collect());
                current = Vec::new();
            }
            _ => current.push(tree),
        }
    }
    if !current.is_empty() {
        groups.push(current.into_iter().collect());
    }
    groups
}

fn path_display(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
