use std::collections::{BTreeMap, BTreeSet};

use crate::topology::WorkspaceSourceInventory;

use super::ledger;

#[path = "phase6_callable_surface_ast.rs"]
mod ast;

use ast::{collect_file_callables, reject_forbidden_symbols_in_source, source_calls_method};

const RUNTIME_ROOT: &str = "crates/worth-ui-runtime/src";
const PRODUCT_ROOT: &str = "crates/worth-ui/src";
const TEST_SUPPORT_ROOT: &str = "crates/worth-ui-test-support/src";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Callable {
    kind: &'static str,
    owner: String,
    method: String,
    source: String,
}

type CallableIdentity = (&'static str, String, String);

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    document: &toml::Value,
) -> Result<(), String> {
    validate_header(document)?;
    let rows = ledger::tables(document, "surface")?;
    let declared = declared_callables(inventory, rows)?;
    let owners = declared_owners(rows)?;
    require_core_owners(&owners)?;
    let actual = actual_callables(inventory, &owners)?;
    audit_definition_sources(&actual, rows)?;
    let actual_identities = callable_identities(&actual);
    let declared_identities = callable_identities(&declared);
    if actual_identities != declared_identities {
        return Err(format!(
            "Phase 6 callable surface differs from its exact manifest; {}",
            describe_difference(&actual_identities, &declared_identities)
        ));
    }
    audit_forbidden_symbols(inventory, ledger::strings(document, "forbidden_symbols")?)?;
    audit_one_ordinary_mounted_entry(&actual)
}

fn declared_owners(rows: &[toml::Value]) -> Result<BTreeSet<(&'static str, String)>, String> {
    rows.iter()
        .map(|row| {
            let kind = ledger::text(row, "kind")?;
            let kind = match kind {
                "inherent" => "inherent",
                "extension_trait" => "extension_trait",
                other => return Err(format!("unknown callable owner kind `{other}`")),
            };
            Ok((kind, ledger::text(row, "owner")?.to_owned()))
        })
        .collect()
}

fn validate_header(document: &toml::Value) -> Result<(), String> {
    if ledger::text(document, "schema")? != "worth-ui.milestone-3.10.1.callable-surface.v1" {
        return Err("Phase 6 callable surface schema should be v1".to_owned());
    }
    Ok(())
}

fn declared_callables(
    inventory: &WorkspaceSourceInventory,
    rows: &[toml::Value],
) -> Result<BTreeSet<Callable>, String> {
    let mut declared = BTreeSet::new();
    let mut owners = BTreeSet::new();
    for row in rows {
        let (kind, owner) = validate_declared_owner(inventory, row, &mut owners)?;
        collect_declared_owner_callers(inventory, row, (kind, &owner), &mut declared)?;
    }
    Ok(declared)
}

fn validate_declared_owner(
    inventory: &WorkspaceSourceInventory,
    row: &toml::Value,
    owners: &mut BTreeSet<(&'static str, String)>,
) -> Result<(&'static str, String), String> {
    let kind = match ledger::text(row, "kind")? {
        "inherent" => "inherent",
        "extension_trait" => "extension_trait",
        other => return Err(format!("unknown callable owner kind `{other}`")),
    };
    let owner = ledger::text(row, "owner")?.to_owned();
    if ledger::text(row, "audience")?.trim().is_empty()
        || ledger::text(row, "feature")?.trim().is_empty()
    {
        return Err(format!(
            "callable owner `{owner}` needs audience and feature posture"
        ));
    }
    if !owners.insert((kind, owner.clone())) {
        return Err(format!("duplicate callable owner `{kind}::{owner}`"));
    }
    validate_declared_sources(inventory, row, &owner)?;
    Ok((kind, owner))
}

fn validate_declared_sources(
    inventory: &WorkspaceSourceInventory,
    row: &toml::Value,
    owner: &str,
) -> Result<(), String> {
    let sources = ledger::strings(row, "sources")?;
    if sources.is_empty() {
        return Err(format!("callable owner `{owner}` needs definition sources"));
    }
    for source in sources {
        if !inventory.contains(source) {
            return Err(format!(
                "callable owner `{owner}` source `{source}` is absent"
            ));
        }
    }
    Ok(())
}

fn collect_declared_owner_callers(
    inventory: &WorkspaceSourceInventory,
    row: &toml::Value,
    identity: (&'static str, &str),
    declared: &mut BTreeSet<Callable>,
) -> Result<(), String> {
    let (kind, owner) = identity;
    let callers = row
        .get("callers")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("callable owner `{owner}` needs a callers table"))?;
    for (method, caller_value) in callers {
        let caller = caller_value
            .as_str()
            .ok_or_else(|| format!("caller for `{owner}::{method}` should be a path"))?;
        validate_real_caller(inventory, owner, method, caller)?;
        declared.insert(Callable {
            kind,
            owner: owner.to_owned(),
            method: method.to_owned(),
            source: String::new(),
        });
    }
    Ok(())
}

fn validate_real_caller(
    inventory: &WorkspaceSourceInventory,
    owner: &str,
    method: &str,
    caller: &str,
) -> Result<(), String> {
    if !inventory.contains(caller) {
        return Err(format!(
            "caller `{caller}` for `{owner}::{method}` is absent"
        ));
    }
    if !source_calls_method(inventory.text(caller), method)? {
        return Err(format!(
            "caller `{caller}` does not call retained callable `{owner}::{method}`"
        ));
    }
    Ok(())
}

fn audit_definition_sources(
    actual: &BTreeSet<Callable>,
    rows: &[toml::Value],
) -> Result<(), String> {
    let mut declared = BTreeMap::<(&str, &str), BTreeSet<&str>>::new();
    for row in rows {
        declared.insert(
            (ledger::text(row, "kind")?, ledger::text(row, "owner")?),
            ledger::strings(row, "sources")?.into_iter().collect(),
        );
    }
    let mut drift = Vec::new();
    for callable in actual {
        let sources = declared
            .get(&(callable.kind, callable.owner.as_str()))
            .ok_or_else(|| format!("callable owner `{}` is unmanifested", callable.owner))?;
        if !sources.contains(callable.source.as_str()) {
            drift.push(format!(
                "callable `{}::{}` is defined in undeclared source `{}`",
                callable.owner, callable.method, callable.source
            ));
        }
    }
    if drift.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "callable definition source inventory drifted: {}",
            drift.join("; ")
        ))
    }
}

