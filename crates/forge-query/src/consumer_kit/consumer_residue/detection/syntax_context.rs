pub(crate) fn is_query_report_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("query") && lower.contains("report")
}

pub(crate) fn is_query_proof_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("query") && lower.contains("proof")
}

pub(crate) fn is_proof_like_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("query")
        || lower.contains("proof")
        || lower.contains("support")
        || lower.contains("receipt")
        || lower.contains("evidence")
}

pub(crate) fn path_contains_ident(path: &syn::Path, expected: &str) -> bool {
    path.segments
        .iter()
        .any(|segment| segment.ident == expected)
}

pub(crate) fn expr_contains_proof_like_path(expr: &syn::Expr) -> bool {
    let mut visitor = ProofLikeExprVisitor { found: false };
    syn::visit::Visit::visit_expr(&mut visitor, expr);
    visitor.found
}

pub(crate) fn expr_contains_support_matrix_context(expr: &syn::Expr) -> bool {
    let mut visitor = SupportMatrixExprVisitor { found: false };
    syn::visit::Visit::visit_expr(&mut visitor, expr);
    visitor.found
}

pub(crate) fn pat_is_proof_like(pat: &syn::Pat) -> bool {
    match pat {
        syn::Pat::Ident(ident) => is_proof_like_name(&ident.ident.to_string()),
        syn::Pat::Type(typed) => pat_is_proof_like(&typed.pat),
        syn::Pat::Tuple(tuple) => tuple.elems.iter().any(pat_is_proof_like),
        syn::Pat::Struct(strukt) => strukt
            .fields
            .iter()
            .any(|field| member_is_proof_like(&field.member) || pat_is_proof_like(&field.pat)),
        _ => false,
    }
}

pub(crate) fn member_is_proof_like(member: &syn::Member) -> bool {
    match member {
        syn::Member::Named(ident) => is_proof_like_name(&ident.to_string()),
        syn::Member::Unnamed(_) => false,
    }
}

struct ProofLikeExprVisitor {
    found: bool,
}

impl<'ast> syn::visit::Visit<'ast> for ProofLikeExprVisitor {
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node
            .path
            .segments
            .iter()
            .any(|segment| is_proof_like_name(&segment.ident.to_string()))
        {
            self.found = true;
        }
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_field(&mut self, node: &'ast syn::Field) {
        if node
            .ident
            .as_ref()
            .is_some_and(|ident| is_proof_like_name(&ident.to_string()))
        {
            self.found = true;
        }
        syn::visit::visit_field(self, node);
    }
}

struct SupportMatrixExprVisitor {
    found: bool,
}

impl<'ast> syn::visit::Visit<'ast> for SupportMatrixExprVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let method_name = node.method.to_string();
        if method_name.contains("support_matrix") || method_name == "runtime_support_matrix" {
            self.found = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node.path.segments.iter().any(|segment| {
            let name = segment.ident.to_string();
            name.contains("support_matrix") || name.contains("SupportMatrix")
        }) {
            self.found = true;
        }
        syn::visit::visit_expr_path(self, node);
    }
}
