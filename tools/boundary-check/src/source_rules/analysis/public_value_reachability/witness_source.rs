//! Parse the versioned downstream witness source into exact callable rows.

use crate::config::PublicValueWitnessPosture;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use syn::{FnArg, Item, PathArguments, ReturnType, Type, TypeParamBound, Visibility};

#[derive(Clone, Debug)]
pub(super) struct WitnessSignature {
    pub(super) posture: PublicValueWitnessPosture,
}

pub(super) fn load(
    root: &Path,
    relative_source: &str,
) -> Result<(PathBuf, BTreeMap<String, WitnessSignature>), String> {
    if Path::new(relative_source).components().any(|component| {
        matches!(
            component,
            std::path::Component::CurDir
                | std::path::Component::ParentDir
                | std::path::Component::RootDir
                | std::path::Component::Prefix(_)
        )
    }) {
        return Err(format!(
            "public-value witness source must be an exact repository-relative path: {relative_source}"
        ));
    }
    let canonical_root = fs::canonicalize(root)
        .map_err(|error| format!("canonicalize witness repository root: {error}"))?;
    let requested = root.join(relative_source);
    let source = fs::canonicalize(&requested).map_err(|error| {
        format!(
            "public-value witness source is missing {}: {error}",
            requested.display()
        )
    })?;
    if !source.starts_with(&canonical_root) || !source.is_file() {
        return Err(format!(
            "public-value witness source escapes the repository: {relative_source}"
        ));
    }
    let mut functions = BTreeMap::new();
    collect_file(&source, &[], &mut functions)?;
    Ok((source, functions))
}

fn collect_file(
    source_path: &Path,
    module_path: &[String],
    functions: &mut BTreeMap<String, WitnessSignature>,
) -> Result<(), String> {
    let source = fs::read_to_string(source_path)
        .map_err(|error| format!("read witness source {}: {error}", source_path.display()))?;
    let file = syn::parse_file(&source)
        .map_err(|error| format!("parse witness source {}: {error}", source_path.display()))?;
    collect_items(source_path, module_path, &file.items, functions)
}

