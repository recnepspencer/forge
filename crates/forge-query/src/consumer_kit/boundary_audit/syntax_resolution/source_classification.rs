use syn::spanned::Spanned;
use syn::visit::Visit;

use super::super::error::{ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind};
use super::super::finding::{ForgeQueryBoundaryAuditFinding, ForgeQueryBoundaryAuditSyntaxClass};
use super::super::source_site::ForgeQueryBoundaryAuditSourceSite;
use super::call_index::ForgeQueryBoundaryAuditCallIndex;
use crate::ForgeQueryProhibitedSeam;

pub(crate) struct ForgeQueryBoundaryAuditSourceClassification {
    findings: Vec<ForgeQueryBoundaryAuditFinding>,
    parsed_item_count: usize,
    visited_call_count: usize,
}

impl ForgeQueryBoundaryAuditSourceClassification {
    pub(crate) fn into_parts(self) -> (Vec<ForgeQueryBoundaryAuditFinding>, usize, usize) {
        (
            self.findings,
            self.parsed_item_count,
            self.visited_call_count,
        )
    }
}

pub(crate) fn classify_boundary_audit_source(
    source_label: &str,
    source_path: Option<&str>,
    source: &str,
    call_index: &ForgeQueryBoundaryAuditCallIndex,
) -> Result<ForgeQueryBoundaryAuditSourceClassification, ForgeQueryBoundaryAuditError> {
    let syntax = parse_boundary_audit_source(source_label, source)?;
    let parsed_item_count = syntax.items.len();
    let mut visitor = ForgeQueryBoundaryAuditVisitor::new(source_label, source_path, call_index);
    visitor.visit_file(&syntax);

    Ok(ForgeQueryBoundaryAuditSourceClassification {
        findings: visitor.findings,
        parsed_item_count,
        visited_call_count: visitor.visited_call_count,
    })
}

fn parse_boundary_audit_source(
    source_label: &str,
    source: &str,
) -> Result<syn::File, ForgeQueryBoundaryAuditError> {
    syn::parse_file(source).map_err(|error| {
        ForgeQueryBoundaryAuditError::for_source(
            ForgeQueryBoundaryAuditErrorKind::RustParseFailed,
            source_label,
            format!("boundary audit source `{source_label}` did not parse as Rust: {error}"),
        )
    })
}

struct ForgeQueryBoundaryAuditVisitor<'a> {
    source_label: String,
    source_path: Option<String>,
    call_index: &'a ForgeQueryBoundaryAuditCallIndex,
    findings: Vec<ForgeQueryBoundaryAuditFinding>,
    visited_call_count: usize,
}

impl<'a> ForgeQueryBoundaryAuditVisitor<'a> {
    fn new(
        source_label: &str,
        source_path: Option<&str>,
        call_index: &'a ForgeQueryBoundaryAuditCallIndex,
    ) -> Self {
        Self {
            source_label: source_label.to_string(),
            source_path: source_path.map(str::to_string),
            call_index,
            findings: Vec::new(),
            visited_call_count: 0,
        }
    }

    fn record_call(
        &mut self,
        seam: Option<ForgeQueryProhibitedSeam>,
        syntax_class: ForgeQueryBoundaryAuditSyntaxClass,
        line: usize,
        column: usize,
    ) {
        self.visited_call_count += 1;
        if let Some(seam) = seam {
            self.findings
                .push(ForgeQueryBoundaryAuditFinding::prohibited_seam_usage(
                    seam,
                    ForgeQueryBoundaryAuditSourceSite::new(
                        self.source_label.clone(),
                        self.source_path.as_deref(),
                        line,
                        column,
                    ),
                    syntax_class,
                ));
        }
    }
}

impl<'ast> Visit<'ast> for ForgeQueryBoundaryAuditVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let (line, column) = source_location(node.method.span());
        self.record_call(
            self.call_index
                .seam_for_method_name(node.method.to_string().as_str()),
            ForgeQueryBoundaryAuditSyntaxClass::MethodCall,
            line,
            column,
        );
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = node.func.as_ref() {
            let (line, column) = source_location(path.span());
            self.record_call(
                self.call_index.seam_for_associated_path(path),
                ForgeQueryBoundaryAuditSyntaxClass::AssociatedPathCall,
                line,
                column,
            );
        }
        syn::visit::visit_expr_call(self, node);
    }
}

fn source_location(span: proc_macro2::Span) -> (usize, usize) {
    let start = span.start();
    (start.line, start.column + 1)
}
