use syn::visit::{self, Visit};
use syn::{Block, Expr, Member, Pat, Stmt};

pub(super) struct SemanticProjection {
    pub(super) proof_steps: Vec<String>,
    observed_steps: Vec<String>,
    macro_identifiers: Vec<String>,
}

impl SemanticProjection {
    pub(super) fn contains_prohibited(&self, step: &str) -> bool {
        self.observed_steps.iter().any(|actual| actual == step)
            || prohibited_identifier(step).is_some_and(|identifier| {
                self.macro_identifiers
                    .iter()
                    .any(|actual| actual == identifier)
            })
    }
}

pub(super) fn project_block(block: &Block) -> SemanticProjection {
    let mut collector = SemanticStepCollector::default();
    collector.visit_block(block);
    SemanticProjection {
        proof_steps: collector.proof_steps,
        observed_steps: collector.observed_steps,
        macro_identifiers: collector.macro_identifiers,
    }
}

pub(super) fn expression_contains_proof_step(expression: &Expr, step: &str) -> bool {
    let mut collector = SemanticStepCollector::default();
    collector.visit_expr(expression);
    collector.proof_steps.iter().any(|actual| actual == step)
}

fn prohibited_identifier(step: &str) -> Option<&str> {
    step.strip_prefix("method:")
        .or_else(|| step.strip_prefix("call:"))
        .or_else(|| step.strip_prefix("path:"))
        .or_else(|| {
            step.strip_prefix("assign:")
                .and_then(|assignment| assignment.split('=').next())
        })
}

#[derive(Default)]
struct SemanticStepCollector {
    proof_steps: Vec<String>,
    observed_steps: Vec<String>,
    macro_identifiers: Vec<String>,
    proof_suspension_depth: usize,
}

impl SemanticStepCollector {
    fn record(&mut self, step: String) {
        self.observed_steps.push(step.clone());
        if self.proof_suspension_depth == 0 {
            self.proof_steps.push(step);
        }
    }

    fn without_positive_proof(&mut self, visit: impl FnOnce(&mut Self)) {
        self.proof_suspension_depth += 1;
        visit(self);
        self.proof_suspension_depth -= 1;
    }

    fn record_macro_identifiers(&mut self, tokens: proc_macro2::TokenStream) {
        for token in tokens {
            match token {
                proc_macro2::TokenTree::Group(group) => {
                    self.record_macro_identifiers(group.stream());
                }
                proc_macro2::TokenTree::Ident(identifier) => {
                    self.macro_identifiers.push(identifier.to_string());
                }
                proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for SemanticStepCollector {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        visit::visit_expr_call(self, call);
        if let Expr::Path(path) = call.func.as_ref() {
            if let Some(segment) = path.path.segments.last() {
                self.record(format!("call:{}", segment.ident));
            }
        }
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        visit::visit_expr_method_call(self, call);
        self.record(format!("method:{}", call.method));
    }

    fn visit_expr_assign(&mut self, assignment: &'ast syn::ExprAssign) {
        visit::visit_expr_assign(self, assignment);
        if let (Expr::Field(field), Expr::Lit(value)) =
            (assignment.left.as_ref(), assignment.right.as_ref())
        {
            if let (Member::Named(name), syn::Lit::Bool(value)) = (&field.member, &value.lit) {
                self.record(format!("assign:{name}={}", value.value));
            }
        }
    }

    fn visit_expr_async(&mut self, deferred: &'ast syn::ExprAsync) {
        self.without_positive_proof(|collector| visit::visit_expr_async(collector, deferred));
    }

    fn visit_expr_closure(&mut self, closure: &'ast syn::ExprClosure) {
        self.without_positive_proof(|collector| visit::visit_expr_closure(collector, closure));
    }

    fn visit_expr_const(&mut self, deferred: &'ast syn::ExprConst) {
        self.without_positive_proof(|collector| visit::visit_expr_const(collector, deferred));
    }

    fn visit_expr_if(&mut self, conditional: &'ast syn::ExprIf) {
        self.visit_expr(&conditional.cond);
        match literal_boolean(&conditional.cond) {
            Some(true) => {
                self.visit_block(&conditional.then_branch);
                if let Some((_, alternative)) = &conditional.else_branch {
                    self.without_positive_proof(|collector| collector.visit_expr(alternative));
                }
            }
            Some(false) => {
                self.without_positive_proof(|collector| {
                    collector.visit_block(&conditional.then_branch)
                });
                if let Some((_, alternative)) = &conditional.else_branch {
                    self.visit_expr(alternative);
                }
            }
            None => {
                self.visit_block(&conditional.then_branch);
                if let Some((_, alternative)) = &conditional.else_branch {
                    self.visit_expr(alternative);
                }
            }
        }
    }

    fn visit_expr_match(&mut self, expression_match: &'ast syn::ExprMatch) {
        self.visit_expr(&expression_match.expr);
        let scrutinee = literal_boolean(&expression_match.expr);
        for arm in &expression_match.arms {
            self.visit_pat(&arm.pat);
            if let Some((_, guard)) = &arm.guard {
                self.visit_expr(guard);
            }
            let unreachable = arm
                .guard
                .as_ref()
                .is_some_and(|(_, guard)| literal_boolean(guard) == Some(false))
                || scrutinee.is_some_and(|value| {
                    pattern_literal_boolean(&arm.pat).is_some_and(|pattern| pattern != value)
                });
            if unreachable {
                self.without_positive_proof(|collector| collector.visit_expr(&arm.body));
            } else {
                self.visit_expr(&arm.body);
            }
        }
    }

    fn visit_macro(&mut self, expression_macro: &'ast syn::Macro) {
        self.record_macro_identifiers(expression_macro.tokens.clone());
        visit::visit_macro(self, expression_macro);
    }

    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if let Some(segment) = path.path.segments.last() {
            self.record(format!("path:{}", segment.ident));
        }
        visit::visit_expr_path(self, path);
    }

    fn visit_pat_tuple_struct(&mut self, pattern: &'ast syn::PatTupleStruct) {
        if let Some(segment) = pattern.path.segments.last() {
            self.record(format!("path:{}", segment.ident));
        }
        visit::visit_pat_tuple_struct(self, pattern);
    }

    fn visit_stmt(&mut self, statement: &'ast Stmt) {
        if matches!(statement, Stmt::Item(_)) {
            self.without_positive_proof(|collector| visit::visit_stmt(collector, statement));
        } else {
            visit::visit_stmt(self, statement);
        }
    }
}

fn literal_boolean(expression: &Expr) -> Option<bool> {
    match expression {
        Expr::Lit(literal) => match &literal.lit {
            syn::Lit::Bool(value) => Some(value.value),
            _ => None,
        },
        _ => None,
    }
}

fn pattern_literal_boolean(pattern: &Pat) -> Option<bool> {
    match pattern {
        Pat::Lit(literal) => match &literal.lit {
            syn::Lit::Bool(value) => Some(value.value),
            _ => None,
        },
        _ => None,
    }
}
