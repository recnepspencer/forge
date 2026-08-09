use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;

use sha2::{Digest, Sha256};

use super::super::documents::{
    read_repository_document, split_csv, QA_AUDITS, QA_SOURCE_MANIFESTS,
};
use super::super::repository_root;
use super::history_contract::{parse_audit_row, AuditRecord};

const LEGACY_BASIS: &str = "legacy-unreproducible-review-capture-sha256";
const LEGACY_CAPTURES: &[(&str, &str)] = &[
    (
        "/root/c8_phase1_critic",
        "08915feeceb9f011c3a0d768b367a7730cfb75744d5b4ed3aa3e34e5ee482a20",
    ),
    (
        "/root/c8_phase1_test_critic",
        "0042856bda3ba1597c1517710c07d5853dac201ad04defb0a525288d5a732cd2",
    ),
    (
        "/root/c8_phase1_test_closure_critic",
        "ed18a561c5be7a22d196162ff6445175e2ea4e63c4c1b7d6f88ffb844eac876e",
    ),
    (
        "/root/c8_phase1_final_critic",
        "9003dc6c6ae1164da36428d6786edb48dd6309f12517ac6204a0ffa54a42044d",
    ),
    (
        "/root/c8_phase1_postfix_closure_critic",
        "e91a678a07c78ac79b3f97397a55a9d3b0c3a4bce40faed6423f5b63665a5ba2",
    ),
];

pub(super) fn validate_source_manifests(audits: &BTreeSet<AuditRecord>) -> Result<(), String> {
    let document = read_repository_document(QA_SOURCE_MANIFESTS)?;
    validate_source_manifest_document(&document, audits)
}

fn validate_source_manifest_document(
    document: &str,
    audits: &BTreeSet<AuditRecord>,
) -> Result<(), String> {
    let mut lines = document.lines();
    if lines.next() != Some("reviewer,basis,identity") {
        return Err("C.8 QA source manifest has an invalid schema".into());
    }
    let mut manifests = BTreeMap::<String, BTreeSet<(String, String)>>::new();
    let mut row_count = 0;
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let columns = split_csv(line, 3)?;
        row_count += 1;
        manifests
            .entry(columns[0].into())
            .or_default()
            .insert((columns[1].into(), columns[2].into()));
    }
    if row_count != manifests.values().map(BTreeSet::len).sum::<usize>() {
        return Err("C.8 QA source manifest contains duplicate rows".into());
    }
    if manifests.len() != audits.len() {
        return Err("C.8 QA source manifest reviewer set is incomplete".into());
    }
    for audit in audits {
        validate_audit_manifest(audit, &manifests)?;
    }
    Ok(())
}

fn validate_audit_manifest(
    audit: &AuditRecord,
    manifests: &BTreeMap<String, BTreeSet<(String, String)>>,
) -> Result<(), String> {
    let rows = manifests
        .get(&audit.0)
        .ok_or_else(|| format!("C.8 audit {} has no source manifest", audit.0))?;
    let bases = rows
        .iter()
        .map(|(basis, _)| basis.as_str())
        .collect::<BTreeSet<_>>();
    let base_revision = identity_for(rows, "base-revision");
    let legacy = bases.contains(LEGACY_BASIS);
    let valid_sources = if legacy {
        legacy_capture_is_retained(rows, &bases, &audit.0)
    } else {
        current_file_manifest_is_valid(rows, &audit.0, &audit.3)
    };
    if base_revision.is_none_or(|identity| !audit.2.starts_with(identity))
        || !valid_sources
        || manifest_identity(&audit.0, rows) != audit.3
    {
        return Err(format!("C.8 audit {} source manifest diverged", audit.0));
    }
    Ok(())
}

fn legacy_capture_is_retained(
    rows: &BTreeSet<(String, String)>,
    bases: &BTreeSet<&str>,
    reviewer: &str,
) -> bool {
    let expected = LEGACY_CAPTURES
        .iter()
        .find_map(|(candidate, identity)| (*candidate == reviewer).then_some(*identity));
    bases == &BTreeSet::from(["base-revision", LEGACY_BASIS])
        && rows.len() == 2
        && identity_for(rows, LEGACY_BASIS) == expected
}

