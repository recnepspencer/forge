use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use toml::Value;

use super::ledger;

const REQUIRED_DOCUMENTS: [&str; 10] = [
    "workspaces/worth-ui/AI_README.md",
    "workspaces/worth-ui/README.md",
    "workspaces/worth-ui/docs/application-lifecycle.md",
    "workspaces/worth-ui/docs/architecture.md",
    "workspaces/worth-ui/docs/authored-composition.md",
    "workspaces/worth-ui/docs/inspection.md",
    "workspaces/worth-ui/docs/migration-3.10.1.md",
    "workspaces/worth-ui/docs/query-binding.md",
    "workspaces/worth-ui/docs/runtime-subsystems.md",
    "workspaces/worth-ui/docs/worth-ui-readme.md",
];
const REQUIRED_SUBSYSTEMS: [&str; 7] = [
    "application",
    "graph",
    "inspection",
    "mounting",
    "observation",
    "planning",
    "session",
];
const REQUIRED_INSERTIONS: [&str; 4] = ["3.11", "3.12", "3.17", "3.18"];

pub(super) fn audit(document: &Value, repository_root: &Path) -> Result<(), String> {
    validate_header(document)?;
    audit_documents(document, repository_root)?;
    audit_examples(document, repository_root)?;
    let phase4 = ledger::load(&repository_root.join(ledger::text(document, "phase4_authority")?))?;
    audit_subsystems(document, &phase4)?;
    let roadmap = read_text(repository_root, ledger::text(document, "roadmap")?)?;
    audit_insertions(document, &phase4, &roadmap)?;
    audit_proof_ledgers(document, repository_root)?;
    audit_completion_markers(document, repository_root)?;
    audit_commands(document)
}

fn validate_header(document: &Value) -> Result<(), String> {
    if ledger::text(document, "schema")? != "worth-ui.milestone-3.10.1.phase-8-closeout.v2"
        || ledger::text(document, "milestone")? != "3.10.1"
        || ledger::integer(document, "phase")? != 8
        || ledger::text(document, "status")? != "complete"
    {
        return Err("Phase 8 closeout header is invalid".to_owned());
    }
    Ok(())
}

fn audit_documents(document: &Value, repository_root: &Path) -> Result<(), String> {
    let rows = ledger::tables(document, "document")?;
    let observed = rows
        .iter()
        .map(|row| ledger::text(row, "path").map(str::to_owned))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let expected = REQUIRED_DOCUMENTS.map(str::to_owned).into_iter().collect();
    if observed != expected {
        return Err(format!(
            "Phase 8 document inventory differs: observed={observed:?}, expected={expected:?}"
        ));
    }
    let forbidden = ledger::strings(document, "forbidden_public_doc_fragments")?;
    for row in rows {
        let path = ledger::text(row, "path")?;
        let text = read_text(repository_root, path)?;
        require_headings(path, &text, &ledger::strings(row, "required_headings")?)?;
        let allows_removed = row
            .get("allows_removed_routes")
            .and_then(Value::as_bool)
            .ok_or_else(|| format!("Phase 8 document `{path}` lacks route posture"))?;
        reject_forbidden_fragments(path, &text, allows_removed, &forbidden)?;
    }
    Ok(())
}

fn audit_examples(document: &Value, repository_root: &Path) -> Result<(), String> {
    let rows = ledger::tables(document, "example")?;
    if rows.len() != 3 {
        return Err("Phase 8 must bind exactly three documented example families".to_owned());
    }
    let mut ids = BTreeSet::new();
    let mut exact_compile_and_run = 0;
    for row in rows {
        let id = ledger::text(row, "id")?;
        if !ids.insert(id) {
            return Err(format!("duplicate Phase 8 example `{id}`"));
        }
        let doc = read_text(repository_root, ledger::text(row, "document")?)?;
        let source = read_text(repository_root, ledger::text(row, "source")?)?;
        let marker = ledger::text(row, "marker")?;
        let witness = ledger::text(row, "witness")?;
        if !doc.contains(marker) || !source.contains(witness) {
            return Err(format!(
                "Phase 8 example `{id}` is not bound to its witness"
            ));
        }
        match ledger::text(row, "contract")? {
            "source-bound" => {}
            "exact-compile-and-run" => {
                exact_compile_and_run += 1;
                require_exact_fenced_source(id, &doc, marker, &source)?;
                require_source_witness(
                    repository_root,
                    ledger::text(row, "execution_source")?,
                    ledger::text(row, "execution_witness")?,
                )?;
                require_source_witness(
                    repository_root,
                    ledger::text(row, "suite_owner")?,
                    ledger::text(row, "suite_witness")?,
                )?;
            }
            contract => return Err(format!("Phase 8 example `{id}` has contract `{contract}`")),
        }
    }
    if exact_compile_and_run != 1 {
        return Err(format!(
            "Phase 8 requires one exact compile-and-run example; found {exact_compile_and_run}"
        ));
    }
    Ok(())
}