fn collect_items(
    source_path: &Path,
    module_path: &[String],
    items: &[Item],
    functions: &mut BTreeMap<String, WitnessSignature>,
) -> Result<(), String> {
    validate_witness_bodies(source_path, items)?;
    for item in items {
        match item {
            Item::Fn(function) if !matches!(function.vis, Visibility::Inherited) => {
                let name = join_path(module_path, &function.sig.ident.to_string());
                let signature = signature(function).ok_or_else(|| {
                    format!(
                        "public-value witness `{name}` must return one exact worth-proof value or accept one exact callback value"
                    )
                })?;
                if functions.insert(name.clone(), signature).is_some() {
                    return Err(format!("duplicate public-value witness function `{name}`"));
                }
            }
            Item::Mod(module) => {
                let mut child = module_path.to_vec();
                child.push(module.ident.to_string());
                if let Some((_, items)) = &module.content {
                    collect_items(source_path, &child, items, functions)?;
                } else {
                    let child_source =
                        resolve_child_source(source_path, &module.ident.to_string())?;
                    collect_file(&child_source, &child, functions)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_witness_bodies(source_path: &Path, items: &[Item]) -> Result<(), String> {
    let forbidden_aliases = forbidden_call_aliases(items);
    let diverging = items
        .iter()
        .filter_map(|item| match item {
            Item::Fn(function)
                if matches!(function.sig.output, ReturnType::Type(_, ref ty)
                if matches!(ty.as_ref(), Type::Never(_))) =>
            {
                Some(function.sig.ident.to_string())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    for item in items {
        let Item::Fn(function) = item else {
            continue;
        };
        if function.sig.unsafety.is_some() {
            return Err(format!(
                "public-value witness source {} contains unsafe function `{}`",
                source_path.display(),
                function.sig.ident
            ));
        }
        let mut audit = WitnessBodyAudit {
            diverging: &diverging,
            forbidden_aliases: &forbidden_aliases,
            finding: None,
        };
        audit.visit_block(&function.block);
        if let Some(finding) = audit.finding {
            return Err(format!(
                "public-value witness source {} function `{}` {finding}",
                source_path.display(),
                function.sig.ident
            ));
        }
    }
    Ok(())
}

struct WitnessBodyAudit<'a> {
    diverging: &'a BTreeSet<String>,
    forbidden_aliases: &'a BTreeSet<String>,
    finding: Option<&'static str>,
}

impl WitnessBodyAudit<'_> {
    fn reject(&mut self, finding: &'static str) {
        if self.finding.is_none() {
            self.finding = Some(finding);
        }
    }
}

impl<'ast> Visit<'ast> for WitnessBodyAudit<'_> {
    fn visit_expr_unsafe(&mut self, expression: &'ast syn::ExprUnsafe) {
        self.reject("contains an unsafe block");
        syn::visit::visit_expr_unsafe(self, expression);
    }

    fn visit_expr_call(&mut self, expression: &'ast syn::ExprCall) {
        if let syn::Expr::Path(function) = expression.func.as_ref() {
            if let Some(name) = function
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
            {
                if matches!(
                    name.as_str(),
                    "zeroed" | "uninitialized" | "transmute" | "exit" | "abort"
                ) {
                    self.reject("calls a forging or early-exit primitive");
                }
                if self.forbidden_aliases.contains(&name) {
                    self.reject("calls an aliased forging or early-exit primitive");
                }
                if self.diverging.contains(&name) {
                    self.reject("calls a locally diverging function");
                }
            }
        }
        syn::visit::visit_expr_call(self, expression);
    }

    fn visit_expr_method_call(&mut self, expression: &'ast syn::ExprMethodCall) {
        if expression.method == "assume_init" {
            self.reject("calls a forging primitive");
        }
        syn::visit::visit_expr_method_call(self, expression);
    }

    fn visit_expr_macro(&mut self, expression: &'ast syn::ExprMacro) {
        if expression.mac.path.segments.last().is_some_and(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "panic" | "unreachable" | "todo"
            )
        }) {
            self.reject("contains a diverging macro");
        }
        syn::visit::visit_expr_macro(self, expression);
    }
}

fn forbidden_call_aliases(items: &[Item]) -> BTreeSet<String> {
    let mut aliases = BTreeSet::new();
    for item in items {
        if let Item::Use(item) = item {
            collect_forbidden_alias(&item.tree, &mut aliases);
        }
    }
    aliases
}

fn collect_forbidden_alias(tree: &syn::UseTree, aliases: &mut BTreeSet<String>) {
    match tree {
        syn::UseTree::Path(path) => collect_forbidden_alias(&path.tree, aliases),
        syn::UseTree::Name(name) if forbidden_call_name(&name.ident.to_string()) => {
            aliases.insert(name.ident.to_string());
        }
        syn::UseTree::Rename(rename) if forbidden_call_name(&rename.ident.to_string()) => {
            aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_forbidden_alias(item, aliases);
            }
        }
        _ => {}
    }
}

fn forbidden_call_name(name: &str) -> bool {
    matches!(
        name,
        "zeroed" | "uninitialized" | "transmute" | "exit" | "abort"
    )
}

fn signature(function: &syn::ItemFn) -> Option<WitnessSignature> {
    if let ReturnType::Type(_, output) = &function.sig.output {
        if exact_worth_proof_type(output) {
            return Some(WitnessSignature {
                posture: PublicValueWitnessPosture::Value,
            });
        }
    }
    function.sig.inputs.iter().find_map(|input| {
        let FnArg::Typed(input) = input else {
            return None;
        };
        callback_value_type(&input.ty).map(|()| WitnessSignature {
            posture: PublicValueWitnessPosture::Callback,
        })
    })
}

fn callback_value_type(ty: &Type) -> Option<()> {
    let bounds = match ty {
        Type::ImplTrait(ty) => &ty.bounds,
        Type::TraitObject(ty) => &ty.bounds,
        _ => return None,
    };
    bounds.iter().find_map(|bound| {
        let TypeParamBound::Trait(bound) = bound else {
            return None;
        };
        let segment = bound.path.segments.last()?;
        if !matches!(
            segment.ident.to_string().as_str(),
            "Fn" | "FnMut" | "FnOnce"
        ) {
            return None;
        }
        let PathArguments::Parenthesized(arguments) = &segment.arguments else {
            return None;
        };
        arguments
            .inputs
            .first()
            .filter(|ty| exact_worth_proof_type(ty))
            .map(|_| ())
    })
}

fn exact_worth_proof_type(ty: &Type) -> bool {
    let Type::Path(ty) = ty else {
        return false;
    };
    ty.qself.is_none()
        && ty
            .path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "worth_proof")
}

fn resolve_child_source(parent: &Path, module: &str) -> Result<PathBuf, String> {
    let directory = parent.parent().ok_or_else(|| {
        format!(
            "witness source {} has no parent directory",
            parent.display()
        )
    })?;
    for candidate in [
        directory.join(format!("{module}.rs")),
        directory.join(module).join("mod.rs"),
    ] {
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(format!(
        "witness module `{module}` declared by {} has no source file",
        parent.display()
    ))
}

fn join_path(module_path: &[String], name: &str) -> String {
    module_path
        .iter()
        .map(String::as_str)
        .chain(std::iter::once(name))
        .collect::<Vec<_>>()
        .join("::")
}