fn current_file_manifest_is_valid(
    rows: &BTreeSet<(String, String)>,
    reviewer: &str,
    snapshot: &str,
) -> bool {
    let manifested = rows
        .iter()
        .filter_map(|(basis, _)| basis.strip_prefix("file:"))
        .collect::<BTreeSet<_>>();
    let Some(base_revision) = identity_for(rows, "base-revision") else {
        return false;
    };
    let Ok(reviewed) = review_paths(base_revision) else {
        return false;
    };
    manifested == reviewed.iter().map(String::as_str).collect()
        && rows.iter().all(|(basis, identity)| {
            basis == "base-revision"
                || basis.strip_prefix("file:").is_some_and(|path| {
                    hex_sha256(identity)
                        && source_identity(path, reviewer, snapshot).as_deref() == Some(identity)
                })
        })
}

fn review_paths(base_revision: &str) -> Result<BTreeSet<String>, String> {
    let committed = Command::new("git")
        .args(["diff", "--name-only", &format!("{base_revision}...HEAD")])
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot enumerate committed C.8 review sources: {error}"))?;
    if !committed.status.success() {
        return Err("git diff failed while enumerating committed C.8 review sources".into());
    }
    let dirty = Command::new("git")
        .args(["status", "--porcelain=v1", "-uall"])
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot enumerate C.8 review sources: {error}"))?;
    if !dirty.status.success() {
        return Err("git status failed while enumerating C.8 review sources".into());
    }
    let mut paths = String::from_utf8(committed.stdout)
        .map_err(|error| format!("git diff returned non-UTF8 source paths: {error}"))?
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let dirty_paths = String::from_utf8(dirty.stdout)
        .map_err(|error| format!("git status returned non-UTF8 source paths: {error}"))?
        .lines()
        .map(|line| {
            line.get(3..)
                .map(str::to_owned)
                .ok_or_else(|| format!("malformed git status row `{line}`"))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    paths.extend(dirty_paths);
    Ok(paths)
}

fn source_identity(path: &str, reviewer: &str, snapshot: &str) -> Option<String> {
    let bytes = std::fs::read(repository_root().join(path)).ok()?;
    let canonical = canonical_source(path, bytes, reviewer, snapshot)?;
    Some(format!("{:x}", Sha256::digest(canonical)))
}

fn canonical_source(path: &str, bytes: Vec<u8>, reviewer: &str, snapshot: &str) -> Option<Vec<u8>> {
    let text = String::from_utf8(bytes).ok()?;
    let canonical = if path.ends_with("physical-reconstruction-c8-qa-source-manifests.csv") {
        normalize_manifest(&text, reviewer)
    } else if path.ends_with("physical-reconstruction-c8-qa-audits.csv") {
        normalize_audit(&text, reviewer)
    } else if path.ends_with("closure_ledger/history_contract.rs") {
        text.replace(snapshot, "<review-snapshot>")
    } else if path.ends_with("physical-reconstruction-c8-closure-ledger.md") {
        normalize_ledger(&text)
    } else {
        text
    };
    Some(canonical.into_bytes())
}

fn normalize_manifest(document: &str, reviewer: &str) -> String {
    document
        .lines()
        .map(|line| {
            let mut columns = line.split(',').collect::<Vec<_>>();
            if columns.first() == Some(&reviewer)
                && columns.get(1).is_some_and(|v| v.starts_with("file:"))
            {
                columns[2] = "<source-identity>";
            }
            columns.join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn normalize_audit(document: &str, reviewer: &str) -> String {
    document
        .lines()
        .map(|line| {
            let mut columns = line.split(',').collect::<Vec<_>>();
            if columns.first() == Some(&reviewer) && columns.len() == 8 {
                columns[3] = "<review-snapshot>";
            }
            columns.join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn normalize_ledger(document: &str) -> String {
    document
        .lines()
        .map(|line| {
            if !line.starts_with("| C8-P1-") || line.starts_with("| C8-P1-F") {
                return line.to_owned();
            }
            let mut columns = line.split('|').collect::<Vec<_>>();
            if columns.len() == 11 {
                columns[6] = " <source-identity> ";
            }
            columns.join("|")
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn hex_sha256(identity: &str) -> bool {
    identity.len() == 64 && identity.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn identity_for<'a>(rows: &'a BTreeSet<(String, String)>, basis: &str) -> Option<&'a str> {
    rows.iter()
        .find_map(|(candidate, identity)| (candidate == basis).then_some(identity.as_str()))
}

fn manifest_identity(reviewer: &str, rows: &BTreeSet<(String, String)>) -> String {
    let mut digest = Sha256::new();
    for (basis, identity) in rows {
        digest.update(reviewer.as_bytes());
        digest.update([0]);
        digest.update(basis.as_bytes());
        digest.update([0]);
        digest.update(identity.as_bytes());
        digest.update([0xff]);
    }
    format!("{:x}", digest.finalize())
}

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
}
