use syn::{Attribute, Block, Expr, File, ImplItem, Item, Pat, Stmt};

mod semantic_projection;

use semantic_projection::{expression_contains_proof_step, project_block, SemanticProjection};

pub(super) struct ParsedRustSource {
    file: File,
}

impl ParsedRustSource {
    pub(super) fn parse(source: &str, owner: &str) -> Result<Self, String> {
        syn::parse_file(source)
            .map(|file| Self { file })
            .map_err(|error| format!("cannot parse {owner} as Rust syntax: {error}"))
    }

    pub(super) fn function(&self, name: &str) -> Result<FunctionSyntax<'_>, String> {
        let mut blocks = Vec::new();
        collect_named_functions(&self.file.items, name, &mut blocks);
        match blocks.as_slice() {
            [block] => Ok(FunctionSyntax {
                name: name.to_owned(),
                block,
            }),
            [] => Err(format!("Rust source omits semantic function `{name}`")),
            _ => Err(format!(
                "Rust source has competing functions named `{name}`"
            )),
        }
    }
}

pub(super) struct FunctionSyntax<'source> {
    name: String,
    block: &'source Block,
}

impl FunctionSyntax<'_> {
    pub(super) fn require_exact(&self, step: &str, expected: usize) -> Result<(), String> {
        let actual = self
            .projection()
            .proof_steps
            .iter()
            .filter(|actual| *actual == step)
            .count();
        if actual == expected {
            Ok(())
        } else {
            Err(format!(
                "`{}` requires {expected} `{step}` semantic steps, found {actual}",
                self.name
            ))
        }
    }

    pub(super) fn require_at_least(&self, step: &str, minimum: usize) -> Result<(), String> {
        let actual = self
            .projection()
            .proof_steps
            .iter()
            .filter(|actual| *actual == step)
            .count();
        if actual >= minimum {
            Ok(())
        } else {
            Err(format!(
                "`{}` requires at least {minimum} `{step}` semantic steps, found {actual}",
                self.name
            ))
        }
    }

    pub(super) fn require_in_order(&self, required: &[&str]) -> Result<(), String> {
        let steps = self.projection().proof_steps;
        let mut offset = 0;
        for required_step in required {
            let Some(found) = steps[offset..]
                .iter()
                .position(|actual| actual == required_step)
            else {
                return Err(format!(
                    "`{}` lost ordered semantic step `{required_step}` after {:?}",
                    self.name,
                    &required[..required
                        .iter()
                        .position(|candidate| candidate == required_step)
                        .unwrap_or_default()]
                ));
            };
            offset += found + 1;
        }
        Ok(())
    }

    pub(super) fn deny(&self, step: &str) -> Result<(), String> {
        if self.projection().contains_prohibited(step) {
            Err(format!(
                "`{}` contains prohibited semantic step `{step}`",
                self.name
            ))
        } else {
            Ok(())
        }
    }

    pub(super) fn require_collected_mapping_step(&self, step: &str) -> Result<(), String> {
        let actual = self
            .block
            .stmts
            .iter()
            .filter_map(local_initializer)
            .filter_map(collected_mapping_body)
            .filter(|body| expression_contains_proof_step(body, step))
            .count();
        if actual == 1 {
            Ok(())
        } else {
            Err(format!(
                "`{}` requires one collected-mapping `{step}` semantic step, found {actual}",
                self.name
            ))
        }
    }

    pub(super) fn let_initializer(&self, binding: &str) -> Result<&Expr, String> {
        self.block
            .stmts
            .iter()
            .find_map(|statement| match statement {
                Stmt::Local(local) => match &local.pat {
                    Pat::Ident(pattern) if pattern.ident == binding => {
                        local.init.as_ref().map(|init| init.expr.as_ref())
                    }
                    _ => None,
                },
                _ => None,
            })
            .ok_or_else(|| format!("`{}` omits semantic binding `{binding}`", self.name))
    }

    fn projection(&self) -> SemanticProjection {
        project_block(self.block)
    }
}

fn local_initializer(statement: &Stmt) -> Option<&Expr> {
    match statement {
        Stmt::Local(local) => local.init.as_ref().map(|init| init.expr.as_ref()),
        _ => None,
    }
}

fn collected_mapping_body(expression: &Expr) -> Option<&Expr> {
    let Expr::MethodCall(collect) = peel_expression_wrappers(expression) else {
        return None;
    };
    if collect.method != "collect" || !collect.args.is_empty() {
        return None;
    }
    let Expr::MethodCall(mapping) = peel_expression_wrappers(&collect.receiver) else {
        return None;
    };
    if mapping.method != "map" || mapping.args.len() != 1 {
        return None;
    }
    match mapping.args.first() {
        Some(Expr::Closure(closure)) => Some(&closure.body),
        _ => None,
    }
}

fn peel_expression_wrappers(mut expression: &Expr) -> &Expr {
    loop {
        expression = match expression {
            Expr::Group(group) => &group.expr,
            Expr::Paren(parenthesized) => &parenthesized.expr,
            Expr::Try(attempt) => &attempt.expr,
            _ => return expression,
        };
    }
}

fn collect_named_functions<'source>(
    items: &'source [Item],
    name: &str,
    blocks: &mut Vec<&'source Block>,
) {
    for item in items {
        match item {
            Item::Fn(function)
                if function.sig.ident == name && !is_exact_cfg_test(&function.attrs) =>
            {
                blocks.push(&function.block);
            }
            Item::Impl(implementation) if !is_exact_cfg_test(&implementation.attrs) => {
                for item in &implementation.items {
                    if let ImplItem::Fn(function) = item {
                        if function.sig.ident == name && !is_exact_cfg_test(&function.attrs) {
                            blocks.push(&function.block);
                        }
                    }
                }
            }
            Item::Mod(module) if !is_exact_cfg_test(&module.attrs) => {
                if let Some((_, items)) = &module.content {
                    collect_named_functions(items, name, blocks);
                }
            }
            _ => {}
        }
    }
}

fn is_exact_cfg_test(attributes: &[Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg")
            && matches!(
                &attribute.meta,
                syn::Meta::List(configuration) if configuration.tokens.to_string() == "test"
            )
    })
}

#[cfg(test)]
mod tests;
