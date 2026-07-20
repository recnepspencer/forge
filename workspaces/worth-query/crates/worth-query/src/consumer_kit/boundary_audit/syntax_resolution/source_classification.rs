use syn::spanned::Spanned;
use syn::visit::Visit;

use super::super::error::{WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind};
use super::super::finding::{WorthQueryBoundaryAuditFinding, WorthQueryBoundaryAuditSyntaxClass};
use super::super::source_site::WorthQueryBoundaryAuditSourceSite;
use super::call_index::WorthQueryBoundaryAuditCallIndex;
use crate::WorthQueryProhibitedSeam;

pub(crate) struct WorthQueryBoundaryAuditSourceClassification {
    findings: Vec<WorthQueryBoundaryAuditFinding>,
    parsed_item_count: usize,
    visited_call_count: usize,
}

impl WorthQueryBoundaryAuditSourceClassification {
    pub(crate) fn into_parts(self) -> (Vec<WorthQueryBoundaryAuditFinding>, usize, usize) {
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
    call_index: &WorthQueryBoundaryAuditCallIndex,
) -> Result<WorthQueryBoundaryAuditSourceClassification, WorthQueryBoundaryAuditError> {
    let syntax = parse_boundary_audit_source(source_label, source)?;
    let parsed_item_count = syntax.items.len();
    let mut visitor = WorthQueryBoundaryAuditVisitor::new(source_label, source_path, call_index);
    visitor.visit_file(&syntax);

    Ok(WorthQueryBoundaryAuditSourceClassification {
        findings: visitor.findings,
        parsed_item_count,
        visited_call_count: visitor.visited_call_count,
    })
}

fn parse_boundary_audit_source(
    source_label: &str,
    source: &str,
) -> Result<syn::File, WorthQueryBoundaryAuditError> {
    syn::parse_file(source).map_err(|error| {
        WorthQueryBoundaryAuditError::for_source(
            WorthQueryBoundaryAuditErrorKind::RustParseFailed,
            source_label,
            format!("boundary audit source `{source_label}` did not parse as Rust: {error}"),
        )
    })
}

struct WorthQueryBoundaryAuditVisitor<'a> {
    source_label: String,
    source_path: Option<String>,
    call_index: &'a WorthQueryBoundaryAuditCallIndex,
    findings: Vec<WorthQueryBoundaryAuditFinding>,
    visited_call_count: usize,
}

impl<'a> WorthQueryBoundaryAuditVisitor<'a> {
    fn new(
        source_label: &str,
        source_path: Option<&str>,
        call_index: &'a WorthQueryBoundaryAuditCallIndex,
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
        seam: Option<WorthQueryProhibitedSeam>,
        syntax_class: WorthQueryBoundaryAuditSyntaxClass,
        line: usize,
        column: usize,
    ) {
        self.visited_call_count += 1;
        if let Some(seam) = seam {
            self.findings
                .push(WorthQueryBoundaryAuditFinding::prohibited_seam_usage(
                    seam,
                    WorthQueryBoundaryAuditSourceSite::new(
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

impl<'ast> Visit<'ast> for WorthQueryBoundaryAuditVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let (line, column) = source_location(node.method.span());
        self.record_call(
            self.call_index
                .seam_for_method_name(node.method.to_string().as_str()),
            WorthQueryBoundaryAuditSyntaxClass::MethodCall,
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
                WorthQueryBoundaryAuditSyntaxClass::AssociatedPathCall,
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
