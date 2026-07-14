use std::collections::BTreeSet;

use crate::identity::hash_parts;

use super::{
    worth_query_product_boundary_evidence_rows, WorthQueryProductBoundaryCertificationBundle,
    WorthQueryProductBoundaryCertificationError, WorthQueryProductBoundaryEvidenceKind as Kind,
};
use crate::consumer_kit::{
    current_capability_grammar_audit, current_declarative_surface_audit,
    current_ordinary_api_snapshot_audit, hard_prohibition_boundary_audit_coverage,
    hard_prohibition_registry, worth_query_capability_grammar,
    worth_query_consumer_residue_certification_evidence,
    worth_query_reference_consumer_adoption_rows, worth_query_reference_consumer_deleted_residue,
};

pub fn certify_declarative_product_boundary(
) -> Result<WorthQueryProductBoundaryCertificationBundle, WorthQueryProductBoundaryCertificationError>
{
    let mut findings = Vec::new();
    check_core_audits(&mut findings);
    check_evidence_rows(&mut findings);
    check_reference_consumers(&mut findings);
    if !findings.is_empty() {
        return Err(WorthQueryProductBoundaryCertificationError::new(findings));
    }

    let components = component_digests();
    let closure_digest = hash_parts(
        &components
            .iter()
            .map(|(name, digest)| format!("{name}:{digest}"))
            .collect::<Vec<_>>(),
    );
    let rows = worth_query_product_boundary_evidence_rows();
    Ok(WorthQueryProductBoundaryCertificationBundle::new(
        components,
        worth_query_capability_grammar().len(),
        rows.iter()
            .filter(|row| row.hostile_case().is_some())
            .count(),
        rows.iter()
            .filter(|row| row.sabotage_case().is_some())
            .count(),
        closure_digest,
    ))
}

fn check_core_audits(findings: &mut Vec<String>) {
    if !current_capability_grammar_audit().is_complete() {
        findings.push("capability grammar audit is incomplete".to_string());
    }
    if !current_declarative_surface_audit().is_complete() {
        findings.push("declarative surface audit is incomplete".to_string());
    }
    if !current_ordinary_api_snapshot_audit().is_complete() {
        findings.push("ordinary facade snapshot drifted".to_string());
    }
    let registry = hard_prohibition_registry();
    let coverage = hard_prohibition_boundary_audit_coverage();
    if registry.rows().len() != coverage.rows().len()
        || registry
            .rows()
            .iter()
            .any(|row| coverage.row(row.seam()).is_none())
    {
        findings.push("hard prohibition coverage does not match its registry".to_string());
    }
    if worth_query_consumer_residue_certification_evidence()
        .iter()
        .any(|row| !row.satisfied())
    {
        findings.push("consumer residue hostile evidence is incomplete".to_string());
    }
}

fn check_evidence_rows(findings: &mut Vec<String>) {
    let rows = worth_query_product_boundary_evidence_rows();
    let ids = rows.iter().map(|row| row.id()).collect::<BTreeSet<_>>();
    if ids.len() != rows.len() {
        findings.push("product boundary evidence ids are not unique".to_string());
    }
    for row in rows {
        match evidence_source(row.source_path())
            .map(|source| source.match_indices(row.source_probe()).count())
        {
            Some(1) => {}
            Some(count) => {
                findings.push(format!("{} has {count} matching evidence probes", row.id()))
            }
            None => findings.push(format!("{} has no registered evidence source", row.id())),
        }
    }
    let hostile = rows
        .iter()
        .filter_map(|row| row.hostile_case())
        .collect::<BTreeSet<_>>();
    let sabotage = rows
        .iter()
        .filter_map(|row| row.sabotage_case())
        .collect::<BTreeSet<_>>();
    if hostile.len() != 9 {
        findings.push("hostile matrix does not cover all nine cases".to_string());
    }
    if sabotage.len() != 6 {
        findings.push("sabotage matrix does not cover all six cases".to_string());
    }
}

