use std::collections::{BTreeMap, BTreeSet};

use syn::visit::{self, Visit};
use syn::{
    Attribute, Block, Expr, ExprCall, ExprClosure, ExprMethodCall, FnArg, ItemEnum, ItemFn,
    ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemType, ItemUnion, Local, PatIdent, Type,
};

use super::super::repository_root;

mod expression_activity;
use expression_activity::{
    expression_has_configuration, expression_identity, known_boolean, path_identity,
    pattern_matches_boolean,
};
#[cfg(test)]
mod tests;

pub(super) fn source_defines_surface(path: &str, surface: &str) -> Result<(), String> {
    let index = parse_source(path)?;
    let parts = surface.split("::").collect::<Vec<_>>();
    let defined = match parts.as_slice() {
        [owner, member] => index
            .associated_functions
            .contains(&(owner.to_string(), member.to_string())),
        [name] => index.declarations.contains(*name),
        _ => false,
    };
    defined
        .then_some(())
        .ok_or_else(|| format!("{path} does not define `{surface}` in parsed Rust syntax"))
}

pub(super) fn source_has_active_call_edges(
    path: &str,
    function: &str,
    required: &[&str],
) -> Result<(), String> {
    let index = parse_source(path)?;
    let calls = index
        .function_calls
        .get(function)
        .ok_or_else(|| format!("{path} lacks active causal function `{function}`"))?;
    let missing = required
        .iter()
        .filter(|callee| !calls.contains(**callee))
        .copied()
        .collect::<Vec<_>>();
    missing.is_empty().then_some(()).ok_or_else(|| {
        format!(
            "{path} function `{function}` lacks causal call edges {}; parsed calls are {}",
            missing.join(" "),
            calls.iter().cloned().collect::<Vec<_>>().join(" ")
        )
    })
}

fn parse_source(path: &str) -> Result<SyntaxIndex, String> {
    let source = std::fs::read_to_string(repository_root().join(path))
        .map_err(|error| format!("cannot read {path}: {error}"))?;
    parse_document(&source).map_err(|error| format!("cannot parse {path}: {error}"))
}

fn parse_document(source: &str) -> Result<SyntaxIndex, syn::Error> {
    let file = syn::parse_file(source)?;
    let mut index = SyntaxIndex::default();
    index.visit_file(&file);
    Ok(index)
}

#[derive(Default)]
struct SyntaxIndex {
    declarations: BTreeSet<String>,
    associated_functions: BTreeSet<(String, String)>,
    function_calls: BTreeMap<String, BTreeSet<String>>,
}

impl<'ast> Visit<'ast> for SyntaxIndex {
    fn visit_item_struct(&mut self, item: &'ast ItemStruct) {
        if has_configuration(&item.attrs) {
            return;
        }
        self.declarations.insert(item.ident.to_string());
        visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast ItemEnum) {
        if has_configuration(&item.attrs) {
            return;
        }
        self.declarations.insert(item.ident.to_string());
        visit::visit_item_enum(self, item);
    }

    fn visit_item_union(&mut self, item: &'ast ItemUnion) {
        if has_configuration(&item.attrs) {
            return;
        }
        self.declarations.insert(item.ident.to_string());
        visit::visit_item_union(self, item);
    }

    fn visit_item_type(&mut self, item: &'ast ItemType) {
        if has_configuration(&item.attrs) {
            return;
        }
        self.declarations.insert(item.ident.to_string());
        visit::visit_item_type(self, item);
    }

