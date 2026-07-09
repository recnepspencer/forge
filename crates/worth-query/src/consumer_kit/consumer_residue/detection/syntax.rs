use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::consumer_kit::boundary_audit::WorthQueryBoundaryAuditError;

use super::super::finding::{
    WorthQueryConsumerResidueFinding, WorthQueryConsumerResidueSourceSite,
};
use super::super::registry::WorthQueryConsumerResidueClass;
use super::syntax_context::{
    expr_contains_proof_like_path, expr_contains_support_matrix_context, is_proof_like_name,
    is_query_proof_name, is_query_report_name, member_is_proof_like, pat_is_proof_like,
    path_contains_ident,
};
use super::{class_filter_allows, rust_parse_error};

pub(crate) struct WorthQueryConsumerResidueSyntaxClassification {
    pub(crate) findings: Vec<WorthQueryConsumerResidueFinding>,
    pub(crate) parsed_item_count: usize,
    pub(crate) visited_node_count: usize,
}

pub(crate) fn classify_consumer_residue_syntax(
    source_label: &str,
    source_path: &str,
    source: &str,
    is_query_owned: bool,
    class_filter: Option<&[WorthQueryConsumerResidueClass]>,
) -> Result<WorthQueryConsumerResidueSyntaxClassification, WorthQueryBoundaryAuditError> {
    let syntax = syn::parse_file(source).map_err(|error| rust_parse_error(source_label, error))?;
    let parsed_item_count = syntax.items.len();
    let mut visitor =
        ConsumerResidueVisitor::new(source_label, source_path, is_query_owned, class_filter);
    visitor.visit_file(&syntax);
    Ok(WorthQueryConsumerResidueSyntaxClassification {
        findings: visitor.findings,
        parsed_item_count,
        visited_node_count: visitor.visited_node_count,
    })
}

struct ConsumerResidueVisitor<'a> {
    source_label: &'a str,
    source_path: &'a str,
    is_query_owned: bool,
    class_filter: Option<&'a [WorthQueryConsumerResidueClass]>,
    findings: Vec<WorthQueryConsumerResidueFinding>,
    visited_node_count: usize,
    proof_like_function_stack: Vec<bool>,
}

impl<'a> ConsumerResidueVisitor<'a> {
    fn new(
        source_label: &'a str,
        source_path: &'a str,
        is_query_owned: bool,
        class_filter: Option<&'a [WorthQueryConsumerResidueClass]>,
    ) -> Self {
        Self {
            source_label,
            source_path,
            is_query_owned,
            class_filter,
            findings: Vec::new(),
            visited_node_count: 0,
            proof_like_function_stack: Vec::new(),
        }
    }

    fn record(
        &mut self,
        residue_class: WorthQueryConsumerResidueClass,
        span: Span,
        matched_pattern: impl Into<String>,
    ) {
        if !class_filter_allows(self.class_filter, residue_class) {
            return;
        }
        if self.is_query_owned && !residue_class.is_test_backend_residue() {
            return;
        }
        let (line, column) = source_location(span);
        self.findings
            .push(WorthQueryConsumerResidueFinding::discovered(
                WorthQueryConsumerResidueSourceSite::new(
                    self.source_label,
                    self.source_path,
                    line,
                    column,
                ),
                residue_class,
                matched_pattern,
            ));
    }

    fn visit_counted(&mut self) {
        self.visited_node_count += 1;
    }

    fn current_function_returns_proof_like(&self) -> bool {
        self.proof_like_function_stack
            .last()
            .copied()
            .unwrap_or(false)
    }

    fn record_proof_string_expr(&mut self, expr: &syn::Expr, target_is_proof_like: bool) {
        if is_debug_format_expr(expr)
            && (target_is_proof_like || expr_contains_proof_like_path(expr))
        {
            self.record(
                WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
                expr.span(),
                "format!(\"{:?}\", query/proof/support/receipt/evidence)",
            );
        }
        if is_delimiter_join_expr(expr)
            && (target_is_proof_like || expr_contains_proof_like_path(expr))
        {
            self.record(
                WorthQueryConsumerResidueClass::DelimiterJoinedQueryProof,
                expr.span(),
                ".join(\"||\")",
            );
        }
        if is_delimiter_format_expr(expr)
            && (target_is_proof_like || expr_contains_proof_like_path(expr))
        {
            self.record(
                WorthQueryConsumerResidueClass::DelimiterFormattedQueryProof,
                expr.span(),
                "format!(\"{}||{}\", ...)",
            );
        }
    }
}

