mod exact_text;
mod source_text_mask;
mod syntax;
mod syntax_context;

use crate::consumer_kit::boundary_audit::{
    ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind,
};

use super::finding::ForgeQueryConsumerResidueFinding;
use super::registry::ForgeQueryConsumerResidueClass;
use syntax::classify_consumer_residue_syntax;

pub(crate) struct ForgeQueryConsumerResidueSourceClassification {
    pub(crate) findings: Vec<ForgeQueryConsumerResidueFinding>,
    pub(crate) parsed_item_count: usize,
    pub(crate) visited_node_count: usize,
}

pub(crate) fn scan_consumer_residue_source(
    source_label: &str,
    source_path: &str,
    source: &str,
    is_query_owned: bool,
    class_filter: Option<&[ForgeQueryConsumerResidueClass]>,
) -> Result<ForgeQueryConsumerResidueSourceClassification, ForgeQueryBoundaryAuditError> {
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
    Ok(ForgeQueryConsumerResidueSourceClassification {
        findings,
        parsed_item_count: syntax.parsed_item_count,
        visited_node_count: syntax.visited_node_count,
    })
}

pub(crate) fn class_filter_allows(
    class_filter: Option<&[ForgeQueryConsumerResidueClass]>,
    class: ForgeQueryConsumerResidueClass,
) -> bool {
    class_filter.is_none_or(|classes| classes.contains(&class))
}

pub(crate) fn rust_parse_error(
    source_label: &str,
    error: syn::Error,
) -> ForgeQueryBoundaryAuditError {
    ForgeQueryBoundaryAuditError::for_source(
        ForgeQueryBoundaryAuditErrorKind::RustParseFailed,
        source_label,
        format!("consumer residue source `{source_label}` did not parse as Rust: {error}"),
    )
}
