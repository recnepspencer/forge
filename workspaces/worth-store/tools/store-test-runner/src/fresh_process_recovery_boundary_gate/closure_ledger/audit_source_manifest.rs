use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;

use sha2::{Digest, Sha256};

use super::super::documents::{read_repository_document, split_csv, QA_SOURCE_MANIFESTS};
use super::super::repository_root;
use super::history_contract::AuditRecord;

#[cfg(test)]
mod tests;

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
    let mut unique_bases = BTreeSet::new();
    let mut row_count = 0;
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let columns = split_csv(line, 3)?;
        if !unique_bases.insert((columns[0].to_owned(), columns[1].to_owned())) {
            return Err(format!(
                "C.8 QA source manifest repeats basis {} for {}",
                columns[1], columns[0]
            ));
        }
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
    let valid_revisions = base_revision.is_some_and(|base| {
        exact_commit_oid(base)
            && commit_exists(base)
            && (legacy
                || identity_for(rows, "reviewed-revision").is_some_and(|reviewed| {
                    exact_commit_oid(reviewed)
                        && commit_exists(reviewed)
                        && base != reviewed
                        && base_is_ancestor(base, reviewed)
                }))
    });
    let valid_sources = if legacy {
        legacy_capture_is_retained(rows, &bases, &audit.0)
    } else {
        committed_file_manifest_is_valid(rows, &audit.0, &audit.3)
    };
    let actual_identity = manifest_identity(&audit.0, rows);
    if !valid_revisions
        || base_revision.is_none_or(|identity| !audit.2.starts_with(identity))
        || !valid_sources
        || actual_identity != audit.3
    {
        return Err(format!(
            "C.8 audit {} source manifest diverged (sources={valid_sources} identity={actual_identity})",
            audit.0
        ));
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

fn committed_file_manifest_is_valid(
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
    let Some(reviewed_revision) = identity_for(rows, "reviewed-revision") else {
        return false;
    };
    let Ok(reviewed) = review_paths(base_revision, reviewed_revision) else {
        return false;
    };
    manifested == reviewed.iter().map(String::as_str).collect()
        && rows.iter().all(|(basis, identity)| {
            basis == "base-revision"
                || basis == "reviewed-revision"
                || basis.strip_prefix("file:").is_some_and(|path| {
                    hex_sha256(identity)
                        && committed_source_identity(path, reviewer, snapshot, reviewed_revision)
                            .as_deref()
                            == Some(identity)
                })
        })
}

fn review_paths(base_revision: &str, reviewed_revision: &str) -> Result<BTreeSet<String>, String> {
    let committed = Command::new("git")
        .args([
            "diff",
            "--name-only",
            &format!("{base_revision}...{reviewed_revision}"),
        ])
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot enumerate committed C.8 review sources: {error}"))?;
    if !committed.status.success() {
        return Err("git diff failed while enumerating committed C.8 review sources".into());
    }
    Ok(String::from_utf8(committed.stdout)
        .map_err(|error| format!("git diff returned non-UTF8 source paths: {error}"))?
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect::<BTreeSet<_>>())
}

fn committed_source_identity(
    path: &str,
    reviewer: &str,
    snapshot: &str,
    reviewed_revision: &str,
) -> Option<String> {
    let output = Command::new("git")
        .args(["show", &format!("{reviewed_revision}:{path}")])
        .current_dir(repository_root())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let bytes = output.stdout;
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
        normalize_history_contract(&text, reviewer, snapshot)
    } else if path.ends_with("physical-reconstruction-c8-closure-ledger.md") {
        normalize_ledger(&text)
    } else {
        text
    };
    Some(canonical.into_bytes())
}

fn normalize_history_contract(document: &str, reviewer: &str, snapshot: &str) -> String {
    if document.contains(snapshot) {
        return document.replace(snapshot, "<review-snapshot>");
    }
    let Some(reviewer_offset) = document.find(&format!("\"{reviewer}\"")) else {
        return document.to_owned();
    };
    let tail = &document[reviewer_offset..];
    let Some((snapshot_offset, candidate)) = quoted_hex_identity(tail) else {
        return document.to_owned();
    };
    let absolute_offset = reviewer_offset + snapshot_offset;
    let mut normalized = document.to_owned();
    normalized.replace_range(
        absolute_offset..absolute_offset + candidate.len(),
        "<review-snapshot>",
    );
    normalized
}

fn quoted_hex_identity(document: &str) -> Option<(usize, &str)> {
    let mut opening_quote = None;
    for (offset, character) in document.char_indices() {
        if character != '"' {
            continue;
        }
        if let Some(opening) = opening_quote.take() {
            let candidate = &document[opening + 1..offset];
            if hex_sha256(candidate) {
                return Some((opening + 1, candidate));
            }
        } else {
            opening_quote = Some(offset);
        }
    }
    None
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

fn exact_commit_oid(identity: &str) -> bool {
    identity.len() == 40 && identity.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn commit_exists(identity: &str) -> bool {
    Command::new("git")
        .args(["cat-file", "-t", identity])
        .current_dir(repository_root())
        .output()
        .is_ok_and(|output| {
            output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "commit"
        })
}

fn base_is_ancestor(base: &str, reviewed: &str) -> bool {
    git_succeeds(&["merge-base", "--is-ancestor", base, reviewed])
}

fn git_succeeds(arguments: &[&str]) -> bool {
    Command::new("git")
        .args(arguments)
        .current_dir(repository_root())
        .output()
        .is_ok_and(|output| output.status.success())
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