    fn visit_item_trait(&mut self, item: &'ast ItemTrait) {
        if has_configuration(&item.attrs) {
            return;
        }
        self.declarations.insert(item.ident.to_string());
        visit::visit_item_trait(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast ItemFn) {
        if has_configuration(&item.attrs) {
            return;
        }
        self.declarations.insert(item.sig.ident.to_string());
        self.function_calls
            .entry(item.sig.ident.to_string())
            .or_default()
            .extend(block_calls(&item.block, item.sig.inputs.iter()));
        visit::visit_item_fn(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast ItemImpl) {
        if has_configuration(&item.attrs) {
            return;
        }
        if let Type::Path(owner) = item.self_ty.as_ref() {
            if let Some(owner) = owner.path.segments.last() {
                for member in &item.items {
                    if let syn::ImplItem::Fn(function) = member {
                        if has_configuration(&function.attrs) {
                            continue;
                        }
                        self.associated_functions
                            .insert((owner.ident.to_string(), function.sig.ident.to_string()));
                        let owner = owner.ident.to_string();
                        let function_key = match &item.trait_ {
                            Some((_, trait_path, _)) => format!(
                                "{owner} as {}::{}",
                                trait_path
                                    .segments
                                    .last()
                                    .expect("trait path has a segment")
                                    .ident,
                                function.sig.ident
                            ),
                            None => format!("{owner}::{}", function.sig.ident),
                        };
                        self.function_calls
                            .entry(function_key)
                            .or_default()
                            .extend(block_calls(&function.block, function.sig.inputs.iter()));
                    }
                }
            }
        }
        visit::visit_item_impl(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        if !has_configuration(&item.attrs) {
            visit::visit_item_mod(self, item);
        }
    }
}

fn has_configuration(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr"))
}

fn block_calls<'ast>(
    block: &'ast Block,
    arguments: impl Iterator<Item = &'ast FnArg>,
) -> BTreeSet<String> {
    let mut bindings = LocalBindings::default();
    for argument in arguments {
        if let FnArg::Typed(argument) = argument {
            bindings.visit_pat(&argument.pat);
        }
    }
    bindings.visit_block(block);

    let mut calls = Calls {
        bindings: bindings.0,
        callees: BTreeSet::new(),
    };
    calls.visit_block(block);
    calls.callees
}

#[derive(Default)]
struct LocalBindings(BTreeSet<String>);

impl<'ast> Visit<'ast> for LocalBindings {
    fn visit_pat_ident(&mut self, pattern: &'ast PatIdent) {
        self.0.insert(pattern.ident.to_string());
        visit::visit_pat_ident(self, pattern);
    }

    fn visit_expr_closure(&mut self, _closure: &'ast ExprClosure) {}

    fn visit_item_fn(&mut self, _function: &'ast ItemFn) {}

    fn visit_item_impl(&mut self, _item: &'ast ItemImpl) {}
}

struct Calls {
    bindings: BTreeSet<String>,
    callees: BTreeSet<String>,
}

impl Calls {
    fn visit_argument(&mut self, parent: &str, argument: &Expr) {
        match argument {
            Expr::Closure(closure)
                if EAGER_CALLBACK_CALLS.contains(&parent) && !has_configuration(&closure.attrs) =>
            {
                let mut nested_bindings = LocalBindings(self.bindings.clone());
                for input in &closure.inputs {
                    nested_bindings.visit_pat(input);
                }
                let mut nested = Self {
                    bindings: nested_bindings.0,
                    callees: BTreeSet::new(),
                };
                nested.visit_expr(&closure.body);
                self.callees.extend(nested.callees);
            }
            expression => self.visit_expr(expression),
        }
    }

    fn visit_boolean_match(&mut self, matched: &syn::ExprMatch, value: bool) {
        self.visit_expr(&matched.expr);
        for arm in &matched.arms {
            if has_configuration(&arm.attrs) {
                continue;
            }
            let Some(pattern_matches) = pattern_matches_boolean(&arm.pat, value) else {
                if let Some((_, guard)) = &arm.guard {
                    self.visit_expr(guard);
                }
                self.visit_expr(&arm.body);
                continue;
            };
            if !pattern_matches {
                continue;
            }
            if let Some((_, guard)) = &arm.guard {
                match known_boolean(guard) {
                    Some(false) => continue,
                    Some(true) => {}
                    None => {
                        self.visit_expr(guard);
                        self.visit_expr(&arm.body);
                        continue;
                    }
                }
            }
            self.visit_expr(&arm.body);
            break;
        }
    }
}

const EAGER_CALLBACK_CALLS: &[&str] = &[
    "method:binding_compaction.for_each_record",
    "method:fate.requires_compaction_at().then",
];

impl<'ast> Visit<'ast> for Calls {
    fn visit_expr(&mut self, expression: &'ast Expr) {
        if expression_has_configuration(expression) {
            return;
        }
        match expression {
            Expr::If(branch) => match known_boolean(&branch.cond) {
                Some(true) => self.visit_block(&branch.then_branch),
                Some(false) => {
                    if let Some((_, alternative)) = &branch.else_branch {
                        self.visit_expr(alternative);
                    }
                }
                None => visit::visit_expr(self, expression),
            },
            Expr::While(loop_expression) if known_boolean(&loop_expression.cond) == Some(false) => {
            }
            Expr::Match(matched) => match known_boolean(&matched.expr) {
                Some(value) => self.visit_boolean_match(matched, value),
                None => visit::visit_expr(self, expression),
            },
            Expr::Binary(binary)
                if matches!(binary.op, syn::BinOp::And(_))
                    && known_boolean(&binary.left) == Some(false) =>
            {
                self.visit_expr(&binary.left);
            }
            Expr::Binary(binary)
                if matches!(binary.op, syn::BinOp::Or(_))
                    && known_boolean(&binary.left) == Some(true) =>
            {
                self.visit_expr(&binary.left);
            }
            _ => visit::visit_expr(self, expression),
        }
    }

    fn visit_local(&mut self, local: &'ast Local) {
        if !has_configuration(&local.attrs) {
            visit::visit_local(self, local);
        }
    }

    fn visit_expr_call(&mut self, call: &'ast ExprCall) {
        if has_configuration(&call.attrs) {
            return;
        }
        let mut parent = None;
        if let Expr::Path(callee) = call.func.as_ref() {
            if let Some(name) = path_identity(&callee.path) {
                let locally_bound =
                    callee.path.segments.len() == 1 && self.bindings.contains(&name);
                let callee = if locally_bound {
                    format!("callback:{name}")
                } else {
                    format!("path:{name}")
                };
                self.callees.insert(callee.clone());
                parent = Some(callee);
            }
        }
        for argument in &call.args {
            if let Some(parent) = &parent {
                self.visit_argument(parent, argument);
            } else if !matches!(argument, Expr::Closure(_)) {
                self.visit_expr(argument);
            }
        }
    }

    fn visit_expr_method_call(&mut self, call: &'ast ExprMethodCall) {
        if has_configuration(&call.attrs) {
            return;
        }
        self.visit_expr(&call.receiver);
        if let Some(receiver) = expression_identity(&call.receiver) {
            let callee = format!("method:{receiver}.{}", call.method);
            self.callees.insert(callee.clone());
            for argument in &call.args {
                self.visit_argument(&callee, argument);
            }
        } else {
            for argument in &call.args {
                if !matches!(argument, Expr::Closure(_)) {
                    self.visit_expr(argument);
                }
            }
        }
    }

    fn visit_expr_closure(&mut self, _closure: &'ast ExprClosure) {}

    fn visit_item_fn(&mut self, _function: &'ast ItemFn) {}

    fn visit_item_impl(&mut self, _item: &'ast ItemImpl) {}
}