fn callable_identities(callables: &BTreeSet<Callable>) -> BTreeSet<CallableIdentity> {
    callables
        .iter()
        .map(|callable| {
            (
                callable.kind,
                callable.owner.clone(),
                callable.method.clone(),
            )
        })
        .collect()
}

fn require_core_owners(owners: &BTreeSet<(&'static str, String)>) -> Result<(), String> {
    for owner in [
        "WorthUi",
        "WorthUiApplicationBuilder",
        "WorthUiApp",
        "WorthUiActiveApplicationSession",
    ] {
        if !owners.contains(&("inherent", owner.to_owned())) {
            return Err(format!(
                "Phase 6 callable manifest omits core owner `{owner}`"
            ));
        }
    }
    Ok(())
}

fn actual_callables(
    inventory: &WorkspaceSourceInventory,
    owners: &BTreeSet<(&'static str, String)>,
) -> Result<BTreeSet<Callable>, String> {
    let mut actual = BTreeSet::new();
    for root in [RUNTIME_ROOT, TEST_SUPPORT_ROOT] {
        for source in inventory.rust_files_under(root) {
            let path = source.relative_path().to_string_lossy().replace('\\', "/");
            let syntax = syn::parse_file(source.text())
                .map_err(|error| format!("{path} should parse: {error}"))?;
            collect_file_callables(&syntax, &path, owners, &mut actual)?;
        }
    }
    Ok(actual)
}

fn audit_forbidden_symbols(
    inventory: &WorkspaceSourceInventory,
    forbidden: Vec<&str>,
) -> Result<(), String> {
    let forbidden = forbidden.into_iter().collect::<BTreeSet<_>>();
    for root in [RUNTIME_ROOT, PRODUCT_ROOT, TEST_SUPPORT_ROOT] {
        for source in inventory.rust_files_under(root) {
            reject_forbidden_symbols_in_source(source.relative_path(), source.text(), &forbidden)?;
        }
    }
    Ok(())
}

fn audit_one_ordinary_mounted_entry(actual: &BTreeSet<Callable>) -> Result<(), String> {
    let entries = actual
        .iter()
        .filter(|callable| {
            callable.kind == "inherent"
                && callable.owner == "WorthUiActiveApplicationSession"
                && callable.method.starts_with("execute_")
        })
        .map(|callable| callable.method.as_str())
        .collect::<Vec<_>>();
    if entries != ["execute_mounted_frame"] {
        return Err(format!(
            "active session should have exactly one ordinary execute entry; actual={entries:?}"
        ));
    }
    Ok(())
}

fn describe_difference(
    actual: &BTreeSet<CallableIdentity>,
    declared: &BTreeSet<CallableIdentity>,
) -> String {
    let unmanifested = actual.difference(declared).collect::<Vec<_>>();
    let absent = declared.difference(actual).collect::<Vec<_>>();
    format!("unmanifested={unmanifested:?}, absent={absent:?}")
}

#[cfg(test)]
#[path = "phase6_callable_surface_tests.rs"]
mod tests;