fn require_source_witness(repository_root: &Path, path: &str, witness: &str) -> Result<(), String> {
    if !read_text(repository_root, path)?.contains(witness) {
        return Err(format!(
            "Phase 8 execution owner `{path}` lacks witness `{witness}`"
        ));
    }
    Ok(())
}

pub(super) fn require_exact_fenced_source(
    id: &str,
    document: &str,
    marker: &str,
    source: &str,
) -> Result<(), String> {
    let after_marker = document
        .split_once(marker)
        .map(|(_, tail)| tail)
        .ok_or_else(|| format!("Phase 8 example `{id}` lacks marker `{marker}`"))?;
    let after_fence = after_marker
        .split_once("```rust")
        .map(|(_, tail)| tail)
        .ok_or_else(|| format!("Phase 8 example `{id}` lacks a Rust fence after its marker"))?;
    let documented = after_fence
        .split_once("```")
        .map(|(code, _)| code)
        .ok_or_else(|| format!("Phase 8 example `{id}` has an unclosed Rust fence"))?;
    let normalize = |text: &str| text.replace("\r\n", "\n");
    if normalize(documented).trim() != normalize(source).trim() {
        return Err(format!(
            "Phase 8 example `{id}` differs from its compiled source"
        ));
    }
    Ok(())
}

fn audit_subsystems(closeout: &Value, phase4: &Value) -> Result<(), String> {
    let closing = named_rows(ledger::tables(closeout, "subsystem")?, "name")?;
    let authority = named_rows(ledger::tables(phase4, "family")?, "name")?;
    require_exact_keys(&closing, &REQUIRED_SUBSYSTEMS, "subsystem")?;
    for name in REQUIRED_SUBSYSTEMS {
        let closing_row = closing[name];
        let authority_row = authority
            .get(name)
            .ok_or_else(|| format!("Phase 4 lacks subsystem `{name}`"))?;
        compare_text(closing_row, authority_row, name, "owner_file")?;
        compare_text(closing_row, authority_row, name, "owner_symbol")?;
        let closing_dependencies = ledger::strings(closing_row, "allowed_dependencies")?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let authority_dependencies = ledger::strings(authority_row, "allowed_family_dependencies")?
            .into_iter()
            .collect::<BTreeSet<_>>();
        if closing_dependencies != authority_dependencies {
            return Err(format!("Phase 8 subsystem `{name}` dependency map drifted"));
        }
    }
    Ok(())
}

fn audit_insertions(closeout: &Value, phase4: &Value, roadmap: &str) -> Result<(), String> {
    let closing = named_rows(ledger::tables(closeout, "future_insertion")?, "milestone")?;
    let authority = named_rows(ledger::tables(phase4, "future_insertion")?, "milestone")?;
    require_exact_keys(&closing, &REQUIRED_INSERTIONS, "future insertion")?;
    for milestone in REQUIRED_INSERTIONS {
        let closing_row = closing[milestone];
        let authority_row = authority
            .get(milestone)
            .ok_or_else(|| format!("Phase 4 lacks insertion `{milestone}`"))?;
        for field in [
            "roadmap_heading",
            "change",
            "owner_scope",
            "owner",
            "insertion",
            "forbidden_owner",
        ] {
            compare_text(closing_row, authority_row, milestone, field)?;
        }
        let heading = ledger::text(closing_row, "roadmap_heading")?;
        if !roadmap.lines().any(|line| line == heading) {
            return Err(format!(
                "Phase 8 `{milestone}` roadmap heading `{heading}` is stale"
            ));
        }
    }
    Ok(())
}

