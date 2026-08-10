use super::super::artifacts::S0NondeterministicMetadata;
use super::super::evidence::S0StableDigest;
use super::super::manifest::S0AuditInputManifest;
use super::phrase_finding::TerminologyPhraseFinding;
use super::phrase_policy::{
    allowed_use_basis, TerminologyAllowedUse, TerminologyAllowlistEntry, TERMINOLOGY_RISK_PHRASES,
};
use super::risk_report::TerminologyRiskReport;
use super::scan_scope::{TerminologyScanInputFile, TerminologyScanPlan};
use super::validation::{
    finding_row_id, path_is_under_scope, stable_digest, terminology_evidence_ref,
    TerminologyCleanupRejection,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

impl TerminologyRiskReport {
    pub fn scan(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        plan: &TerminologyScanPlan,
        manifest: &S0AuditInputManifest,
        inputs: &[TerminologyScanInputFile],
        allowlist: &[TerminologyAllowlistEntry],
    ) -> Result<Self, TerminologyCleanupRejection> {
        let manifest_paths = manifest
            .matched_files()
            .iter()
            .map(|file| file.path())
            .collect::<BTreeSet<_>>();
        let scope_paths = plan
            .scopes()
            .iter()
            .map(super::scan_scope::TerminologyScanScope::path)
            .collect::<BTreeSet<_>>();
        let allowlist_index = index_allowlist(allowlist)?;
        let rows = collect_rows(inputs, &manifest_paths, &scope_paths, &allowlist_index)?;
        let scan_digest = build_scan_digest(plan, allowlist, inputs, &rows)?;
        Self::new(
            source_revision,
            roadmap_parent_digest,
            generated_by,
            nondeterministic_metadata,
            rows,
            scan_digest,
        )
    }
}

fn index_allowlist<'a>(
    allowlist: &'a [TerminologyAllowlistEntry],
) -> Result<BTreeMap<(&'a str, u64, &'a str), TerminologyAllowedUse>, TerminologyCleanupRejection> {
    allowlist
        .iter()
        .try_fold(BTreeMap::new(), |mut index, entry| {
            let key = (
                entry.path.as_str(),
                entry.line_number,
                entry.phrase.as_str(),
            );
            if index.insert(key, entry.allowed_use.clone()).is_some() {
                return Err(TerminologyCleanupRejection::DuplicateAllowlistEntry);
            }
            Ok(index)
        })
}

fn collect_rows<'a>(
    inputs: &[TerminologyScanInputFile],
    manifest_paths: &BTreeSet<&str>,
    scope_paths: &BTreeSet<&str>,
    allowlist_index: &BTreeMap<(&'a str, u64, &'a str), TerminologyAllowedUse>,
) -> Result<Vec<TerminologyPhraseFinding>, TerminologyCleanupRejection> {
    let mut seen_inputs = BTreeSet::new();
    let mut rows = Vec::new();
    for input in inputs {
        validate_scan_input(input, &mut seen_inputs, scope_paths, manifest_paths)?;
        rows.extend(scan_input_lines(input, allowlist_index)?);
    }
    Ok(rows)
}

fn validate_scan_input<'a>(
    input: &'a TerminologyScanInputFile,
    seen_inputs: &mut BTreeSet<&'a str>,
    scope_paths: &BTreeSet<&str>,
    manifest_paths: &BTreeSet<&str>,
) -> Result<(), TerminologyCleanupRejection> {
    if !seen_inputs.insert(input.path()) {
        return Err(TerminologyCleanupRejection::DuplicateScanInput);
    }
    if !scope_paths
        .iter()
        .any(|scope| path_is_under_scope(input.path(), scope))
    {
        return Err(TerminologyCleanupRejection::InputOutsideDeclaredScanScope);
    }
    if !manifest_paths.contains(input.path()) {
        return Err(TerminologyCleanupRejection::InputOutsideManifest);
    }
    Ok(())
}