fn check_reference_consumers(findings: &mut Vec<String>) {
    let rows = worth_query_reference_consumer_adoption_rows();
    if rows.len() != 2
        || rows
            .iter()
            .any(|row| row.after().ceremony_count() >= row.before().ceremony_count())
    {
        findings.push("reference consumer DX cutover is incomplete".to_string());
    }
    let kinds = worth_query_reference_consumer_deleted_residue()
        .iter()
        .map(|row| row.kind())
        .collect::<BTreeSet<_>>();
    if kinds.len() != 5 {
        findings.push("reference consumer deletion evidence is incomplete".to_string());
    }
}

fn component_digests() -> Vec<(&'static str, String)> {
    vec![
        ("facade", facade_digest()),
        ("grammar", grammar_digest()),
        ("prohibition", prohibition_digest()),
        ("residue", residue_digest()),
        ("dx", dx_digest()),
        (
            "reference-consumer",
            evidence_digest(Kind::ReferenceConsumer),
        ),
        ("semantic-parity", evidence_digest(Kind::SemanticParity)),
        ("lifecycle", evidence_digest(Kind::Lifecycle)),
        ("bounded-work", evidence_digest(Kind::BoundedWork)),
        ("hostile", evidence_digest(Kind::HostileRuntime)),
        ("compile-boundary", evidence_digest(Kind::CompileBoundary)),
        ("sabotage", evidence_digest(Kind::Sabotage)),
    ]
}

fn facade_digest() -> String {
    hash_parts(
        &current_ordinary_api_snapshot_audit()
            .snapshots()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.namespace(),
                    row.symbol_count(),
                    row.symbol_digest()
                )
            })
            .collect::<Vec<_>>(),
    )
}
fn grammar_digest() -> String {
    hash_parts(
        &worth_query_capability_grammar()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    row.family().as_str(),
                    row.namespace(),
                    row.declare(),
                    row.refine(),
                    row.terminal(),
                    row.transcript_probe()
                )
            })
            .collect::<Vec<_>>(),
    )
}
fn prohibition_digest() -> String {
    hash_parts(
        &hard_prohibition_registry()
            .rows()
            .iter()
            .map(|row| format!("{}:{}", row.seam_key(), row.enforcement_tier().as_str()))
            .collect::<Vec<_>>(),
    )
}
fn residue_digest() -> String {
    hash_parts(
        &worth_query_consumer_residue_certification_evidence()
            .iter()
            .map(|row| row.case_digest().to_string())
            .collect::<Vec<_>>(),
    )
}
fn dx_digest() -> String {
    let mut parts = worth_query_capability_grammar()
        .iter()
        .map(|row| format!("{}:{}", row.family().as_str(), row.target().total()))
        .collect::<Vec<_>>();
    parts.extend(
        worth_query_reference_consumer_adoption_rows()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.consumer(),
                    row.before().ceremony_count(),
                    row.after().ceremony_count()
                )
            }),
    );
    hash_parts(&parts)
}
fn evidence_digest(kind: Kind) -> String {
    hash_parts(
        &worth_query_product_boundary_evidence_rows()
            .iter()
            .filter(|row| row.kind() == kind)
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.id(),
                    row.source_path(),
                    row.enforcement_layer()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn evidence_source(path: &str) -> Option<&'static str> {
    match path {
        "tests/declarative_product_boundary_certification/hostile_matrix.rs" => Some(include_str!(
            "../../../tests/declarative_product_boundary_certification/hostile_matrix.rs"
        )),
        "tests/declarative_product_boundary_certification/sabotage_matrix.rs" => {
            Some(include_str!(
                "../../../tests/declarative_product_boundary_certification/sabotage_matrix.rs"
            ))
        }
        "tests/declarative_product_boundary_certification/parity_bounded.rs" => Some(include_str!(
            "../../../tests/declarative_product_boundary_certification/parity_bounded.rs"
        )),
        "tests/declarative_product_boundary_compile_fail.rs" => Some(include_str!(
            "../../../tests/declarative_product_boundary_compile_fail.rs"
        )),
        _ => None,
    }
}
