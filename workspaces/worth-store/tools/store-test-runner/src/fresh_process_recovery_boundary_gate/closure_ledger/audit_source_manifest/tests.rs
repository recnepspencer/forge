use std::collections::BTreeSet;
use std::process::Command;

use super::{base_is_ancestor, commit_exists, exact_commit_oid, validate_source_manifest_document};
use crate::fresh_process_recovery_boundary_gate::closure_ledger::history_contract::parse_audit_row;
use crate::fresh_process_recovery_boundary_gate::documents::{
    read_repository_document, QA_AUDITS, QA_SOURCE_MANIFESTS,
};
use crate::fresh_process_recovery_boundary_gate::repository_root;

#[test]
fn duplicate_and_substituted_source_manifests_are_rejected() {
    let audits = read_repository_document(QA_AUDITS).expect("read audits");
    let records = audits
        .lines()
        .skip(1)
        .map(parse_audit_row)
        .collect::<Result<BTreeSet<_>, _>>()
        .expect("parse audits");
    let manifest = read_repository_document(QA_SOURCE_MANIFESTS).expect("read source manifests");
    let first = manifest.lines().nth(1).expect("manifest row");
    let duplicate = manifest.replacen(first, &format!("{first}\n{first}"), 1);
    assert!(validate_source_manifest_document(&duplicate, &records).is_err());
    let substituted = manifest.replacen(
        "08915feeceb9f011c3a0d768b367a7730cfb75744d5b4ed3aa3e34e5ee482a20",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        1,
    );
    assert!(validate_source_manifest_document(&substituted, &records).is_err());
    let omitted = manifest
        .lines()
        .filter(|line| {
            !line.contains("file:_docs/worth-store/physical-reconstruction-c8-closure-ledger.md")
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert!(validate_source_manifest_document(&omitted, &records).is_err());
    assert_symbolic_and_non_commit_anchors_fail(&manifest, &records);
    let duplicate_basis = format!(
        "{manifest}/root/c8_phase1_absolute_final_critic,reviewed-revision,c960d5593a23d8a0d09ad5cc795e9e605f55e250\n"
    );
    assert!(validate_source_manifest_document(&duplicate_basis, &records).is_err());
}

fn assert_symbolic_and_non_commit_anchors_fail(
    manifest: &str,
    records: &BTreeSet<super::AuditRecord>,
) {
    let reviewed = "reviewed-revision,de2277376e0452da821b4dd360c2284b6128bb4a";
    let symbolic = manifest.replace(reviewed, "reviewed-revision,HEAD");
    assert!(validate_source_manifest_document(&symbolic, records).is_err());
    let blob_identity = reviewed_blob_identity();
    assert!(exact_commit_oid(&blob_identity) && !commit_exists(&blob_identity));
    let non_commit = manifest.replace(reviewed, &format!("reviewed-revision,{blob_identity}"));
    assert!(validate_source_manifest_document(&non_commit, records).is_err());
    assert!(!base_is_ancestor(
        "de2277376e0452da821b4dd360c2284b6128bb4a",
        "c960d5593a23d8a0d09ad5cc795e9e605f55e250"
    ));
}

fn reviewed_blob_identity() -> String {
    let output = Command::new("git")
        .args([
            "rev-parse",
            "de2277376e0452da821b4dd360c2284b6128bb4a:_docs/worth-store/physical-reconstruction-c8-public-api.csv",
        ])
        .current_dir(repository_root())
        .output()
        .expect("resolve reviewed blob");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("blob identity is UTF-8")
        .trim()
        .to_owned()
}