fn scan_input_lines<'a>(
    input: &TerminologyScanInputFile,
    allowlist_index: &BTreeMap<(&'a str, u64, &'a str), TerminologyAllowedUse>,
) -> Result<Vec<TerminologyPhraseFinding>, TerminologyCleanupRejection> {
    let mut rows = Vec::new();
    for (line_idx, line) in input.contents().lines().enumerate() {
        rows.extend(findings_for_line(
            input.path(),
            (line_idx + 1) as u64,
            line,
            allowlist_index,
        )?);
    }
    Ok(rows)
}

fn findings_for_line<'a>(
    path: &str,
    line_number: u64,
    line: &str,
    allowlist_index: &BTreeMap<(&'a str, u64, &'a str), TerminologyAllowedUse>,
) -> Result<Vec<TerminologyPhraseFinding>, TerminologyCleanupRejection> {
    let normalized_line = line.to_ascii_lowercase();
    let mut rows = Vec::new();
    for phrase in TERMINOLOGY_RISK_PHRASES {
        if !normalized_line.contains(phrase) {
            continue;
        }
        let allowed_use = allowlist_index
            .get(&(path, line_number, phrase))
            .cloned()
            .ok_or(TerminologyCleanupRejection::UnclassifiedPhraseFinding)?;
        let deferred_s_sequences = match &allowed_use {
            TerminologyAllowedUse::QualifiedPhysicalDebt { deferred_sequence } => {
                vec![deferred_sequence.clone()]
            }
            _ => Vec::new(),
        };
        let status = match allowed_use {
            TerminologyAllowedUse::OverclaimedPhysicalPosture => {
                super::super::artifacts::S0ArtifactRowStatus::Deferred
            }
            _ => super::super::artifacts::S0ArtifactRowStatus::Admitted,
        };
        rows.push(TerminologyPhraseFinding::new(
            finding_row_id(path, line_number, phrase)?,
            path,
            vec![terminology_evidence_ref(path, line_number, phrase)],
            deferred_s_sequences,
            status,
            "S.0 terminology risk finding.",
            phrase,
            line_number,
            line.trim(),
            allowed_use,
        )?);
    }
    Ok(rows)
}

fn build_scan_digest(
    plan: &TerminologyScanPlan,
    allowlist: &[TerminologyAllowlistEntry],
    inputs: &[TerminologyScanInputFile],
    rows: &[TerminologyPhraseFinding],
) -> Result<S0StableDigest, TerminologyCleanupRejection> {
    let mut scope_paths = plan
        .scopes()
        .iter()
        .map(|scope| scope.path().to_string())
        .collect::<Vec<_>>();
    scope_paths.sort();
    let mut allowlist_basis = allowlist
        .iter()
        .map(|entry| {
            (
                entry.path.clone(),
                entry.line_number,
                entry.phrase.clone(),
                allowed_use_basis(&entry.allowed_use),
            )
        })
        .collect::<Vec<_>>();
    allowlist_basis.sort();
    let mut input_basis = inputs
        .iter()
        .map(|input| (input.path().to_string(), input.contents().to_string()))
        .collect::<Vec<_>>();
    input_basis.sort();
    let mut row_basis = rows
        .iter()
        .map(|row| {
            (
                row.row_id().as_str().to_string(),
                row.subject_path_or_symbol().to_string(),
                row.line_number(),
                allowed_use_basis(row.allowed_use()),
            )
        })
        .collect::<Vec<_>>();
    row_basis.sort();
    stable_digest(&TerminologyScanDigestBasis {
        scopes: scope_paths,
        allowlist: allowlist_basis,
        inputs: input_basis,
        rows: row_basis,
    })
}

#[derive(Serialize)]
struct TerminologyScanDigestBasis {
    scopes: Vec<String>,
    allowlist: Vec<(String, u64, String, String)>,
    inputs: Vec<(String, String)>,
    rows: Vec<(String, String, u64, String)>,
}
