use std::collections::BTreeMap;

use syn::visit::Visit;

use super::error::{
    WorthQueryEvidenceReportAdoptionError, WorthQueryEvidenceReportAdoptionErrorKind,
};
use super::finding::{
    WorthQueryEvidenceReportAdoptionFinding, WorthQueryEvidenceReportAdoptionFindingKind,
    WorthQueryEvidenceReportAdoptionSyntaxClass,
};
use super::source_set::WorthQueryEvidenceReportAdoptionResidueClassification;

const DIGEST_SYMBOLS: &[&str] = &[
    "digest_owned_parts",
    "digest_owned_parts_with_scope",
    "ConstructionDigestScope",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryEvidenceReportAdoptionResidueSite {
    symbol: String,
    syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass,
    line: usize,
    column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryEvidenceReportAdoptionSourceClassification {
    residue_sites: Vec<WorthQueryEvidenceReportAdoptionResidueSite>,
    parsed_item_count: usize,
}

impl WorthQueryEvidenceReportAdoptionSourceClassification {
    pub(crate) fn residue_sites(&self) -> &[WorthQueryEvidenceReportAdoptionResidueSite] {
        &self.residue_sites
    }

    pub(crate) fn parsed_item_count(&self) -> usize {
        self.parsed_item_count
    }

    pub(crate) fn into_findings(
        &self,
        source_label: &str,
        source_path: Option<&str>,
        classification: WorthQueryEvidenceReportAdoptionResidueClassification,
    ) -> Vec<WorthQueryEvidenceReportAdoptionFinding> {
        if classification.permits_digest_residue() {
            return Vec::new();
        }
        let kind = match classification {
            WorthQueryEvidenceReportAdoptionResidueClassification::CoveredQueryEvidenceAdoption => {
                WorthQueryEvidenceReportAdoptionFindingKind::CoveredSurfaceUsesWorthDigest
            }
            WorthQueryEvidenceReportAdoptionResidueClassification::Unclassified => {
                WorthQueryEvidenceReportAdoptionFindingKind::UnclassifiedWorthDigestResidue
            }
            WorthQueryEvidenceReportAdoptionResidueClassification::DefendedDomainArtifactIdentity => {
                unreachable!("defended residue is handled above")
            }
        };
        self.residue_sites
            .iter()
            .map(|site| {
                WorthQueryEvidenceReportAdoptionFinding::new(
                    kind,
                    source_label,
                    source_path,
                    site.symbol.clone(),
                    site.syntax_class,
                    classification,
                    site.line,
                    site.column,
                )
            })
            .collect()
    }

    pub(crate) fn symbol_usage_counts(&self) -> BTreeMap<String, usize> {
        let mut counts = BTreeMap::new();
        for site in &self.residue_sites {
            *counts.entry(site.symbol.clone()).or_insert(0) += 1;
        }
        counts
    }
}

pub(crate) fn classify_evidence_report_adoption_source(
    source_label: &str,
    source: &str,
) -> Result<
    WorthQueryEvidenceReportAdoptionSourceClassification,
    WorthQueryEvidenceReportAdoptionError,
> {
    let syntax = syn::parse_file(source).map_err(|error| {
        WorthQueryEvidenceReportAdoptionError::for_source(
            WorthQueryEvidenceReportAdoptionErrorKind::RustParseFailed,
            source_label,
            format!(
                "evidence report adoption source `{source_label}` did not parse as Rust: {error}"
            ),
        )
    })?;
    let parsed_item_count = syntax.items.len();
    let mut visitor = WorthQueryEvidenceReportAdoptionVisitor::default();
    visitor.visit_file(&syntax);
    Ok(WorthQueryEvidenceReportAdoptionSourceClassification {
        residue_sites: visitor.residue_sites,
        parsed_item_count,
    })
}

#[derive(Default)]
struct WorthQueryEvidenceReportAdoptionVisitor {
    residue_sites: Vec<WorthQueryEvidenceReportAdoptionResidueSite>,
}

impl WorthQueryEvidenceReportAdoptionVisitor {
    fn record(
        &mut self,
        symbol: &str,
        syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass,
        span: proc_macro2::Span,
    ) {
        let start = span.start();
        self.residue_sites
            .push(WorthQueryEvidenceReportAdoptionResidueSite {
                symbol: symbol.to_string(),
                syntax_class,
                line: start.line,
                column: start.column + 1,
            });
    }

    fn record_path_segments(
        &mut self,
        path: &syn::Path,
        syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass,
    ) {
        for segment in &path.segments {
            let name = segment.ident.to_string();
            if DIGEST_SYMBOLS.contains(&name.as_str()) {
                self.record(&name, syntax_class, segment.ident.span());
            }
        }
    }
}

impl<'ast> Visit<'ast> for WorthQueryEvidenceReportAdoptionVisitor {
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        self.record_path_segments(
            &node.path,
            WorthQueryEvidenceReportAdoptionSyntaxClass::PathReference,
        );
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        record_use_tree(&mut self.residue_sites, &node.tree);
        syn::visit::visit_item_use(self, node);
    }
}

fn record_use_tree(
    residue_sites: &mut Vec<WorthQueryEvidenceReportAdoptionResidueSite>,
    tree: &syn::UseTree,
) {
    match tree {
        syn::UseTree::Path(path) => {
            let name = path.ident.to_string();
            if DIGEST_SYMBOLS.contains(&name.as_str()) {
                let start = path.ident.span().start();
                residue_sites.push(WorthQueryEvidenceReportAdoptionResidueSite {
                    symbol: name,
                    syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass::UseImport,
                    line: start.line,
                    column: start.column + 1,
                });
            }
            record_use_tree(residue_sites, &path.tree);
        }
        syn::UseTree::Name(name) => {
            let symbol = name.ident.to_string();
            if DIGEST_SYMBOLS.contains(&symbol.as_str()) {
                let start = name.ident.span().start();
                residue_sites.push(WorthQueryEvidenceReportAdoptionResidueSite {
                    symbol,
                    syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass::UseImport,
                    line: start.line,
                    column: start.column + 1,
                });
            }
        }
        syn::UseTree::Rename(rename) => {
            let symbol = rename.ident.to_string();
            if DIGEST_SYMBOLS.contains(&symbol.as_str()) {
                let start = rename.ident.span().start();
                residue_sites.push(WorthQueryEvidenceReportAdoptionResidueSite {
                    symbol,
                    syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass::UseImport,
                    line: start.line,
                    column: start.column + 1,
                });
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                record_use_tree(residue_sites, item);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}
