mod exact_text;
mod source_text_mask;
mod syntax;
mod syntax_context;

use crate::consumer_kit::boundary_audit::{
    WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind,
};

use super::finding::WorthQueryConsumerResidueFinding;
use super::registry::WorthQueryConsumerResidueClass;
use syntax::classify_consumer_residue_syntax;

pub(crate) struct WorthQueryConsumerResidueSourceClassification {
    pub(crate) findings: Vec<WorthQueryConsumerResidueFinding>,
    pub(crate) parsed_item_count: usize,
    pub(crate) visited_node_count: usize,
}

pub(crate) fn scan_consumer_residue_source(
    source_label: &str,
    source_path: &str,
    source: &str,
    is_query_owned: bool,
    class_filter: Option<&[WorthQueryConsumerResidueClass]>,
) -> Result<WorthQueryConsumerResidueSourceClassification, WorthQueryBoundaryAuditError> {
    let mut findings =
        exact_text::find_exact_text_residue(source_label, source_path, source, class_filter);
    let syntax = classify_consumer_residue_syntax(
        source_label,
        source_path,
        source,
        is_query_owned,
        class_filter,
    )?;
    findings.extend(syntax.findings);
    Ok(WorthQueryConsumerResidueSourceClassification {
        findings,
        parsed_item_count: syntax.parsed_item_count,
        visited_node_count: syntax.visited_node_count,
    })
}

pub(crate) fn class_filter_allows(
    class_filter: Option<&[WorthQueryConsumerResidueClass]>,
    class: WorthQueryConsumerResidueClass,
) -> bool {
    class_filter.is_none_or(|classes| classes.contains(&class))
}

pub(crate) fn rust_parse_error(
    source_label: &str,
    error: syn::Error,
) -> WorthQueryBoundaryAuditError {
    WorthQueryBoundaryAuditError::for_source(
        WorthQueryBoundaryAuditErrorKind::RustParseFailed,
        source_label,
        format!("consumer residue source `{source_label}` did not parse as Rust: {error}"),
    )
}
