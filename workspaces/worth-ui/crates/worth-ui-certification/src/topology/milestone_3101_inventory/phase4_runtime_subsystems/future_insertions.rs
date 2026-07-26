use std::collections::{BTreeMap, BTreeSet};

use toml::Value;

use super::super::ledger;

type InsertionContract<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str);

const RUNTIME_FAMILY: &str = "runtime-family";
const EXTERNAL_AUTHORITY: &str = "external-authority";

pub(super) fn audit(document: &Value, families: &BTreeSet<String>) -> Result<(), String> {
    let expected = expected_insertions();
    let mut actual = BTreeMap::new();
    for row in ledger::tables(document, "future_insertion")? {
        let milestone = ledger::text(row, "milestone")?;
        let contract = insertion_contract(row)?;
        validate_owner_scope(milestone, contract.2, contract.3, families)?;
        if actual.insert(milestone, contract).is_some() {
            return Err(format!(
                "future milestone `{milestone}` has multiple owners"
            ));
        }
    }
    if actual != expected {
        return Err(format!(
            "future insertion contracts differ: actual={actual:?}, expected={expected:?}"
        ));
    }
    Ok(())
}

fn insertion_contract(row: &Value) -> Result<InsertionContract<'_>, String> {
    Ok((
        ledger::text(row, "roadmap_heading")?,
        ledger::text(row, "change")?,
        ledger::text(row, "owner_scope")?,
        ledger::text(row, "owner")?,
        ledger::text(row, "insertion")?,
        ledger::text(row, "forbidden_owner")?,
    ))
}

fn validate_owner_scope(
    milestone: &str,
    owner_scope: &str,
    owner: &str,
    families: &BTreeSet<String>,
) -> Result<(), String> {
    match owner_scope {
        RUNTIME_FAMILY if families.contains(owner) => Ok(()),
        EXTERNAL_AUTHORITY if owner == "worth-ui-dsl" => Ok(()),
        RUNTIME_FAMILY | EXTERNAL_AUTHORITY => Err(format!(
            "future milestone `{milestone}` has invalid `{owner_scope}` owner `{owner}`"
        )),
        unknown => Err(format!(
            "future milestone `{milestone}` has unknown owner scope `{unknown}`"
        )),
    }
}

fn expected_insertions() -> BTreeMap<&'static str, InsertionContract<'static>> {
    BTreeMap::from([
        (
            "3.11",
            (
                "### Milestone 3.11: Visual Snapshot Receipts and Hit-Test Identity Bridge",
                "visual snapshot truth and mounted-receipt identity bridge",
                RUNTIME_FAMILY,
                "application",
                "application replacement preparation and commit",
                "session",
            ),
        ),
        (
            "3.12",
            (
                "### Milestone 3.12: Observation Intake and Hot Rebind Planner",
                "semantic host observation admission before bounded hot-rebind planning",
                RUNTIME_FAMILY,
                "observation",
                "after structural host-report validation",
                "mounting",
            ),
        ),
        (
            "3.17",
            (
                "### Milestone 3.17: DSL Expressions, Conditions, and Semantic Evaluation",
                "runtime evaluation and invalidation of sealed DSL expression artifacts",
                RUNTIME_FAMILY,
                "planning",
                "planning input handoff before active-plan publication",
                "session",
            ),
        ),
        (
            "3.18",
            (
                "### Milestone 3.18: DSL Composition, Modules, and Lowering Equivalence",
                "module and composition lowering before the sealed semantic handoff",
                EXTERNAL_AUTHORITY,
                "worth-ui-dsl",
                "before the sealed semantic handoff; no runtime subsystem insertion",
                "session",
            ),
        ),
    ])
}
