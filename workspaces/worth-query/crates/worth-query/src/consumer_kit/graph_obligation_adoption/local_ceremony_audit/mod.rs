mod source_text_mask;

use crate::WorthQueryBoundaryAuditSourceSet;

use super::kit_digest;
use source_text_mask::mask_comments_and_string_literals;

const LOCAL_CEREMONY_PATTERNS: [&str; 6] = [
    "WorthQueryGraphObligationRegistration::",
    "WorthQueryGraphObligationRegistrationCatalog::from_registrations",
    "WorthQueryGraphObligationIndex::from_catalog",
    "WorthQueryGraphObligationSupportMatrixRow::new",
    "WorthQueryGraphTouchSelector::",
    "select_graph_obligations_for_touch",
];

const LOCAL_LEGALITY_FOLKLORE_PATTERNS: [&str; 8] = [
    "InvariantPack",
    "invariant_pack",
    "manual_precheck",
    "manual_pre_check",
    "private_validator",
    "validator_dispatch",
    "phase_chain",
    "local_legality_graph",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationLocalCeremonyFinding {
    source_label: String,
    source_path: Option<String>,
    pattern: &'static str,
    line: usize,
    column: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationLocalCeremonyAudit {
    findings: Vec<WorthQueryGraphObligationLocalCeremonyFinding>,
    evaluated_source_count: usize,
    audited_source_labels: Vec<String>,
    audit_digest: String,
}

impl WorthQueryGraphObligationLocalCeremonyAudit {
    pub fn evaluate(sources: &WorthQueryBoundaryAuditSourceSet) -> Self {
        let findings = collect_local_ceremony_findings(sources);
        let evaluated_source_count = sources.sources().len();
        let audited_source_labels = sources
            .sources()
            .iter()
            .map(|source| source.label().to_string())
            .collect::<Vec<_>>();
        let evaluated_source_count_text = evaluated_source_count.to_string();
        let audit_digest = kit_digest(
            "graph-obligation-local-ceremony-audit",
            [evaluated_source_count_text.as_str()]
                .into_iter()
                .chain(audited_source_labels.iter().map(String::as_str))
                .chain(findings.iter().map(|finding| finding.pattern)),
        );
        Self {
            findings,
            evaluated_source_count,
            audited_source_labels,
            audit_digest,
        }
    }

    pub fn clean() -> Self {
        Self {
            findings: Vec::new(),
            evaluated_source_count: 0,
            audited_source_labels: Vec::new(),
            audit_digest: kit_digest("graph-obligation-local-ceremony-audit", ["clean"]),
        }
    }

    pub fn findings(&self) -> &[WorthQueryGraphObligationLocalCeremonyFinding] {
        &self.findings
    }

    pub fn has_no_local_ceremony(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn is_evaluated(&self) -> bool {
        self.evaluated_source_count > 0
    }

    pub fn evaluated_source_count(&self) -> usize {
        self.evaluated_source_count
    }

    pub fn audited_source_labels(&self) -> &[String] {
        &self.audited_source_labels
    }

    pub fn is_clean(&self) -> bool {
        self.has_no_local_ceremony()
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}

impl WorthQueryGraphObligationLocalCeremonyFinding {
    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    pub fn pattern(&self) -> &'static str {
        self.pattern
    }

    pub fn source_path(&self) -> Option<&str> {
        self.source_path.as_deref()
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

fn collect_local_ceremony_findings(
    sources: &WorthQueryBoundaryAuditSourceSet,
) -> Vec<WorthQueryGraphObligationLocalCeremonyFinding> {
    sources
        .sources()
        .iter()
        .flat_map(|source| {
            let searchable = mask_comments_and_string_literals(source.source());
            searchable
                .lines()
                .enumerate()
                .flat_map(|(line_index, line)| {
                    LOCAL_CEREMONY_PATTERNS
                        .into_iter()
                        .chain(LOCAL_LEGALITY_FOLKLORE_PATTERNS)
                        .filter_map(move |pattern| {
                            line.find(pattern).map(|column| {
                                WorthQueryGraphObligationLocalCeremonyFinding {
                                    source_label: source.label().to_string(),
                                    source_path: source.path().map(ToOwned::to_owned),
                                    pattern,
                                    line: line_index + 1,
                                    column: column + 1,
                                }
                            })
                        })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