fn audit_proof_ledgers(document: &Value, repository_root: &Path) -> Result<(), String> {
    let rows = ledger::tables(document, "proof_ledger")?;
    let phases = rows
        .iter()
        .map(|row| ledger::integer(row, "phase"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if phases != (3_i64..=8).collect() {
        return Err(format!("Phase 8 proof-ledger phases differ: {phases:?}"));
    }
    for row in rows {
        let path = ledger::text(row, "path")?;
        audit_proof_ledger_text(path, &read_text(repository_root, path)?)?;
    }
    Ok(())
}

fn audit_completion_markers(document: &Value, repository_root: &Path) -> Result<(), String> {
    require_marker(
        "roadmap",
        &read_text(repository_root, ledger::text(document, "roadmap")?)?,
        ledger::text(document, "roadmap_marker")?,
    )?;
    require_marker(
        "milestone spec",
        &read_text(repository_root, ledger::text(document, "milestone_spec")?)?,
        ledger::text(document, "milestone_marker")?,
    )
}

fn audit_commands(document: &Value) -> Result<(), String> {
    if ledger::text(document, "generated_context_command")?
        .trim()
        .is_empty()
    {
        return Err("Phase 8 generated-context command is empty".to_owned());
    }
    if ledger::strings(document, "verification_commands")?.len() != 9 {
        return Err("Phase 8 verification command set changed".to_owned());
    }
    Ok(())
}

fn read_text(repository_root: &Path, path: &str) -> Result<String, String> {
    fs::read_to_string(repository_root.join(path))
        .map_err(|error| format!("Phase 8 source `{path}` should be readable: {error}"))
}

fn named_rows<'a>(rows: &'a [Value], key: &str) -> Result<BTreeMap<&'a str, &'a Value>, String> {
    let mut named = BTreeMap::new();
    for row in rows {
        let name = ledger::text(row, key)?;
        if named.insert(name, row).is_some() {
            return Err(format!("duplicate Phase 8 `{key}` row `{name}`"));
        }
    }
    Ok(named)
}

fn require_exact_keys(
    rows: &BTreeMap<&str, &Value>,
    expected: &[&str],
    label: &str,
) -> Result<(), String> {
    let actual = rows.keys().copied().collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(format!("Phase 8 {label} keys differ: {actual:?}"));
    }
    Ok(())
}

fn compare_text(
    closing: &Value,
    authority: &Value,
    label: &str,
    field: &str,
) -> Result<(), String> {
    if ledger::text(closing, field)? != ledger::text(authority, field)? {
        return Err(format!("Phase 8 `{label}` field `{field}` drifted"));
    }
    Ok(())
}

pub(super) fn require_headings(path: &str, text: &str, headings: &[&str]) -> Result<(), String> {
    for heading in headings {
        if !text.lines().any(|line| line == *heading) {
            return Err(format!("Phase 8 document `{path}` lacks `{heading}`"));
        }
    }
    Ok(())
}

pub(super) fn reject_forbidden_fragments(
    path: &str,
    text: &str,
    allows_removed: bool,
    forbidden: &[&str],
) -> Result<(), String> {
    if allows_removed {
        return Ok(());
    }
    for fragment in forbidden {
        if text.contains(fragment) {
            return Err(format!(
                "Phase 8 public document `{path}` contains stale route `{fragment}`"
            ));
        }
    }
    Ok(())
}

pub(super) fn audit_proof_ledger_text(path: &str, text: &str) -> Result<(), String> {
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| format!("Phase 8 proof ledger `{path}` is empty"))?;
    if parse_csv_line(header).len() != 7 {
        return Err(format!("Phase 8 proof ledger `{path}` header is invalid"));
    }
    let mut count = 0;
    for line in lines {
        let fields = parse_csv_line(line);
        if fields.len() != 7 {
            return Err(format!("Phase 8 proof ledger `{path}` row is invalid"));
        }
        if fields[4].trim().is_empty() || fields[4] == "not yet evaluated" {
            return Err(format!("Phase 8 proof ledger `{path}` has empty evidence"));
        }
        if fields[6] != "PROVED" {
            return Err(format!(
                "Phase 8 proof ledger `{path}` has open status `{}`",
                fields[6]
            ));
        }
        count += 1;
    }
    if count == 0 {
        return Err(format!("Phase 8 proof ledger `{path}` has no claims"));
    }
    Ok(())
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = line.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                fields.push(std::mem::take(&mut field));
            }
            other => field.push(other),
        }
    }
    fields.push(field);
    fields
}

pub(super) fn require_marker(label: &str, text: &str, marker: &str) -> Result<(), String> {
    if !text.contains(marker) {
        return Err(format!(
            "Phase 8 {label} lacks completion marker `{marker}`"
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "phase8_closeout_tests.rs"]
mod tests;