impl<'ast> Visit<'ast> for ConsumerResidueVisitor<'_> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.visit_counted();
        self.proof_like_function_stack
            .push(is_proof_like_name(&node.sig.ident.to_string()));
        syn::visit::visit_item_fn(self, node);
        self.proof_like_function_stack.pop();
    }

    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.visit_counted();
        let name = node.ident.to_string();
        if is_query_report_name(&name) {
            self.record(
                WorthQueryConsumerResidueClass::LocalQueryReport,
                node.ident.span(),
                name.clone(),
            );
        }
        if is_query_proof_name(&name) {
            self.record(
                WorthQueryConsumerResidueClass::LocalQueryProof,
                node.ident.span(),
                name,
            );
        }
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
        self.visit_counted();
        if path_contains_ident(&node.path, "WorthQuerySupportSnapshotRow") {
            self.record(
                WorthQueryConsumerResidueClass::RawSupportSnapshotRow,
                node.path.span(),
                "WorthQuerySupportSnapshotRow",
            );
        }
        syn::visit::visit_type_path(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.visit_counted();
        if node.method == "row_for_family" && expr_contains_support_matrix_context(&node.receiver) {
            self.record(
                WorthQueryConsumerResidueClass::SupportMatrixRowSearch,
                node.method.span(),
                "row_for_family",
            );
        }
        if node.method == "join" && is_delimiter_join_call(node) {
            let target_context = expr_contains_proof_like_path(&node.receiver);
            if target_context {
                self.record(
                    WorthQueryConsumerResidueClass::DelimiterJoinedQueryProof,
                    node.method.span(),
                    ".join(\"||\")",
                );
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_local(&mut self, node: &'ast syn::Local) {
        self.visit_counted();
        if let Some(init) = node.init.as_ref() {
            self.record_proof_string_expr(&init.expr, pat_is_proof_like(&node.pat));
        }
        syn::visit::visit_local(self, node);
    }

    fn visit_expr_assign(&mut self, node: &'ast syn::ExprAssign) {
        self.visit_counted();
        let target_context = expr_contains_proof_like_path(&node.left);
        self.record_proof_string_expr(&node.right, target_context);
        syn::visit::visit_expr_assign(self, node);
    }

    fn visit_expr_return(&mut self, node: &'ast syn::ExprReturn) {
        self.visit_counted();
        if let Some(expr) = node.expr.as_ref() {
            self.record_proof_string_expr(
                expr,
                self.current_function_returns_proof_like() || expr_contains_proof_like_path(expr),
            );
        }
        syn::visit::visit_expr_return(self, node);
    }

    fn visit_field_value(&mut self, node: &'ast syn::FieldValue) {
        self.visit_counted();
        self.record_proof_string_expr(&node.expr, member_is_proof_like(&node.member));
        syn::visit::visit_field_value(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.visit_counted();
        syn::visit::visit_macro(self, node);
    }
}

fn is_debug_format_expr(expr: &syn::Expr) -> bool {
    macro_path_is(expr, "format") && macro_tokens_contain(expr, "{:?}")
}

fn is_delimiter_format_expr(expr: &syn::Expr) -> bool {
    macro_path_is(expr, "format") && macro_tokens_contain(expr, "||")
}

fn is_delimiter_join_expr(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::MethodCall(call) => is_delimiter_join_call(call),
        _ => false,
    }
}

fn is_delimiter_join_call(call: &syn::ExprMethodCall) -> bool {
    call.method == "join" && call.args.iter().any(expr_is_double_pipe_literal)
}

fn macro_path_is(expr: &syn::Expr, macro_name: &str) -> bool {
    match expr {
        syn::Expr::Macro(mac) => mac.mac.path.is_ident(macro_name),
        _ => false,
    }
}

fn macro_tokens_contain(expr: &syn::Expr, needle: &str) -> bool {
    match expr {
        syn::Expr::Macro(mac) => mac.mac.tokens.to_string().contains(needle),
        _ => false,
    }
}

fn expr_is_double_pipe_literal(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(value) => value.value() == "||",
            _ => false,
        },
        _ => false,
    }
}

fn source_location(span: Span) -> (usize, usize) {
    let start = span.start();
    (start.line, start.column + 1)
}
