mod documentation_audit;
mod evidence_manifest;
mod source_audit;

use std::collections::BTreeSet;
use std::path::Path;

use crate::consumer_kit::native_value_authority_inventory::{
    audit_native_value_grammar, current_native_value_authority_audit,
    worth_query_native_value_authority_rows, worth_query_native_value_grammar,
};
use crate::identity::hash_parts;
use documentation_audit::audit_native_value_documentation;
use source_audit::audit_native_value_consumers;

pub use evidence_manifest::{
    worth_query_milestone_nine_thirteen_native_value_evidence_rows,
    WorthQueryMilestoneNineThirteenNativeValueEvidenceRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind {
    EvidenceSourceReadFailed,
    EvidenceProbeDrift,
    AuthorityAuditReadFailed,
    ConsumerAuditReadFailed,
    DocumentationAuditReadFailed,
    CompileFailManifestReadFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineThirteenNativeValueCertificationError {
    kind: WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind,
    path: Option<String>,
    message: String,
}

impl WorthQueryMilestoneNineThirteenNativeValueCertificationError {
    fn new(
        kind: WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind,
        path: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            path,
            message: message.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind {
        self.kind
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for WorthQueryMilestoneNineThirteenNativeValueCertificationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for WorthQueryMilestoneNineThirteenNativeValueCertificationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineThirteenNativeValueCertificationBundle {
    evidence_digest: String,
    native_authority_digest: String,
    consumer_source_digest: String,
    documentation_digest: String,
    certification_digest: String,
    authority_finding_count: usize,
    grammar_gap_count: usize,
    consumer_residue_count: usize,
    documentation_disagreement_count: usize,
    phase_manifest_gap_count: usize,
    native_family_count: usize,
    compile_fail_fixture_count: usize,
}

impl WorthQueryMilestoneNineThirteenNativeValueCertificationBundle {
    pub fn is_closed(&self) -> bool {
        self.authority_finding_count == 0
            && self.grammar_gap_count == 0
            && self.consumer_residue_count == 0
            && self.documentation_disagreement_count == 0
            && self.phase_manifest_gap_count == 0
            && self.native_family_count == 26
            && self.compile_fail_fixture_count >= 216
            && !self.evidence_digest.is_empty()
            && !self.native_authority_digest.is_empty()
            && !self.consumer_source_digest.is_empty()
            && !self.documentation_digest.is_empty()
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn native_authority_digest(&self) -> &str {
        &self.native_authority_digest
    }

    pub fn consumer_source_digest(&self) -> &str {
        &self.consumer_source_digest
    }

    pub fn documentation_digest(&self) -> &str {
        &self.documentation_digest
    }

    pub fn authority_finding_count(&self) -> usize {
        self.authority_finding_count
    }

    pub fn grammar_gap_count(&self) -> usize {
        self.grammar_gap_count
    }

    pub fn consumer_residue_count(&self) -> usize {
        self.consumer_residue_count
    }

    pub fn documentation_disagreement_count(&self) -> usize {
        self.documentation_disagreement_count
    }

    pub fn phase_manifest_gap_count(&self) -> usize {
        self.phase_manifest_gap_count
    }

    pub fn native_family_count(&self) -> usize {
        self.native_family_count
    }

    pub fn compile_fail_fixture_count(&self) -> usize {
        self.compile_fail_fixture_count
    }
}

pub fn certify_milestone_nine_thirteen_native_values(
    repository_root: impl AsRef<Path>,
) -> Result<
    WorthQueryMilestoneNineThirteenNativeValueCertificationBundle,
    WorthQueryMilestoneNineThirteenNativeValueCertificationError,
> {
    let repository_root = repository_root.as_ref();
    let evidence_digest = source_backed_evidence_digest(repository_root)?;
    let phase_manifest_gap_count = phase_manifest_gap_count();
    let compile_fail_fixture_count = compile_fail_fixture_count(repository_root)?;

    let authority_audit = current_native_value_authority_audit().map_err(|error| {
        WorthQueryMilestoneNineThirteenNativeValueCertificationError::new(
            WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind::AuthorityAuditReadFailed,
            Some("crates/worth-query/src".to_string()),
            format!("native-value authority audit failed: {error}"),
        )
    })?;
    let grammar_audit = audit_native_value_grammar(worth_query_native_value_grammar());
    let authority_finding_count = authority_audit.findings().len();
    let grammar_gap_count = grammar_audit.missing_scalar_types().len()
        + grammar_audit.duplicate_families().len()
        + grammar_audit.missing_cell_families().len()
        + grammar_audit.struct_row_count().abs_diff(1);
    let native_family_count = worth_query_native_value_grammar().len();
    let native_authority_digest = native_authority_digest();

    let consumer_audit = audit_native_value_consumers(repository_root).map_err(|error| {
        WorthQueryMilestoneNineThirteenNativeValueCertificationError::new(
            WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind::ConsumerAuditReadFailed,
            None,
            format!("native-value consumer audit failed: {error}"),
        )
    })?;
    let documentation_audit =
        audit_native_value_documentation(repository_root).map_err(|error| {
            WorthQueryMilestoneNineThirteenNativeValueCertificationError::new(
                WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind::DocumentationAuditReadFailed,
                None,
                format!("native-value documentation audit failed: {error}"),
            )
        })?;
    let consumer_residue_count = consumer_audit.findings.len();
    let documentation_disagreement_count = documentation_audit.disagreements.len();
    let certification_digest = hash_parts(&[
        evidence_digest.clone(),
        native_authority_digest.clone(),
        consumer_audit.source_digest.clone(),
        documentation_audit.source_digest.clone(),
        format!("authority_findings:{authority_finding_count}"),
        format!("grammar_gaps:{grammar_gap_count}"),
        format!("consumer_residue:{consumer_residue_count}"),
        format!("documentation_disagreements:{documentation_disagreement_count}"),
        format!("phase_manifest_gaps:{phase_manifest_gap_count}"),
        format!("native_families:{native_family_count}"),
        format!("compile_fail_fixtures:{compile_fail_fixture_count}"),
    ]);

    Ok(
        WorthQueryMilestoneNineThirteenNativeValueCertificationBundle {
            evidence_digest,
            native_authority_digest,
            consumer_source_digest: consumer_audit.source_digest,
            documentation_digest: documentation_audit.source_digest,
            certification_digest,
            authority_finding_count,
            grammar_gap_count,
            consumer_residue_count,
            documentation_disagreement_count,
            phase_manifest_gap_count,
            native_family_count,
            compile_fail_fixture_count,
        },
    )
}

fn source_backed_evidence_digest(
    repository_root: &Path,
) -> Result<String, WorthQueryMilestoneNineThirteenNativeValueCertificationError> {
    let mut parts = Vec::new();
    for row in worth_query_milestone_nine_thirteen_native_value_evidence_rows() {
        let path = repository_root.join(row.path());
        let source = std::fs::read_to_string(&path).map_err(|error| {
            WorthQueryMilestoneNineThirteenNativeValueCertificationError::new(
                WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind::EvidenceSourceReadFailed,
                Some(path.display().to_string()),
                format!("failed to read native-value evidence source: {error}"),
            )
        })?;
        let probe_count = source.match_indices(row.probe()).count();
        if probe_count != 1 {
            return Err(WorthQueryMilestoneNineThirteenNativeValueCertificationError::new(
                WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind::EvidenceProbeDrift,
                Some(path.display().to_string()),
                format!(
                    "phase {} evidence probe must occur exactly once, found {probe_count}: {}",
                    row.phase(),
                    row.probe()
                ),
            ));
        }
        parts.push(format!(
            "{}:{}:{}:{source}",
            row.phase(),
            row.path(),
            row.probe()
        ));
    }
    Ok(hash_parts(&parts))
}

fn phase_manifest_gap_count() -> usize {
    let expected = (21..=30).collect::<BTreeSet<_>>();
    let observed = worth_query_milestone_nine_thirteen_native_value_evidence_rows()
        .iter()
        .map(|row| row.phase())
        .collect::<BTreeSet<_>>();
    expected.symmetric_difference(&observed).count()
}

fn compile_fail_fixture_count(
    repository_root: &Path,
) -> Result<usize, WorthQueryMilestoneNineThirteenNativeValueCertificationError> {
    let path = repository_root.join("crates/worth-query/tests/ui/aspect_native_query");
    let entries = std::fs::read_dir(&path).map_err(|error| {
        WorthQueryMilestoneNineThirteenNativeValueCertificationError::new(
            WorthQueryMilestoneNineThirteenNativeValueCertificationErrorKind::CompileFailManifestReadFailed,
            Some(path.display().to_string()),
            format!("failed to read native-value compile-fail manifest: {error}"),
        )
    })?;
    Ok(entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|value| value.to_str()) == Some("rs"))
        .count())
}

fn native_authority_digest() -> String {
    let mut parts = worth_query_native_value_authority_rows()
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}:{}:{:?}:{:?}:{}",
                row.symbol(),
                row.defining_path(),
                row.exporting_paths().join(","),
                row.consumer_surfaces().join(","),
                row.class(),
                row.disposition(),
                row.closure_owner(),
            )
        })
        .collect::<Vec<_>>();
    parts.extend(worth_query_native_value_grammar().iter().map(|row| {
        format!(
            "{:?}:{:?}:{}:{}:{:?}:{}:{}:{}",
            row.family(),
            row.scalar_type(),
            row.semantic_carrier(),
            row.authoring_path(),
            row.predicate_capabilities(),
            row.projection_form(),
            row.refinement_form(),
            row.certification_owner(),
        )
    }));
    hash_parts(&parts)
}
