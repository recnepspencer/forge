use syn::visit::{self, Visit};
use syn::{Block, Expr, File, ImplItem, Item, Member, Pat, Stmt};

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
        let actual = self.steps().iter().filter(|actual| *actual == step).count();
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
        let actual = self.steps().iter().filter(|actual| *actual == step).count();
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
        let steps = self.steps();
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
        self.require_exact(step, 0)
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

    fn steps(&self) -> Vec<String> {
        let mut collector = SemanticStepCollector::default();
        collector.visit_block(self.block);
        collector.steps
    }
}

fn collect_named_functions<'source>(
    items: &'source [Item],
    name: &str,
    blocks: &mut Vec<&'source Block>,
) {
    for item in items {
        match item {
            Item::Fn(function) if function.sig.ident == name => blocks.push(&function.block),
            Item::Impl(implementation) => {
                for item in &implementation.items {
                    if let ImplItem::Fn(function) = item {
                        if function.sig.ident == name {
                            blocks.push(&function.block);
                        }
                    }
                }
            }
            Item::Mod(module) => {
                if let Some((_, items)) = &module.content {
                    collect_named_functions(items, name, blocks);
                }
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct SemanticStepCollector {
    steps: Vec<String>,
}

impl<'ast> Visit<'ast> for SemanticStepCollector {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        visit::visit_expr_call(self, call);
        if let Expr::Path(path) = call.func.as_ref() {
            if let Some(segment) = path.path.segments.last() {
                self.steps.push(format!("call:{}", segment.ident));
            }
        }
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        visit::visit_expr_method_call(self, call);
        self.steps.push(format!("method:{}", call.method));
    }

    fn visit_expr_assign(&mut self, assignment: &'ast syn::ExprAssign) {
        if let (Expr::Field(field), Expr::Lit(value)) =
            (assignment.left.as_ref(), assignment.right.as_ref())
        {
            if let (Member::Named(name), syn::Lit::Bool(value)) = (&field.member, &value.lit) {
                self.steps.push(format!("assign:{name}={}", value.value));
            }
        }
        visit::visit_expr_assign(self, assignment);
    }

    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if let Some(segment) = path.path.segments.last() {
            self.steps.push(format!("path:{}", segment.ident));
        }
        visit::visit_expr_path(self, path);
    }

    fn visit_pat_tuple_struct(&mut self, pattern: &'ast syn::PatTupleStruct) {
        if let Some(segment) = pattern.path.segments.last() {
            self.steps.push(format!("path:{}", segment.ident));
        }
        visit::visit_pat_tuple_struct(self, pattern);
    }
}

#[cfg(test)]
mod tests {
    use super::ParsedRustSource;

    #[test]
    fn nested_calls_follow_rust_evaluation_order() {
        let source = ParsedRustSource::parse(
            r#"
                fn nested_order() {
                    owner.release_group_before_effect();
                    Reserved::from_members(pending).release_after_no_effect();
                }
            "#,
            "nested-call control",
        )
        .expect("parse nested-call control");
        source
            .function("nested_order")
            .expect("find nested-call control")
            .require_in_order(&[
                "method:release_group_before_effect",
                "call:from_members",
                "method:release_after_no_effect",
            ])
            .unwrap_or_else(|denial| {
                panic!("MUTANT_PREDICATE:wal-source-semantic-order-inverted-accepted: {denial}")
            });
    }
}
